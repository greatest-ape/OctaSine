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
    MasterVolume(Normal),
}


trait Parameter {
    fn as_string(&self) -> String;
}


#[derive(Debug, Clone)]
struct MasterVolume {
    knob_state: knob::State,
    knob_value: Normal,
}


impl Default for MasterVolume {
    fn default() -> Self {
        Self {
            knob_state: knob::State::new(
                FloatRange::default().default_normal_param()
            ),
            knob_value: Normal::default(),
        }
    }
}


impl Parameter for MasterVolume {
    fn as_string(&self) -> String {
        format!("{:.4}", self.knob_value.as_f32())
    }
}


pub(super) struct OctaSineIcedApplication {
    sync_only: Arc<SyncOnlyState>,
    master_volume: MasterVolume,
}


impl Application for OctaSineIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<SyncOnlyState>;

    fn new(sync_only: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            sync_only,
            master_volume: Default::default(),
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        crate::PLUGIN_NAME.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MasterVolume(value) => {
                self.master_volume.knob_value = value;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let knob: Element<_> = {
            let value = Text::new(self.master_volume.as_string()).size(12);

            let knob = knob::Knob::new(
                &mut self.master_volume.knob_state,
                Message::MasterVolume
            );

            Column::new()
                .push(Text::new("Master volume").size(12))
                .push(knob)
                .push(value)
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