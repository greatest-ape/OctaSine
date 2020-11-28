use std::sync::Arc;

use iced_baseview::{executor, Application, Command};
use iced_baseview::{
    Column, Element, Row, Text,
};
use iced_audio::{
    knob, Normal, FloatRange
};

use crate::SyncOnlyState;


#[derive(Debug, Clone)]
pub enum Message {
    Knob(Normal),
}


pub(super) struct OctaSineIcedApplication {
    sync_only: Arc<SyncOnlyState>,
    knob_state: knob::State,
    knob_value: Normal,
}


impl Application for OctaSineIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<SyncOnlyState>;

    fn new(sync_only: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            sync_only,
            knob_state: knob::State::new(FloatRange::default().default_normal_param()),
            knob_value: Normal::default(),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        crate::PLUGIN_NAME.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Knob(value) => {
                self.knob_value = value;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let knob: Element<_> = {
            let knob = knob::Knob::new(
                &mut self.knob_state,
                Message::Knob
            );

            Column::new()
                .push(Text::new("Volume").size(10))
                .push(knob)
                .push(Text::new(format!("{:.4}", self.knob_value.as_f32())))
                .into()
        };
        
        Column::new()
            .push(
                Row::new()
                    .padding(20)
                    .push(knob)
            )
            .into()
    }
}