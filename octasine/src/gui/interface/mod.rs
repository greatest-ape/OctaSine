use iced_baseview::{executor, Align, Application, Command, Subscription, WindowSubs};
use iced_baseview::{
    Column, Element, Row, Container, Rule, Length, Space, renderer, Font, Point, Text, HorizontalAlignment
};

use crate::GuiSyncHandle;

mod envelope;
mod knob;
mod lfo;
mod mod_matrix;
mod operator;
mod preset_picker;
mod boolean_picker;

use lfo::LfoWidgets;
use operator::OperatorWidgets;
use knob::OctaSineKnob;
use mod_matrix::ModulationMatrix;
use preset_picker::PresetPicker;


pub const FONT_SIZE: u16 = 14;
pub const LINE_HEIGHT: u16 = 14;

const FONT_REGULAR: &[u8] = OPEN_SANS_REGULAR;

const FONT_BOLD: Font = Font::External {
    name: "Open Sans Semi Bold",
    bytes: OPEN_SANS_SEMI_BOLD,
};
const FONT_VERY_BOLD: Font = Font::External {
    name: "Open Sans Bold",
    bytes: OPEN_SANS_BOLD,
};

const OPEN_SANS_REGULAR: &[u8] = include_bytes!(
    "../../../../contrib/open-sans/OpenSans-Regular.ttf"
);
const OPEN_SANS_SEMI_BOLD: &[u8] = include_bytes!(
    "../../../../contrib/open-sans/OpenSans-SemiBold.ttf"
);
const OPEN_SANS_BOLD: &[u8] = include_bytes!(
    "../../../../contrib/open-sans/OpenSans-Bold.ttf"
);


pub trait SnapPoint {
    fn snap(self) -> Self;
}


impl SnapPoint for Point {
    fn snap(self) -> Self {
        Point {
            x: self.x.floor() + 0.5,
            y: self.y.floor() + 0.5,
        }
    }
}


#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    ParameterChange(usize, f64),
    PresetChange(usize),
}


pub struct OctaSineIcedApplication<H: GuiSyncHandle> {
    sync_handle: H,
    host_display_needs_update: bool,
    frame_counter: usize,
    master_volume: OctaSineKnob,
    master_frequency: OctaSineKnob,
    modulation_matrix: ModulationMatrix,
    preset_picker: PresetPicker,
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
    lfo_1: LfoWidgets,
    lfo_2: LfoWidgets,
    lfo_3: LfoWidgets,
    lfo_4: LfoWidgets,
}


impl <H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(
        &mut self,
        parameter_index: usize,
        value: f64,
    ){
        let v = value;
        let h = &self.sync_handle;

        match parameter_index {
            0 => self.master_volume.set_value(h, v),
            1 => self.master_frequency.set_value(h, v),
            2 => {
                self.operator_1.volume.set_value(h, v);
                self.modulation_matrix.set_operator_1_volume(value);
            },
            3 => self.operator_1.panning.set_value(h, v),
            4 => self.operator_1.wave_type.set_value(v),
            5 => self.operator_1.mod_index.set_value(h, v),
            6 => {
                self.operator_1.feedback.set_value(h, v);
                self.modulation_matrix.set_operator_1_feedback(value);
            },
            7 => self.operator_1.frequency_ratio.set_value(h, v),
            8 => self.operator_1.frequency_free.set_value(h, v),
            9 => self.operator_1.frequency_fine.set_value(h, v),
            10 => self.operator_1.envelope.set_attack_duration(v),
            11 => self.operator_1.envelope.set_attack_end_value(v),
            12 => self.operator_1.envelope.set_decay_duration(v),
            13 => self.operator_1.envelope.set_decay_end_value(v),
            14 => self.operator_1.envelope.set_release_duration(v),
            15 => {
                self.operator_2.volume.set_value(h, v);
                    self.modulation_matrix.set_operator_2_volume(value);
            },
            16 => self.operator_2.panning.set_value(h, v),
            17 => self.operator_2.wave_type.set_value(v),
            18 => {
                self.operator_2.additive.as_mut()
                    .unwrap()
                    .set_value(h, v);
                self.modulation_matrix.set_operator_2_additive(v);
            },
            19 => self.operator_2.mod_index.set_value(h, v),
            20 => {
                self.operator_2.feedback.set_value(h, v);
                self.modulation_matrix.set_operator_2_feedback(value);
            },
            21 => self.operator_2.frequency_ratio.set_value(h, v),
            22 => self.operator_2.frequency_free.set_value(h, v),
            23 => self.operator_2.frequency_fine.set_value(h, v),
            24 => self.operator_2.envelope.set_attack_duration(v),
            25 => self.operator_2.envelope.set_attack_end_value(v),
            26 => self.operator_2.envelope.set_decay_duration(v),
            27 => self.operator_2.envelope.set_decay_end_value(v),
            28 => self.operator_2.envelope.set_release_duration(v),
            29 => {
                self.modulation_matrix.set_operator_3_volume(value);
                self.operator_3.volume.set_value(h, v);
            },
            30 => self.operator_3.panning.set_value(h, v),
            31 => self.operator_3.wave_type.set_value(v),
            32 => {
                self.operator_3.additive.as_mut()
                    .unwrap()
                    .set_value(h, v);
                self.modulation_matrix.set_operator_3_additive(v);
            },
            33 => self.modulation_matrix.set_operator_3_target(v),
            34 => self.operator_3.mod_index.set_value(h, v),
            35 => {
                self.operator_3.feedback.set_value(h, v);
                self.modulation_matrix.set_operator_3_feedback(value)
            },
            36 => self.operator_3.frequency_ratio.set_value(h, v),
            37 => self.operator_3.frequency_free.set_value(h, v),
            38 => self.operator_3.frequency_fine.set_value(h, v),
            39 => self.operator_3.envelope.set_attack_duration(v),
            40 => self.operator_3.envelope.set_attack_end_value(v),
            41 => self.operator_3.envelope.set_decay_duration(v),
            42 => self.operator_3.envelope.set_decay_end_value(v),
            43 => self.operator_3.envelope.set_release_duration(v),
            44 => {
                self.operator_4.volume.set_value(h, v);
                self.modulation_matrix.set_operator_4_volume(value);
            },
            45 => self.operator_4.panning.set_value(h, v),
            46 => self.operator_4.wave_type.set_value(v),
            47 => {
                self.operator_4.additive.as_mut()
                    .unwrap()
                    .set_value(h, v);
                self.modulation_matrix.set_operator_4_additive(v);
            },
            48 => self.modulation_matrix.set_operator_4_target(v),
            49 => self.operator_4.mod_index.set_value(h, v),
            50 => {
                self.operator_4.feedback.set_value(h, v);
                self.modulation_matrix.set_operator_4_feedback(value);
            },
            51 => self.operator_4.frequency_ratio.set_value(h, v),
            52 => self.operator_4.frequency_free.set_value(h, v),
            53 => self.operator_4.frequency_fine.set_value(h, v),
            54 => self.operator_4.envelope.set_attack_duration(v),
            55 => self.operator_4.envelope.set_attack_end_value(v),
            56 => self.operator_4.envelope.set_decay_duration(v),
            57 => self.operator_4.envelope.set_decay_end_value(v),
            58 => self.operator_4.envelope.set_release_duration(v),
            _ => (),
        }
    }

    fn update_widgets_from_parameters(&mut self){
        let opt_changes = self.sync_handle.get_bank()
            .get_changed_parameters_from_gui();
        
        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    self.set_value(index, *new_value);
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
        let preset_picker = PresetPicker::new(&sync_handle);

        let operator_1 = OperatorWidgets::new(&sync_handle, 0);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3);

        let lfo_1 = LfoWidgets::new(&sync_handle, 0);
        let lfo_2 = LfoWidgets::new(&sync_handle, 1);
        let lfo_3 = LfoWidgets::new(&sync_handle, 2);
        let lfo_4 = LfoWidgets::new(&sync_handle, 3);

        let app = Self {
            sync_handle,
            host_display_needs_update: false,
            frame_counter: 0,
            master_volume,
            master_frequency,
            modulation_matrix,
            preset_picker,
            operator_1,
            operator_2,
            operator_3,
            operator_4,
            lfo_1,
            lfo_2,
            lfo_3,
            lfo_4,
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
    
    fn renderer_settings() -> renderer::Settings {
        renderer::Settings {
            default_font: Some(FONT_REGULAR),
            default_text_size: FONT_SIZE,
            antialiasing: Some(renderer::Antialiasing::MSAAx4),
            ..Default::default()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Frame => {
                if self.sync_handle.get_bank().get_presets_changed(){
                    self.preset_picker = PresetPicker::new(&self.sync_handle);
                }
                self.update_widgets_from_parameters();

                // Update host display less often for better performance.
                // This is not a good solution, but it is OK for now.
                if self.frame_counter % 8 == 0 {
                    if self.host_display_needs_update {
                        self.sync_handle.update_host_display();

                        self.host_display_needs_update = false;
                    }
                }

                self.frame_counter = self.frame_counter.wrapping_add(1);
            },
            Message::ParameterChange(index, value) => {
                self.set_value(index, value);

                self.sync_handle.set_parameter(index, value);
                self.host_display_needs_update = true;
            },
            Message::PresetChange(index) => {
                self.sync_handle.set_preset_index(index);
                self.host_display_needs_update = true;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view();
        let master_frequency = self.master_frequency.view();
        let modulation_matrix = self.modulation_matrix.view();
        let preset_picker = self.preset_picker.view();
        let operator_1 = self.operator_1.view();
        let operator_2 = self.operator_2.view();
        let operator_3 = self.operator_3.view();
        let operator_4 = self.operator_4.view();
        let lfo_1 = self.lfo_1.view();
        let lfo_2 = self.lfo_2.view();
        let lfo_3 = self.lfo_3.view();
        let lfo_4 = self.lfo_4.view();

        let all = Column::new()
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                    )
                    .push(
                        Container::new(
                            Text::new("OctaSine")
                                .font(FONT_VERY_BOLD)
                                .size(FONT_SIZE * 2)
                                .horizontal_alignment(HorizontalAlignment::Center)
                        )
                            .width(Length::FillPortion(1))
                            .align_y(Align::Center)
                            .align_x(Align::Center)
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::End)
                    )
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_4)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_3)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_2)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_1)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(
                Row::new()
                    .push(lfo_1)
                    .push(
                        Container::new(
                            Rule::vertical(LINE_HEIGHT)
                        )
                            .height(Length::Units(LINE_HEIGHT * 6))
                    )
                    .push(lfo_2)
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(
                Row::new()
                    .push(lfo_3)
                    .push(
                        Container::new(
                            Rule::vertical(LINE_HEIGHT)
                        )
                            .height(Length::Units(LINE_HEIGHT * 6))
                    )
                    .push(lfo_4)
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(
                Row::new()
                    .height(Length::Units(mod_matrix::HEIGHT))
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .push(
                                Row::new()
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                                    .push(preset_picker)
                            )
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::End)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(
                                        Space::with_width(Length::Units(LINE_HEIGHT))
                                    )
                                    .push(modulation_matrix)
                                    .push(
                                        Space::with_width(Length::Units(LINE_HEIGHT))
                                    )
                                    .push(
                                        Container::new(
                                            Rule::vertical(LINE_HEIGHT)
                                        )
                                            .height(Length::Units(mod_matrix::HEIGHT)))
                                    .push(master_volume)
                                    .push(master_frequency)
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                            )
                    )
            )
            ;

        Container::new(all)
            .into()
    }
}
