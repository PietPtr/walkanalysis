use std::sync::Arc;

use iced::widget::combo_box;
use nih_plug::{prelude::{Editor, GuiContext}, wrapper::vst3::vst3_sys::gui};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;

use crate::{ExerciseKind, FormKind, WalkAnalysisParams};

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(500, 706)
}

pub(crate) fn create(
    params: Arc<WalkAnalysisParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<WalkanalysisEditor>(editor_state, params)
}

pub struct WalkanalysisEditor {
    params: Arc<WalkAnalysisParams>,
    context: Arc<dyn GuiContext>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ParamUpdate(nih_widgets::ParamMessage),
    Test,
}

impl IcedEditor for WalkanalysisEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<WalkAnalysisParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = WalkanalysisEditor { params, context };

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
        match message {
            Message::ParamUpdate(param_message) => self.handle_param_message(param_message),
            Message::Test => println!("got test message"),
            TODO: get the correction to show as a string in the gui
            TODO: selectors for the form and exercise
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("WalkAnalysis")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
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
