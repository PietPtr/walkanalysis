use std::sync::{Arc, RwLock};

// use iced::PickList;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use walkanalysis::{
    exercise::analysis::{Analysis, Correction},
    form::form::FormPiece,
};

use crate::{ExerciseKind, FormKind};

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

type WalkanalysisInitializationType = Arc<RwLock<WalkanalysisSharedState>>;

impl WalkanalysisEditor {
    fn view_form_and_correction<'a>(&self) -> Element<'a, Message> {
        let current_state = self.state.read().unwrap();
        let form = current_state.selected_form.form();
        let mut column = Column::new();

        let new_row = || Row::new().width(Length::Fill).padding(16);

        let mut row = new_row();
        for form_piece in form.music {
            match form_piece {
                FormPiece::Key(_) => (), // TODO: display key
                FormPiece::CountInBar => {
                    row = row.push(Text::new("1 2 3 4").width(Length::Fill));
                }
                FormPiece::ChordBar(chord) => {
                    let text =
                        Text::new(ascii(format!("{}", chord.flat_symbol()))).width(Length::Fill);
                    row = row.push(text);
                } // TODO: determine sharp / flat from form
                FormPiece::HalfBar(chord1, chord2) => {
                    let symbols = Row::new()
                        .push(
                            Text::new(ascii(format!("{}", chord1.flat_symbol())))
                                .width(Length::Fill),
                        )
                        .push(
                            Text::new(ascii(format!("{}", chord2.flat_symbol())))
                                .width(Length::Fill),
                        )
                        .width(Length::Fill);

                    row = row.push(symbols);
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

        let picker_row = Row::new()
            .padding(4)
            .spacing(8)
            .push(form_picker)
            .push(exercise_picker);

        Column::new()
            .align_items(Alignment::Center)
            .push(picker_row)
            .push(
                Text::new(format!("{}", current_state.selected_form))
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(30)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
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
