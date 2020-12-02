use std::sync::Arc;

use iced_baseview::{executor, Application, Command, Align};
use iced_baseview::{
    Column, Element, Row, Container, Rule, Text, Length, Space
};
use iced_audio::Normal;

use crate::SyncHandle;

mod widgets;
mod operator;

use operator::OperatorWidgets;
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
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
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

        let operator_1 = OperatorWidgets::new(&sync_handle, 0);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3);

        let app = Self {
            sync_handle,
            master_volume,
            master_frequency,
            operator_1,
            operator_2,
            operator_3,
            operator_4,
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
        let operator_1 = self.operator_1.view(&self.sync_handle);
        let operator_2 = self.operator_2.view(&self.sync_handle);
        let operator_3 = self.operator_3.view(&self.sync_handle);
        let operator_4 = self.operator_4.view(&self.sync_handle);

        let all = Column::new()
            .spacing(16)
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .align_items(Align::Start)
                            .push(
                                Row::new()
                                    // .push(Text::new("OctaSine"))
                            )
                        )
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .align_items(Align::End)
                            .push(
                                Row::new()
                                    .push(master_volume)
                                    .push(master_frequency)
                            )
                            // .push(Space::with_width(Length::Units(32)))
                        )
            )
            .push(Rule::horizontal(0))
            .push(operator_4)
            .push(Rule::horizontal(0))
            .push(operator_3)
            .push(Rule::horizontal(0))
            .push(operator_2)
            .push(Rule::horizontal(0))
            .push(operator_1);

        Container::new(all)
            .padding(16)
            .into()
    }
}