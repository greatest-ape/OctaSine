use std::sync::Arc;

use iced_baseview::{executor, Application, Command, Align};
use iced_baseview::{
    Column, Element, Row,
};
use iced_audio::Normal;

use vst::host::Host;

use crate::SyncOnlyState;

mod widgets;

use widgets::OctaSineKnob;


#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    ParameterChange(usize, Normal),
}


trait ParameterWidget {
    fn view(&mut self, sync_state: &Arc<SyncOnlyState>) -> Element<Message>;
    fn set_value(&mut self, value: f64);
}


pub(super) struct OctaSineIcedApplication {
    sync_only: Arc<SyncOnlyState>,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
}


impl OctaSineIcedApplication {
    fn update_widgets_from_parameters(&mut self){
        let opt_changes = self.sync_only.presets
            .get_changed_parameters_from_gui();
        
        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    match index {
                        0 => self.master_volume.set_value(*new_value),
                        1 => self.master_frequency.set_value(*new_value),
                        _ => (),
                    }
                }
            }
        }
    }
}


impl Application for OctaSineIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<SyncOnlyState>;

    fn new(sync_only: Self::Flags) -> (Self, Command<Self::Message>) {
        let master_volume = OctaSineKnob::master_volume(&sync_only);
        let master_frequency = OctaSineKnob::master_frequency(&sync_only);

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
            Message::Frame => {
                self.update_widgets_from_parameters();
            },
            Message::ParameterChange(index, value) => {
                self.sync_only.presets.set_parameter_value_float_from_gui(
                    index,
                    value.as_f32() as f64
                );

                // FIXME: maybe shouldn't be called in GUI code like this
                self.sync_only.host.update_display();
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view(&self.sync_only);
        let master_frequency = self.master_frequency.view(&self.sync_only);
        
        Row::new()
            .padding(16)
            .align_items(Align::Center)
            .spacing(16)
            .push(master_volume)
            .push(master_frequency)
            .into()
    }
}