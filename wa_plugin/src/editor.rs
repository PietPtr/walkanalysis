use std::sync::{Arc, RwLock};

// use iced::PickList;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use walkanalysis::{
    exercise::analysis::{Analysis, Correction, Mistake, NoteAnalysis},
    form::form::FormPiece,
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

pub struct WrittenBar {
    form_piece: FormPiece,
    analyzed_beats: [Option<NoteAnalysis>; 4],
    correction_beats: [Option<Mistake>; 4],
    current_beat: Option<u16>,
}

impl WrittenBar {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let chord_symbol: Element<'a, Message> = match &self.form_piece {
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

        for i in 0..4 {
            beats = beats
                .push(
                    Text::new(format!("{}", i + 1))
                        .width(Length::Fill)
                        .font(fonts::ROBOTO_MONO_REGULAR),
                )
                .align_items(Alignment::Center);
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
        for form_piece in form.music {
            let new_form_piece = form_piece.clone();
            match form_piece {
                FormPiece::Key(_) => (), // TODO: display key
                FormPiece::CountOff => {
                    // TODO: show live count-down near title or something
                }
                FormPiece::ChordBar(_) | FormPiece::HalfBar(_, _) => {
                    let bar = WrittenBar {
                        form_piece: new_form_piece,
                        analyzed_beats: [None; 4],
                        correction_beats: [const { None }; 4],
                        current_beat: None,
                    };
                    row = row.push(bar.view())
                }
                FormPiece::LineBreak => {
                    column = column.push(row);
                    row = new_row()
                }
            }
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
            .push(form_picker)
            .push(exercise_picker)
            .push(count_off_text);

        let test = Container::new(picker_row)
            .align_x(alignment::Horizontal::Right)
            .style(container::Style {
                ..container::Style::default()
            });

        Column::new()
            .align_items(Alignment::Center)
            .push(test)
            .push(
                Text::new(format!("{}", current_state.selected_form))
                    .font(fonts::EB_GARAMOND_MEDIUM)
                    .size(40),
                // .width(Length::Fill)
                // .horizontal_alignment(alignment::Horizontal::Center)
                // .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(form_and_correction)
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
