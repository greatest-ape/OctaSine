use iced_baseview::{Align, Application, Command, Subscription, WindowSubs, executor};
use iced_baseview::{
    Column, Element, Row, Container, Length, Space, renderer, Font, Point, Text, HorizontalAlignment, VerticalAlignment
};

use crate::GuiSyncHandle;
use crate::parameters::values::{MasterVolumeValue, MasterFrequencyValue};

mod divider;
mod envelope;
mod knob;
mod lfo;
mod lfo_target_picker;
mod mod_matrix;
mod operator;
mod preset_picker;
mod style;
mod boolean_picker;

use divider::VerticalRule;
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
    ParameterChanges(Vec<(usize, f64)>),
    PresetChange(usize),
    EnvelopeZoomIn(usize),
    EnvelopeZoomOut(usize),
    EnvelopeSyncViewports {
        viewport_factor: f32,
        x_offset: f32,
    },
}


pub struct OctaSineIcedApplication<H: GuiSyncHandle> {
    sync_handle: H,
    host_display_needs_update: bool,
    frame_counter: usize,
    master_volume: OctaSineKnob<MasterVolumeValue>,
    master_frequency: OctaSineKnob<MasterFrequencyValue>,
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
    lfo_vr_1: divider::VerticalRule,
    lfo_vr_2: divider::VerticalRule,
    lfo_vr_3: divider::VerticalRule,
    lfo_vr_4: divider::VerticalRule,
}


impl <H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(
        &mut self,
        parameter_index: usize,
        value: f64,
    ){
        let v = value;

        match parameter_index {
            0 => self.master_volume.set_value(v),
            1 => self.master_frequency.set_value(v),
            2 => {
                self.operator_1.volume.set_value(v);
                self.modulation_matrix.set_operator_1_volume(value);
            },
            3 => self.operator_1.panning.set_value(v),
            4 => self.operator_1.wave_type.set_value(v),
            5 => self.operator_1.mod_index.set_value(v),
            6 => {
                self.operator_1.feedback.set_value(v);
                self.modulation_matrix.set_operator_1_feedback(value);
            },
            7 => self.operator_1.frequency_ratio.set_value(v),
            8 => self.operator_1.frequency_free.set_value(v),
            9 => self.operator_1.frequency_fine.set_value(v),
            10 => self.operator_1.envelope.set_attack_duration(v),
            11 => self.operator_1.envelope.set_attack_end_value(v),
            12 => self.operator_1.envelope.set_decay_duration(v),
            13 => self.operator_1.envelope.set_decay_end_value(v),
            14 => self.operator_1.envelope.set_release_duration(v),
            15 => {
                self.operator_2.volume.set_value(v);
                    self.modulation_matrix.set_operator_2_volume(value);
            },
            16 => self.operator_2.panning.set_value(v),
            17 => self.operator_2.wave_type.set_value(v),
            18 => {
                self.operator_2.additive.as_mut()
                    .unwrap()
                    .set_value(v);
                self.modulation_matrix.set_operator_2_additive(v);
            },
            19 => self.operator_2.mod_index.set_value(v),
            20 => {
                self.operator_2.feedback.set_value(v);
                self.modulation_matrix.set_operator_2_feedback(value);
            },
            21 => self.operator_2.frequency_ratio.set_value(v),
            22 => self.operator_2.frequency_free.set_value(v),
            23 => self.operator_2.frequency_fine.set_value(v),
            24 => self.operator_2.envelope.set_attack_duration(v),
            25 => self.operator_2.envelope.set_attack_end_value(v),
            26 => self.operator_2.envelope.set_decay_duration(v),
            27 => self.operator_2.envelope.set_decay_end_value(v),
            28 => self.operator_2.envelope.set_release_duration(v),
            29 => {
                self.modulation_matrix.set_operator_3_volume(value);
                self.operator_3.volume.set_value(v);
            },
            30 => self.operator_3.panning.set_value(v),
            31 => self.operator_3.wave_type.set_value(v),
            32 => {
                self.operator_3.additive.as_mut()
                    .unwrap()
                    .set_value(v);
                self.modulation_matrix.set_operator_3_additive(v);
            },
            33 => self.modulation_matrix.set_operator_3_target(v),
            34 => self.operator_3.mod_index.set_value(v),
            35 => {
                self.operator_3.feedback.set_value(v);
                self.modulation_matrix.set_operator_3_feedback(value)
            },
            36 => self.operator_3.frequency_ratio.set_value(v),
            37 => self.operator_3.frequency_free.set_value(v),
            38 => self.operator_3.frequency_fine.set_value(v),
            39 => self.operator_3.envelope.set_attack_duration(v),
            40 => self.operator_3.envelope.set_attack_end_value(v),
            41 => self.operator_3.envelope.set_decay_duration(v),
            42 => self.operator_3.envelope.set_decay_end_value(v),
            43 => self.operator_3.envelope.set_release_duration(v),
            44 => {
                self.operator_4.volume.set_value(v);
                self.modulation_matrix.set_operator_4_volume(value);
            },
            45 => self.operator_4.panning.set_value(v),
            46 => self.operator_4.wave_type.set_value(v),
            47 => {
                self.operator_4.additive.as_mut()
                    .unwrap()
                    .set_value(v);
                self.modulation_matrix.set_operator_4_additive(v);
            },
            48 => self.modulation_matrix.set_operator_4_target(v),
            49 => self.operator_4.mod_index.set_value(v),
            50 => {
                self.operator_4.feedback.set_value(v);
                self.modulation_matrix.set_operator_4_feedback(value);
            },
            51 => self.operator_4.frequency_ratio.set_value(v),
            52 => self.operator_4.frequency_free.set_value(v),
            53 => self.operator_4.frequency_fine.set_value(v),
            54 => self.operator_4.envelope.set_attack_duration(v),
            55 => self.operator_4.envelope.set_attack_end_value(v),
            56 => self.operator_4.envelope.set_decay_duration(v),
            57 => self.operator_4.envelope.set_decay_end_value(v),
            58 => self.operator_4.envelope.set_release_duration(v),
            59 => self.lfo_1.target.set_value(v),
            60 => self.lfo_1.bpm_sync.set_value(v),
            61 => self.lfo_1.frequency_ratio.set_value(v),
            62 => self.lfo_1.frequency_free.set_value(v),
            63 => self.lfo_1.mode.set_value(v),
            64 => self.lfo_1.shape.set_value(v),
            65 => self.lfo_1.amount.set_value(v),
            66 => self.lfo_2.target.set_value(v),
            67 => self.lfo_2.bpm_sync.set_value(v),
            68 => self.lfo_2.frequency_ratio.set_value(v),
            69 => self.lfo_2.frequency_free.set_value(v),
            70 => self.lfo_2.mode.set_value(v),
            71 => self.lfo_2.shape.set_value(v),
            72 => self.lfo_2.amount.set_value(v),
            73 => self.lfo_3.target.set_value(v),
            74 => self.lfo_3.bpm_sync.set_value(v),
            75 => self.lfo_3.frequency_ratio.set_value(v),
            76 => self.lfo_3.frequency_free.set_value(v),
            77 => self.lfo_3.mode.set_value(v),
            78 => self.lfo_3.shape.set_value(v),
            79 => self.lfo_3.amount.set_value(v),
            80 => self.lfo_4.target.set_value(v),
            81 => self.lfo_4.bpm_sync.set_value(v),
            82 => self.lfo_4.frequency_ratio.set_value(v),
            83 => self.lfo_4.frequency_free.set_value(v),
            84 => self.lfo_4.mode.set_value(v),
            85 => self.lfo_4.shape.set_value(v),
            86 => self.lfo_4.amount.set_value(v),
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
        let master_volume = knob::master_volume(&sync_handle);
        let master_frequency = knob::master_frequency(&sync_handle);
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

        let lfo_vr_1 = VerticalRule::new(
            Length::Units(LINE_HEIGHT * 2),
            Length::Units(LINE_HEIGHT * 16)
        );
        let lfo_vr_2 = lfo_vr_1.clone();
        let lfo_vr_3 = lfo_vr_1.clone();
        let lfo_vr_4 = lfo_vr_1.clone();

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
            lfo_vr_1,
            lfo_vr_2,
            lfo_vr_3,
            lfo_vr_4,
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
            Message::EnvelopeZoomIn(operator_index) => {
                match operator_index {
                    0 => self.operator_1.envelope.zoom_in(),
                    1 => self.operator_2.envelope.zoom_in(),
                    2 => self.operator_3.envelope.zoom_in(),
                    3 => self.operator_4.envelope.zoom_in(),
                    _ => unreachable!(),
                }
            },
            Message::EnvelopeZoomOut(operator_index) => {
                match operator_index {
                    0 => self.operator_1.envelope.zoom_out(),
                    1 => self.operator_2.envelope.zoom_out(),
                    2 => self.operator_3.envelope.zoom_out(),
                    3 => self.operator_4.envelope.zoom_out(),
                    _ => unreachable!(),
                }
            },
            Message::EnvelopeSyncViewports { viewport_factor, x_offset } => {
                self.operator_1.envelope.set_viewport(viewport_factor, x_offset);
                self.operator_2.envelope.set_viewport(viewport_factor, x_offset);
                self.operator_3.envelope.set_viewport(viewport_factor, x_offset);
                self.operator_4.envelope.set_viewport(viewport_factor, x_offset);
            },
            Message::ParameterChange(index, value) => {
                self.set_value(index, value);

                self.sync_handle.set_parameter(index, value);
                self.host_display_needs_update = true;
            },
            Message::ParameterChanges(changes) => {
                for (index, value) in changes {
                    self.set_value(index, value);
                    self.sync_handle.set_parameter(index, value);
                }

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

        let master_title = Text::new("Master")
            .size(FONT_SIZE * 3 / 2)
            .height(Length::Units(LINE_HEIGHT * 2))
            .width(Length::Units(LINE_HEIGHT * 8))
            .font(FONT_VERY_BOLD)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center);

        let all = Column::new()
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .height(Length::Units(LINE_HEIGHT * 4))
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                    )
                    .push(
                        Container::new(
                            Text::new("OctaSine")
                                .font(FONT_VERY_BOLD)
                                .size(FONT_SIZE * 2 + FONT_SIZE / 2)
                                .horizontal_alignment(HorizontalAlignment::Center)
                        )
                            .width(Length::FillPortion(1))
                            .align_x(Align::Center)
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(1))
                            .align_items(Align::End)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                            .push(
                                Row::new()
                                    .push(preset_picker)
                                    .push(
                                        Space::with_width(Length::Units(LINE_HEIGHT))
                                    )
                            )
                    )
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(operator_4)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_3)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_2)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
            .push(operator_1)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(
                        Space::with_width(Length::Units(LINE_HEIGHT))
                    )
                    .push(lfo_1)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(
                                LINE_HEIGHT * 3
                            )))
                            .push(self.lfo_vr_1.view())
                    )
                    .push(lfo_2)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(
                                LINE_HEIGHT * 3
                            )))
                            .push(self.lfo_vr_2.view())
                    )
                    .push(lfo_3)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(
                                LINE_HEIGHT * 3
                            )))
                            .push(self.lfo_vr_3.view())
                    )
                    .push(lfo_4)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(
                                LINE_HEIGHT * 3
                            )))
                            .push(self.lfo_vr_4.view())
                    )
                    .push(
                        Column::new()
                            .width(Length::Units(LINE_HEIGHT * 8))
                            .push(
                                Row::new()
                                    .push(
                                        Container::new(master_title)
                                            .width(Length::Units(LINE_HEIGHT * 8))
                                            .height(Length::Units(LINE_HEIGHT * 2))
                                            .align_x(Align::Center)
                                            .align_y(Align::Center)
                                    )
                            )
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                            .push(
                                Row::new()
                                    .push(master_volume)
                                    .push(master_frequency)
                            )
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                            .push(
                                Row::new()
                                    .push(modulation_matrix)
                            )
                    )
            );

        Container::new(all)
            .into()
    }
}
