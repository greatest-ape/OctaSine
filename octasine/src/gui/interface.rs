use std::sync::Arc;

use iced_baseview::{executor, Application, Command};
use iced_baseview::{
    Column, Element, Row, Text,
};
use iced_audio::{
    knob, Normal, NormalParam, FloatRange
};

use crate::SyncOnlyState;


#[derive(Debug, Clone)]
pub enum Message {
    MasterVolume(Normal),
}


trait Parameter {
    fn new(sync_only: &Arc<SyncOnlyState>) -> Self;
    fn view(&mut self) -> Element<Message>;
    fn update(&mut self, sync_only: &Arc<SyncOnlyState>, value: Normal);
}


#[derive(Debug, Clone)]
struct MasterVolume {
    knob_state: knob::State,
}


impl Parameter for MasterVolume {
    fn new(sync_only: &Arc<SyncOnlyState>) -> Self {
        let value = sync_only.presets.get_parameter_value_float(0);

        let value = Normal::new(value as f32);

        let normal_param = NormalParam {
            value,
            default: value, // FIXME
        };
        
        Self {
            knob_state: knob::State::new(normal_param),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let value = Text::new(
            format!("{:.4}", self.knob_state.normal_param.value.as_f32())
        ).size(12);

        let knob = knob::Knob::new(
            &mut self.knob_state,
            Message::MasterVolume
        );

        Column::new()
            .push(Text::new("Master volume").size(12))
            .push(knob)
            .push(value)
            .into()
    }

    fn update(&mut self, sync_only: &Arc<SyncOnlyState>, value: Normal){
        sync_only.presets.set_parameter_value_float(0, value.as_f32() as f64)
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
        let master_volume = MasterVolume::new(&sync_only);

        let app = Self {
            sync_only,
            master_volume,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        crate::PLUGIN_NAME.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MasterVolume(value) => {
                self.master_volume.update(&self.sync_only, value)
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view();
        
        Column::new()
            .push(
                Row::new()
                    .padding(20)
                    .push(master_volume)
            )
            .into()
    }
}