use std::sync::Arc;

use iced_baseview::{executor, Application, Command, Align};
use iced_baseview::{
    Column, Element, Row,
};
use iced_audio::Normal;

use crate::SyncHandle;

mod widgets;

use widgets::OctaSineKnob;


#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    ParameterChange(usize, Normal),
}


trait ParameterWidget<H: SyncHandle> {
    fn view(&mut self, sync_state: &Arc<H>) -> Element<Message>;
    fn set_value(&mut self, value: f64);
}


pub struct OctaSineIcedApplication<H: SyncHandle> {
    sync_handle: Arc<H>,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
}


impl <H: SyncHandle> OctaSineIcedApplication<H> {
    fn update_widgets_from_parameters(&mut self){
        let opt_changes = self.sync_handle.get_presets()
            .get_changed_parameters_from_gui();
        
        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    match index {
                        0 => ParameterWidget::<H>::set_value(&mut self.master_volume, *new_value),
                        1 => ParameterWidget::<H>::set_value(&mut self.master_frequency, *new_value),
                        _ => (),
                    }
                }
            }
        }
    }
}


impl <H: SyncHandle>Application for OctaSineIcedApplication<H> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<H>;

    fn new(sync_handle: Self::Flags) -> (Self, Command<Self::Message>) {
        let master_volume = OctaSineKnob::master_volume(&sync_handle);
        let master_frequency = OctaSineKnob::master_frequency(&sync_handle);

        let app = Self {
            sync_handle,
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
                self.sync_handle.get_presets().set_parameter_value_float_from_gui(
                    index,
                    value.as_f32() as f64
                );

                self.sync_handle.update_host_display();
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view(&self.sync_handle);
        let master_frequency = self.master_frequency.view(&self.sync_handle);
        
        Row::new()
            .padding(16)
            .align_items(Align::Center)
            .spacing(16)
            .push(master_volume)
            .push(master_frequency)
            .into()
    }
}