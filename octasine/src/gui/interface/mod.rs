use std::sync::Arc;

use iced_baseview::{executor, Application, Command, Align};
use iced_baseview::{
    Column, Element, Row, Container, Rule, Text, Length, Space
};
use iced_audio::Normal;

use crate::SyncHandle;

mod envelope;
mod knob;
mod operator;

use operator::OperatorWidgets;
use knob::OctaSineKnob;


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
                        2 => ParameterWidget::<H>::set_value(&mut self.operator_1.volume, *new_value),
                        3 => ParameterWidget::<H>::set_value(&mut self.operator_1.panning, *new_value),
                        4 => (),
                        5 => ParameterWidget::<H>::set_value(&mut self.operator_1.mod_index, *new_value),
                        6 => ParameterWidget::<H>::set_value(&mut self.operator_1.feedback, *new_value),
                        7 => ParameterWidget::<H>::set_value(&mut self.operator_1.frequency_ratio, *new_value),
                        8 => ParameterWidget::<H>::set_value(&mut self.operator_1.frequency_free, *new_value),
                        9 => ParameterWidget::<H>::set_value(&mut self.operator_1.frequency_fine, *new_value),
                        10 => self.operator_1.envelope.set_attack_duration(*new_value),
                        11 => self.operator_1.envelope.set_attack_end_value(*new_value),
                        12 => self.operator_1.envelope.set_decay_duration(*new_value),
                        13 => self.operator_1.envelope.set_decay_end_value(*new_value),
                        14 => self.operator_1.envelope.set_release_duration(*new_value),
                        15 => ParameterWidget::<H>::set_value(&mut self.operator_2.volume, *new_value),
                        16 => ParameterWidget::<H>::set_value(&mut self.operator_2.panning, *new_value),
                        17 => (),
                        18 => (),
                        19 => ParameterWidget::<H>::set_value(&mut self.operator_2.mod_index, *new_value),
                        20 => ParameterWidget::<H>::set_value(&mut self.operator_2.feedback, *new_value),
                        21 => ParameterWidget::<H>::set_value(&mut self.operator_2.frequency_ratio, *new_value),
                        22 => ParameterWidget::<H>::set_value(&mut self.operator_2.frequency_free, *new_value),
                        23 => ParameterWidget::<H>::set_value(&mut self.operator_2.frequency_fine, *new_value),
                        24 => self.operator_2.envelope.set_attack_duration(*new_value),
                        25 => self.operator_2.envelope.set_attack_end_value(*new_value),
                        26 => self.operator_2.envelope.set_decay_duration(*new_value),
                        27 => self.operator_2.envelope.set_decay_end_value(*new_value),
                        28 => self.operator_2.envelope.set_release_duration(*new_value),
                        29 => ParameterWidget::<H>::set_value(&mut self.operator_3.volume, *new_value),
                        30 => ParameterWidget::<H>::set_value(&mut self.operator_3.panning, *new_value),
                        31 => (),
                        32 => (),
                        33 => (),
                        34 => ParameterWidget::<H>::set_value(&mut self.operator_3.mod_index, *new_value),
                        35 => ParameterWidget::<H>::set_value(&mut self.operator_3.feedback, *new_value),
                        36 => ParameterWidget::<H>::set_value(&mut self.operator_3.frequency_ratio, *new_value),
                        37 => ParameterWidget::<H>::set_value(&mut self.operator_3.frequency_free, *new_value),
                        38 => ParameterWidget::<H>::set_value(&mut self.operator_3.frequency_fine, *new_value),
                        39 => self.operator_3.envelope.set_attack_duration(*new_value),
                        40 => self.operator_3.envelope.set_attack_end_value(*new_value),
                        41 => self.operator_3.envelope.set_decay_duration(*new_value),
                        42 => self.operator_3.envelope.set_decay_end_value(*new_value),
                        43 => self.operator_3.envelope.set_release_duration(*new_value),
                        44 => ParameterWidget::<H>::set_value(&mut self.operator_4.volume, *new_value),
                        45 => ParameterWidget::<H>::set_value(&mut self.operator_4.panning, *new_value),
                        46 => (),
                        47 => (),
                        48 => (),
                        49 => ParameterWidget::<H>::set_value(&mut self.operator_4.mod_index, *new_value),
                        50 => ParameterWidget::<H>::set_value(&mut self.operator_4.feedback, *new_value),
                        51 => ParameterWidget::<H>::set_value(&mut self.operator_4.frequency_ratio, *new_value),
                        52 => ParameterWidget::<H>::set_value(&mut self.operator_4.frequency_free, *new_value),
                        53 => ParameterWidget::<H>::set_value(&mut self.operator_4.frequency_fine, *new_value),
                        54 => self.operator_4.envelope.set_attack_duration(*new_value),
                        55 => self.operator_4.envelope.set_attack_end_value(*new_value),
                        56 => self.operator_4.envelope.set_decay_duration(*new_value),
                        57 => self.operator_4.envelope.set_decay_end_value(*new_value),
                        58 => self.operator_4.envelope.set_release_duration(*new_value),
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