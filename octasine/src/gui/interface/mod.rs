use std::sync::Arc;

use iced_baseview::{executor, Application, Command};
use iced_baseview::{
    Column, Element, Row,
};
use iced_audio::Normal;

use crate::SyncOnlyState;

mod widgets;

use widgets::OctaSineKnob;


#[derive(Debug, Clone)]
pub enum Message {
    ParameterChange(usize, Normal),
}


trait ParameterWidget {
    fn view(&mut self) -> Element<Message>;
}


pub(super) struct OctaSineIcedApplication {
    sync_only: Arc<SyncOnlyState>,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
}


impl Application for OctaSineIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<SyncOnlyState>;

    fn new(sync_only: Self::Flags) -> (Self, Command<Self::Message>) {
        let master_volume = OctaSineKnob::new(
            &sync_only,
            "Master\nvolume".to_string(),
            0
        );
        let master_frequency = OctaSineKnob::new(
            &sync_only,
            "Master\nfrequency".to_string(),
            1
        );

        let app = Self {
            sync_only,
            master_volume,
            master_frequency
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        crate::PLUGIN_NAME.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ParameterChange(index, value) => {
                self.sync_only.presets.set_parameter_value_float(
                    index,
                    value.as_f32() as f64
                )
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view();
        let master_frequency = self.master_frequency.view();
        
        Column::new()
            .push(
                Row::new()
                    .padding(16)
                    .push(master_volume)
                    .push(master_frequency)
            )
            .into()
    }
}
