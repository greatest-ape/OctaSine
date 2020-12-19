use iced_baseview::{executor, Align, Application, Command, Subscription, WindowSubs};
use iced_baseview::{
    Column, Element, Row, Container, Rule, Length, Space
};

use crate::GuiSyncHandle;

mod envelope;
mod knob;
mod operator;
mod picker;
mod routing;

use operator::OperatorWidgets;
use knob::OctaSineKnob;
use routing::ModulationMatrix;


#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    ParameterChange(usize, f64),
}


pub struct OctaSineIcedApplication<H: GuiSyncHandle> {
    sync_handle: H,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
    modulation_matrix: ModulationMatrix,
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
}


impl <H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn update_widgets_from_parameters(&mut self){
        let opt_changes = self.sync_handle.get_presets()
            .get_changed_parameters_from_gui();
        
        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    let v = *new_value;

                    match index {
                        0 => self.master_volume.set_value(v),
                        1 => self.master_frequency.set_value(v),
                        2 => self.operator_1.volume.set_value(v),
                        3 => self.operator_1.panning.set_value(v),
                        4 => (),
                        5 => self.operator_1.mod_index.set_value(v),
                        6 => self.operator_1.feedback.set_value(v),
                        7 => self.operator_1.frequency_ratio.set_value(v),
                        8 => self.operator_1.frequency_free.set_value(v),
                        9 => self.operator_1.frequency_fine.set_value(v),
                        10 => self.operator_1.envelope.set_attack_duration(v),
                        11 => self.operator_1.envelope.set_attack_end_value(v),
                        12 => self.operator_1.envelope.set_decay_duration(v),
                        13 => self.operator_1.envelope.set_decay_end_value(v),
                        14 => self.operator_1.envelope.set_release_duration(v),
                        15 => self.operator_2.volume.set_value(v),
                        16 => self.operator_2.panning.set_value(v),
                        17 => (),
                        18 => {
                            self.operator_2.additive.as_mut()
                                .unwrap()
                                .set_value(v);
                            self.modulation_matrix.set_operator_2_additive(v);
                        },
                        19 => self.operator_2.mod_index.set_value(v),
                        20 => self.operator_2.feedback.set_value(v),
                        21 => self.operator_2.frequency_ratio.set_value(v),
                        22 => self.operator_2.frequency_free.set_value(v),
                        23 => self.operator_2.frequency_fine.set_value(v),
                        24 => self.operator_2.envelope.set_attack_duration(v),
                        25 => self.operator_2.envelope.set_attack_end_value(v),
                        26 => self.operator_2.envelope.set_decay_duration(v),
                        27 => self.operator_2.envelope.set_decay_end_value(v),
                        28 => self.operator_2.envelope.set_release_duration(v),
                        29 => self.operator_3.volume.set_value(v),
                        30 => self.operator_3.panning.set_value(v),
                        31 => (),
                        32 => {
                            self.operator_3.additive.as_mut()
                                .unwrap()
                                .set_value(v);
                            self.modulation_matrix.set_operator_3_additive(v);
                        },
                        33 => self.modulation_matrix.set_operator_3_target(v),
                        34 => self.operator_3.mod_index.set_value(v),
                        35 => self.operator_3.feedback.set_value(v),
                        36 => self.operator_3.frequency_ratio.set_value(v),
                        37 => self.operator_3.frequency_free.set_value(v),
                        38 => self.operator_3.frequency_fine.set_value(v),
                        39 => self.operator_3.envelope.set_attack_duration(v),
                        40 => self.operator_3.envelope.set_attack_end_value(v),
                        41 => self.operator_3.envelope.set_decay_duration(v),
                        42 => self.operator_3.envelope.set_decay_end_value(v),
                        43 => self.operator_3.envelope.set_release_duration(v),
                        44 => self.operator_4.volume.set_value(v),
                        45 => self.operator_4.panning.set_value(v),
                        46 => (),
                        47 => {
                            self.operator_4.additive.as_mut()
                                .unwrap()
                                .set_value(v);
                            self.modulation_matrix.set_operator_4_additive(v);
                        },
                        48 => self.modulation_matrix.set_operator_4_target(v),
                        49 => self.operator_4.mod_index.set_value(v),
                        50 => self.operator_4.feedback.set_value(v),
                        51 => self.operator_4.frequency_ratio.set_value(v),
                        52 => self.operator_4.frequency_free.set_value(v),
                        53 => self.operator_4.frequency_fine.set_value(v),
                        54 => self.operator_4.envelope.set_attack_duration(v),
                        55 => self.operator_4.envelope.set_attack_end_value(v),
                        56 => self.operator_4.envelope.set_decay_duration(v),
                        57 => self.operator_4.envelope.set_decay_end_value(v),
                        58 => self.operator_4.envelope.set_release_duration(v),
                        _ => (),
                    }
                }
            }
        }
    }
}


impl <H: GuiSyncHandle>Application for OctaSineIcedApplication<H> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = H;

    fn new(
        sync_handle: Self::Flags,
    ) -> (Self, Command<Self::Message>) {
        let master_volume = OctaSineKnob::master_volume(&sync_handle);
        let master_frequency = OctaSineKnob::master_frequency(&sync_handle);
        let modulation_matrix = ModulationMatrix::new(&sync_handle);

        let operator_1 = OperatorWidgets::new(&sync_handle, 0);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3);

        let app = Self {
            sync_handle,
            master_volume,
            master_frequency,
            modulation_matrix,
            operator_1,
            operator_2,
            operator_3,
            operator_4,
        };

        (app, Command::none())
    }

    fn subscription(
        &self,
        window_subs: &mut WindowSubs<Self::Message>
    ) -> Subscription<Self::Message> {
        window_subs.on_frame = Some(Message::Frame);

        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Frame => {
                self.update_widgets_from_parameters();
            },
            Message::ParameterChange(index, value) => {
                self.sync_handle.get_presets().set_parameter_value_float_from_gui(
                    index,
                    value,
                );

                match index {
                    18 => self.modulation_matrix.set_operator_2_additive(value),
                    32 => self.modulation_matrix.set_operator_3_additive(value),
                    33 => self.modulation_matrix.set_operator_3_target(value),
                    47 => self.modulation_matrix.set_operator_4_additive(value),
                    48 => self.modulation_matrix.set_operator_4_target(value),
                    _ => ()
                }

                self.sync_handle.update_host_display();
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view(&self.sync_handle);
        let master_frequency = self.master_frequency.view(&self.sync_handle);
        let modulation_matrix = self.modulation_matrix.view();
        let operator_1 = self.operator_1.view(&self.sync_handle);
        let operator_2 = self.operator_2.view(&self.sync_handle);
        let operator_3 = self.operator_3.view(&self.sync_handle);
        let operator_4 = self.operator_4.view(&self.sync_handle);

        let all = Column::new()
            .spacing(16)
            .push(Rule::horizontal(0))
            .push(operator_4)
            .push(Rule::horizontal(0))
            .push(operator_3)
            .push(Rule::horizontal(0))
            .push(operator_2)
            .push(Rule::horizontal(0))
            .push(operator_1)
            .push(Rule::horizontal(0))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .align_items(Align::Start)
                            .push(
                                Row::new()
                            )
                        )
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .align_items(Align::End)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(modulation_matrix)
                                    .push(
                                        Space::with_width(Length::Units(16))
                                    )
                                    .push(
                                        Container::new(
                                            Rule::vertical(16)
                                        )
                                            .height(Length::Units(routing::HEIGHT)))
                                    .push(master_volume)
                                    .push(master_frequency)
                            )
                            // .push(Space::with_width(Length::Units(32)))
                        )
            )
            .push(Rule::horizontal(0));

        Container::new(all)
            .padding(16)
            .into()
    }
}