// TODO: fix restart behaviors:
// - when the song ends and the analysis is shown, allow selecting a new form / exercise
// - when analysis + correction is available, add a toggle to switch between analysis or correction (with icons? [magnifier glass] <toggle> [pen and paper])
// TODO: show correction: for every beat, show the note that was played, and highlight green or red for correct or wrong

use std::sync::{Arc, RwLock};

// use iced::PickList;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::{alignment::Horizontal, *};
use walkanalysis::{
    exercise::analysis::{Analysis, Correction, Mistake, MistakeKind, NoteAnalysis},
    form::{form::FormPiece, key, note::Note},
};

use crate::{
    colors, fonts,
    styles::{MyContainerStyle, MyPicklistStyle, MyTogglerStyle},
    ExerciseKind, FormKind,
};

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(500, 706)
}

pub(crate) fn create(
    init: WalkanalysisInitializationType,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<WalkanalysisEditor>(editor_state, init)
}

fn ascii(str: String) -> String {
    str.replace("♭", "b").replace("♯", "#")
}

/// All the state shared between audio and UI thread
pub struct WalkanalysisSharedState {
    pub selected_form: FormKind,
    pub selected_exercise: ExerciseKind,
    pub correction: Option<Correction>,
    pub analysis: Option<Analysis>,
    pub beat_pos: Option<f64>,
}

impl WalkanalysisSharedState {
    pub fn is_recording(&self) -> bool {
        if self.beat_pos.is_some() {
            self.analysis.is_none()
        } else {
            false
        }
    }
}

pub struct WalkanalysisEditor {
    state: Arc<RwLock<WalkanalysisSharedState>>,
    show_correction_instead_of_analysis: bool,
    show_chord_tone_instead_of_degree_in_analysis: bool,
    show_expected_instead_of_found_in_correction: bool,

    context: Arc<dyn GuiContext>,
    form_selector_state: pick_list::State<FormKind>,
    exercise_selector_state: pick_list::State<ExerciseKind>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FormSelected(FormKind),
    ExerciseSelected(ExerciseKind),
    AnalysisOrCorrection(bool),
    ExpectedOrFound(bool),
    ChordToneOrDegree(bool),
}

pub struct WrittenBar<'a> {
    form_piece: FormPiece,
    analyzed_beats: Option<&'a [Option<NoteAnalysis>; 4]>,
    correction_beats: &'a [Option<Mistake>; 4],
    /// None if this bar is not being recorded right now,
    /// Some(beat) if the current recording is at beat beat of this bar.
    current_beat: Option<u32>,
}

impl<'a> WrittenBar<'a> {
    fn view_mistake<'b>(
        note: Option<Note>,
        mistake: Option<Mistake>,
        show_expected_instead_of_found_in_correction: bool,
    ) -> Element<'b, Message> {
        let Some(mistake) = mistake else {
            return Container::new(
                Text::new(
                    note.map(|n| ascii(format!("{}", n.flat())))
                        .unwrap_or("?".into()),
                )
                .color(Color::WHITE),
            )
            .style(MyContainerStyle {
                background: Some(Background::Color(colors::GREEN)),
                border_radius: 4.,
                ..Default::default()
            })
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into();
        };

        let note_to_display = match mistake.mistake {
            MistakeKind::WrongNote { played, expected } => {
                if show_expected_instead_of_found_in_correction {
                    Some(expected)
                } else {
                    Some(played)
                }
            }
            _ => note,
        };

        let note_text = note_to_display
            .map(|n| ascii(format!("{}", n.flat())))
            .unwrap_or("?".into());

        // TODO: display what the mistake was exactly somehow
        Container::new(Text::new(note_text).color(Color::WHITE))
            .style(MyContainerStyle {
                background: Some(Background::Color(colors::RED)),
                border_radius: 4.,
                ..Default::default()
            })
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_note_analysis<'b>(
        analysis: &NoteAnalysis,
        show_chord_tone_instead_of_note_in_analysis: bool,
    ) -> Element<'b, Message> {
        match analysis {
            NoteAnalysis::Silence => Row::new().into(),
            NoteAnalysis::Note {
                note,
                degree_in_key,
                role_in_chord,
            } => {
                // chord tone is shown as a number
                // degree scale as a color:
                // 1     2    3    4      5   6      7
                // green cyan blue purple red orange yellow
                // chromatic grayed out version of inbetween color
                let chord_tone_str = match role_in_chord {
                    walkanalysis::form::chord::ChordTone::Root => "1",
                    walkanalysis::form::chord::ChordTone::Third => "3",
                    walkanalysis::form::chord::ChordTone::Fifth => "5",
                    walkanalysis::form::chord::ChordTone::Seventh => "7",
                    walkanalysis::form::chord::ChordTone::NoChordTone => "-",
                };

                let background_color = degree_in_key.map(|degree| match degree {
                    key::Degree::First => colors::GREEN,
                    key::Degree::Second => colors::YELLOW,
                    key::Degree::Third => colors::ORANGE,
                    key::Degree::Fourth => colors::RED,
                    key::Degree::Fifth => colors::PURPLE,
                    key::Degree::Sixth => colors::BLUE,
                    key::Degree::Seventh => colors::LIGHT_BLUE,
                    key::Degree::Chromatic => colors::GREY,
                });

                let beat_text = if show_chord_tone_instead_of_note_in_analysis {
                    format!("{}", chord_tone_str)
                } else {
                    ascii(format!("{}", note.flat()))
                };

                Container::new(Text::new(beat_text).color(Color::WHITE))
                    .style(MyContainerStyle {
                        background: background_color.map(|c| Background::Color(c)),
                        border_radius: 4.,
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
            NoteAnalysis::NoteDuringSilence { note: _ } => Row::new().into(), // TODO: this
        }
    }

    pub fn view<'b>(
        &self,
        show_correction_instead_of_analysis: bool,
        show_chord_tone_instead_of_degree_in_analysis: bool,
        show_expected_instead_of_found_in_correction: bool,
    ) -> Element<'b, Message> {
        const CHORD_SYMBOL_SIZE: u16 = 24;
        let chord_symbol: Element<'b, Message> = match &self.form_piece {
            FormPiece::Key(_) => unreachable!(),
            FormPiece::LineBreak => unreachable!(),
            FormPiece::CountOff => unreachable!(),
            FormPiece::ChordBar(chord) => Text::new(ascii(format!("{}", chord.flat_symbol())))
                .font(fonts::EB_GARAMOND_MEDIUM)
                .size(CHORD_SYMBOL_SIZE)
                .into(), // TODO: determine sharp / flat from form
            FormPiece::HalfBar(chord1, chord2) => Row::new()
                .push(
                    Text::new(ascii(format!("{}", chord1.flat_symbol())))
                        .font(fonts::EB_GARAMOND_MEDIUM)
                        .size(CHORD_SYMBOL_SIZE)
                        .width(Length::Fill),
                )
                .push(
                    Text::new(ascii(format!("{}", chord2.flat_symbol())))
                        .font(fonts::EB_GARAMOND_MEDIUM)
                        .size(CHORD_SYMBOL_SIZE)
                        .width(Length::Fill),
                )
                .into(),
        };

        let mut beats = Row::new().spacing(1).padding(Padding {
            top: 1,
            right: 0,
            bottom: 1,
            left: 0,
        });

        if let Some(analyzed_beats) = self.analyzed_beats.as_ref() {
            for (beat, mistake) in analyzed_beats.iter().zip(self.correction_beats.iter()) {
                let Some(beat) = beat else {
                    beats = beats.push(
                        Container::new(Text::new("?").font(fonts::ROBOTO_MONO_REGULAR))
                            .width(Length::Fill)
                            .center_x()
                            .center_y(),
                    );

                    continue;
                };

                if show_correction_instead_of_analysis {
                    beats = beats.push(Self::view_mistake(
                        beat.note(),
                        *mistake,
                        show_expected_instead_of_found_in_correction,
                    ));
                } else {
                    beats = beats.push(Self::view_note_analysis(
                        beat,
                        show_chord_tone_instead_of_degree_in_analysis,
                    ));
                }
            }
        } else {
            for i in 0..4 {
                beats = beats.push(
                    Container::new(Text::new(format!("{}", i + 1)).color(
                        if self.current_beat == Some(i) {
                            Color::WHITE
                        } else {
                            Color::BLACK
                        },
                    ))
                    .style(MyContainerStyle {
                        background: if self.current_beat == Some(i) {
                            Some(Background::Color(colors::RED))
                        } else {
                            None
                        },
                        border_radius: 4.,
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .center_x()
                    .center_y(),
                )
            }
        }

        Column::new()
            .push(chord_symbol)
            .push(beats)
            .width(Length::Fill)
            .into()
    }
}

type WalkanalysisInitializationType = Arc<RwLock<WalkanalysisSharedState>>;

impl WalkanalysisEditor {
    fn view_form_and_correction<'a>(&self) -> Element<'a, Message> {
        let current_state = self.state.read().unwrap();
        let form = current_state.selected_form.form();
        let mut column = Column::new();
        let new_row = || {
            Row::new()
                .width(Length::Fill)
                .padding(Padding {
                    top: 0,
                    right: 16,
                    bottom: 8,
                    left: 16,
                })
                .spacing(1)
        };

        let mut row = new_row();
        let mut form_beat_counter: u32 = 0;
        for form_piece in form.music {
            let new_form_piece = form_piece.clone();
            match form_piece {
                FormPiece::Key(_) => (), // TODO: display key
                FormPiece::CountOff => {
                    // TODO: show live count-down near title or something
                }
                FormPiece::ChordBar(_) | FormPiece::HalfBar(_, _) => {
                    let current_beat: Option<_> = current_state.beat_pos.and_then(|beat_pos| {
                        let beat_pos = beat_pos.floor() as u32;
                        if beat_pos >= form_beat_counter && beat_pos < form_beat_counter + 4 {
                            Some(beat_pos % 4)
                        } else {
                            None
                        }
                    });

                    let beats: [u32; 4] = [
                        form_beat_counter,
                        form_beat_counter + 1,
                        form_beat_counter + 2,
                        form_beat_counter + 3,
                    ];

                    let analyzed_beats = current_state.analysis.as_ref().map(|analysis| {
                        beats.map(|beat| analysis.beat_analysis.get(&beat).cloned().map(|n| n.1))
                    });

                    let correction_beats = current_state.correction.as_ref().map(|correction| {
                        beats.map(|beat| correction.mistakes.get(&beat).copied())
                    });

                    let bar = WrittenBar {
                        form_piece: new_form_piece,
                        analyzed_beats: analyzed_beats.as_ref(),
                        correction_beats: correction_beats.as_ref().unwrap_or(&[const { None }; 4]),
                        current_beat,
                    };
                    row = row.push(bar.view(
                        self.show_correction_instead_of_analysis,
                        self.show_chord_tone_instead_of_degree_in_analysis,
                        self.show_expected_instead_of_found_in_correction,
                    ))
                }
                FormPiece::LineBreak => {
                    column = column.push(row);
                    row = new_row()
                }
            }
            form_beat_counter += form_piece.length_in_beats();
        }
        column = column.push(row);

        column.into()
    }
}

impl IcedEditor for WalkanalysisEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = WalkanalysisInitializationType;

    fn new(
        state: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = WalkanalysisEditor {
            state,

            context,
            form_selector_state: Default::default(),
            exercise_selector_state: Default::default(),
            show_correction_instead_of_analysis: false,
            show_chord_tone_instead_of_degree_in_analysis: false,
            show_expected_instead_of_found_in_correction: false,
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        let mut state = self.state.write().unwrap();
        match message {
            Message::FormSelected(form_kind) => state.selected_form = form_kind,
            Message::ExerciseSelected(exercise) => state.selected_exercise = exercise,
            Message::AnalysisOrCorrection(choice) => {
                self.show_correction_instead_of_analysis = choice;
            }
            Message::ExpectedOrFound(choice) => {
                self.show_expected_instead_of_found_in_correction = choice
            }
            Message::ChordToneOrDegree(choice) => {
                self.show_chord_tone_instead_of_degree_in_analysis = choice
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let current_state = self.state.read().unwrap();

        // Title
        let title = Text::new(format!("{}", current_state.selected_form))
            .font(fonts::EB_GARAMOND_MEDIUM)
            .size(40)
            .height(Length::Units(45))
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill);

        // Most complex component: the form and all the beats
        let form_and_correction = self.view_form_and_correction();

        // Dropdown for selecting the form
        let form_picker = PickList::new(
            &mut self.form_selector_state,
            &FormKind::ALL[..],
            Some(current_state.selected_form),
            Message::FormSelected,
        )
        .font(fonts::EB_GARAMOND_MEDIUM)
        .style(MyPicklistStyle {});

        // Dropdown to select the exercise
        let exercise_picker = PickList::new(
            &mut self.exercise_selector_state,
            &ExerciseKind::ALL[..],
            Some(current_state.selected_exercise),
            Message::ExerciseSelected,
        )
        .font(fonts::EB_GARAMOND_MEDIUM)
        .style(MyPicklistStyle {});

        // Row with dropdowns
        let mut picker_row = Row::new()
            .padding(4)
            .spacing(8)
            .height(Length::Units(48))
            .align_items(Alignment::Center);

        // Count off and recording symbol at the bottom
        let count_off_text = format!(
            "{}",
            current_state.beat_pos.map(|b| countoff(b)).unwrap_or("")
        );
        let count_off = Container::new(Text::new(count_off_text).color(Color::WHITE))
            .style(MyContainerStyle {
                background: Some(Background::Color(if current_state.is_recording() {
                    colors::BRIGHT_RED
                } else {
                    colors::GREY
                })),
                border_radius: 50.0,
                ..Default::default()
            })
            .padding(3)
            .width(Length::Units(26))
            .height(Length::Units(26))
            .center_x()
            .center_y();

        // Analysis or correction toggle
        let analysis_or_correction_toggle = Row::new()
            .push(
                Text::new("ANALYSIS")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Right),
            )
            .push(
                Toggler::new(
                    self.show_correction_instead_of_analysis,
                    None,
                    Message::AnalysisOrCorrection,
                )
                .style(MyTogglerStyle {})
                .width(Length::Shrink),
            )
            .push(
                Text::new("CORRECTION")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(4);

        let expected_or_found_toggle = Row::new()
            .push(
                Text::new("FOUND")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Right),
            )
            .push(
                Toggler::new(
                    self.show_expected_instead_of_found_in_correction,
                    None,
                    Message::ExpectedOrFound,
                )
                .style(MyTogglerStyle {})
                .width(Length::Shrink),
            )
            .push(
                Text::new("EXPECTED")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(4);

        let chord_tone_or_degree_toggle = Row::new()
            .push(
                Text::new("NOTE")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Right),
            )
            .push(
                Toggler::new(
                    self.show_chord_tone_instead_of_degree_in_analysis,
                    None,
                    Message::ChordToneOrDegree,
                )
                .style(MyTogglerStyle {})
                .width(Length::Shrink),
            )
            .push(
                Text::new("CHORD TONE")
                    .size(18)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            )
            .spacing(4);

        // Integrate the dropdown, toggle switch, and count off in a single menu
        if !current_state.is_recording() {
            picker_row = picker_row.push(form_picker).push(exercise_picker);
        } else {
            picker_row = picker_row.push(count_off);
        }

        let mut menu_column = Column::new().align_items(Alignment::Center);

        if current_state.analysis.is_some() && current_state.correction.is_some() {
            menu_column = menu_column.push(analysis_or_correction_toggle);

            if self.show_correction_instead_of_analysis {
                menu_column = menu_column.push(expected_or_found_toggle)
            } else {
                menu_column = menu_column.push(chord_tone_or_degree_toggle)
            }
        }

        menu_column = menu_column.push(picker_row);

        Column::new()
            .push(title)
            .push(form_and_correction)
            .push(Space::new(Length::Units(0), Length::Fill))
            .push(menu_column)
            .align_items(Alignment::Center)
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}

fn countoff(beat_pos: f64) -> &'static str {
    let beat_pos = beat_pos.floor() as usize;
    match beat_pos {
        0 => "1",
        1 => "1",
        2 => "2",
        3 => "2",
        4 => "1",
        5 => "2",
        6 => "3",
        7 => "4",
        _ => "",
    }
}
