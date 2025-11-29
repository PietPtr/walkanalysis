use std::sync::{Arc, RwLock};

// use iced::PickList;
use nih_plug::{
    params::Param,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use walkanalysis::exercise::analysis::Correction;

use crate::{ExerciseKind, FormKind, WalkAnalysisParams};

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(500, 706)
}

pub(crate) fn create(
    init: WalkanalysisInitializationType,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<WalkanalysisEditor>(editor_state, init)
}

/// All the state shared between audio and UI thread
pub struct WalkanalysisSharedState {
    pub selected_form: FormKind,
    pub selected_exercise: ExerciseKind,
    pub correction: Option<Correction>,
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
    ParamUpdate(nih_widgets::ParamMessage),
    FormSelected(FormKind),
    ExerciseSelected(ExerciseKind),
}

type WalkanalysisInitializationType = Arc<RwLock<WalkanalysisSharedState>>;

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
        window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        let mut state = self.state.write().unwrap();
        match message {
            Message::ParamUpdate(param_message) => self.handle_param_message(param_message),
            Message::FormSelected(form_kind) => state.selected_form = form_kind,
            Message::ExerciseSelected(exercise) => state.selected_exercise = exercise,
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let current_state = self.state.read().unwrap();

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
            .push(
                Text::new(format!("{:?}", current_state.correction))
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(12),
            )
            .push(Text::new(format!("{:?}", current_state.beat_pos)))
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
