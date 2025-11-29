use std::sync::{Arc, RwLock};

// use iced::PickList;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use walkanalysis::{
    exercise::analysis::{self, Analysis, Correction, Mistake, NoteAnalysis},
    form::{form::FormPiece, key},
};

use crate::{fonts, ExerciseKind, FormKind};

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

pub struct WalkanalysisEditor {
    state: Arc<RwLock<WalkanalysisSharedState>>,

    context: Arc<dyn GuiContext>,
    form_selector_state: pick_list::State<FormKind>,
    exercise_selector_state: pick_list::State<ExerciseKind>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FormSelected(FormKind),
    ExerciseSelected(ExerciseKind),
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
    fn view_note_analysis<'b>(analysis: &NoteAnalysis) -> Element<'b, Message> {
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
                    walkanalysis::form::chord::ChordTone::NoChordTone => "x",
                };

                let background_color = degree_in_key.map(|degree| match degree {
                    key::Degree::First => Color::from_rgb8(80, 170, 120),
                    key::Degree::Second => Color::from_rgb8(70, 160, 180),
                    key::Degree::Third => Color::from_rgb8(70, 130, 200),
                    key::Degree::Fourth => Color::from_rgb8(110, 100, 200),
                    key::Degree::Fifth => Color::from_rgb8(190, 90, 120),
                    key::Degree::Sixth => Color::from_rgb8(210, 150, 80),
                    key::Degree::Seventh => Color::from_rgb8(220, 200, 80),
                    key::Degree::Chromatic => Color::from_rgb8(150, 150, 150),
                });

                Container::new(Text::new(format!("{}", chord_tone_str)))
                    .style(MyContainerStyle {
                        background: background_color.map(|c| Background::Color(c)),
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center)
                    .into()
            }
            NoteAnalysis::NoteDuringSilence { note } => Row::new().into(), // TODO: this
        }
    }

    pub fn view<'b>(&self) -> Element<'b, Message> {
        let chord_symbol: Element<'b, Message> = match &self.form_piece {
            FormPiece::Key(_) => unreachable!(),
            FormPiece::LineBreak => unreachable!(),
            FormPiece::CountOff => unreachable!(),
            FormPiece::ChordBar(chord) => Text::new(ascii(format!("{}", chord.flat_symbol())))
                .font(fonts::EB_GARAMOND_MEDIUM)
                .size(24)
                .into(), // TODO: determine sharp / flat from form
            FormPiece::HalfBar(chord1, chord2) => Row::new()
                .push(
                    Text::new(ascii(format!("{}", chord1.flat_symbol())))
                        .font(fonts::EB_GARAMOND_MEDIUM)
                        .width(Length::Fill),
                )
                .push(
                    Text::new(ascii(format!("{}", chord2.flat_symbol())))
                        .font(fonts::EB_GARAMOND_MEDIUM)
                        .width(Length::Fill),
                )
                .into(),
        };

        let mut beats = Row::new();

        if let Some(analyzed_beats) = self.analyzed_beats.as_ref() {
            for (i, beat) in analyzed_beats.iter().enumerate() {
                let Some(beat) = beat else {
                    beats = beats
                        .push(Text::new("-").font(fonts::ROBOTO_MONO_REGULAR))
                        .width(Length::Fill);
                    continue;
                };

                beats = beats.push(Self::view_note_analysis(beat));
                println!("[{}] {:?}", i, beat);
            }
        } else {
            for i in 0..4 {
                beats = beats
                    .push(Text::new(format!("{}", i + 1)).width(Length::Fill).font(
                        if self.current_beat == Some(i) {
                            fonts::ROBOTO_MONO_MEDIUM
                        } else {
                            fonts::ROBOTO_MONO_REGULAR
                        },
                    ))
                    .align_items(Alignment::Center);
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
        let new_row = || Row::new().width(Length::Fill).padding(16);

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

                    let bar = WrittenBar {
                        form_piece: new_form_piece,
                        analyzed_beats: analyzed_beats.as_ref(),
                        correction_beats: &[const { None }; 4],
                        current_beat,
                    };
                    row = row.push(bar.view())
                }
                FormPiece::LineBreak => {
                    column = column.push(row);
                    row = new_row()
                }
            }
            form_beat_counter += form_piece.length_in_beats();
        }
        column = column.push(row);

        // each chord symbol should be accompanied in a column with 4 thingos that:
        // once available, show the note analysis (role in chord if any, degree as a background color, chromatic is gray-ish inbetween color)
        // otherwise should still take the same amount of space
        // if the current beat is at that beat then it must be shown somehow

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
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let current_state = self.state.read().unwrap();

        let form_and_correction = self.view_form_and_correction();

        let form_picker = PickList::new(
            &mut self.form_selector_state,
            &FormKind::ALL[..],
            Some(current_state.selected_form),
            Message::FormSelected,
        );

        let exercise_picker = PickList::new(
            &mut self.exercise_selector_state,
            &ExerciseKind::ALL[..],
            Some(current_state.selected_exercise),
            Message::ExerciseSelected,
        );

        let count_off_text = Text::new(format!(
            "{}",
            current_state.beat_pos.map(|b| countoff(b)).unwrap_or("")
        ));

        let picker_row = Row::new()
            .padding(4)
            .spacing(8)
            .height(Length::Units(64))
            .push(form_picker)
            .push(exercise_picker)
            .push(
                Container::new(count_off_text)
                    .style(MyContainerStyle {
                        background: Some(Background::Color(Color::from_rgb8(230, 230, 230))),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::BLACK,
                        ..Default::default()
                    })
                    .padding(3)
                    .width(Length::Units(32))
                    .height(Length::Units(32)),
            );

        Column::new()
            .push(
                Text::new(format!("{}", current_state.selected_form))
                    .font(fonts::EB_GARAMOND_MEDIUM)
                    .size(40)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .width(Length::Fill),
            )
            .push(form_and_correction)
            .push(Space::new(Length::Units(0), Length::Fill))
            .push(picker_row)
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

pub struct MyContainerStyle {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for MyContainerStyle {
    fn default() -> Self {
        Self {
            text_color: Default::default(),
            background: Default::default(),
            border_radius: Default::default(),
            border_width: Default::default(),
            border_color: Default::default(),
        }
    }
}

impl container::StyleSheet for MyContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}
