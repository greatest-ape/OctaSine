use iced_baseview::{
    alignment::Horizontal, alignment::Vertical, button, Button, Color, Column, Container, Element,
    Font, Length, Point, Row, Rule, Space, Text, WindowQueue,
};
use iced_baseview::{executor, Alignment, Application, Command, Subscription, WindowSubs};

use crate::parameter_values::{MasterFrequencyValue, MasterVolumeValue};
use crate::{get_version_info, sync::GuiSyncHandle};

mod boolean_picker;
mod common;
mod envelope;
mod knob;
mod lfo;
mod lfo_target_picker;
mod mod_matrix;
mod mod_target_picker;
mod mute_button;
mod operator;
mod patch_picker;
pub mod style;
mod wave_picker;

use knob::OctaSineKnob;
use lfo::LfoWidgets;
use mod_matrix::ModulationMatrix;
use operator::OperatorWidgets;
use patch_picker::PatchPicker;
use style::Theme;

use self::common::{container_l1, container_l2, container_l3, space_l3};
use self::operator::ModTargetPicker;
use self::style::Style;

use super::GuiSettings;
use crate::settings::Settings;

pub const FONT_SIZE: u16 = 12;
pub const LINE_HEIGHT: u16 = 12;

const FONT_REGULAR: &[u8] = OPEN_SANS_REGULAR;

const FONT_BOLD: Font = Font::External {
    name: "Open Sans Semi Bold",
    bytes: OPEN_SANS_SEMI_BOLD,
};
const FONT_VERY_BOLD: Font = Font::External {
    name: "Open Sans Bold",
    bytes: OPEN_SANS_BOLD,
};

const OPEN_SANS_REGULAR: &[u8] =
    include_bytes!("../../../../contrib/open-sans/OpenSans-Regular.ttf");
const OPEN_SANS_SEMI_BOLD: &[u8] =
    include_bytes!("../../../../contrib/open-sans/OpenSans-SemiBold.ttf");
const OPEN_SANS_BOLD: &[u8] = include_bytes!("../../../../contrib/open-sans/OpenSans-Bold.ttf");

fn get_info_text() -> String {
    format!(
        "Copyright © 2019-2022 Joakim Frostegård\nSite: OctaSine.com. Build: {}",
        get_version_info()
    )
}

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
    ChangeSingleParameterBegin(usize),
    ChangeSingleParameterEnd(usize),
    ChangeSingleParameterSetValue(usize, f64),
    ChangeSingleParameterImmediate(usize, f64),
    ChangeTwoParametersBegin((usize, usize)),
    ChangeTwoParametersEnd((usize, usize)),
    ChangeTwoParametersSetValues((usize, f64), (usize, f64)),
    ToggleInfo,
    PatchChange(usize),
    EnvelopeZoomIn(usize),
    EnvelopeZoomOut(usize),
    EnvelopeZoomToFit(usize),
    EnvelopeSyncViewports { viewport_factor: f32, x_offset: f32 },
    ToggleColorMode,
}

pub struct OctaSineIcedApplication<H: GuiSyncHandle> {
    sync_handle: H,
    style: style::Theme,
    toggle_info_state: button::State,
    toggle_style_state: button::State,
    show_version: bool,
    master_volume: OctaSineKnob<MasterVolumeValue>,
    master_frequency: OctaSineKnob<MasterFrequencyValue>,
    modulation_matrix: ModulationMatrix,
    patch_picker: PatchPicker,
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
    lfo_1: LfoWidgets,
    lfo_2: LfoWidgets,
    lfo_3: LfoWidgets,
    lfo_4: LfoWidgets,
}

impl<H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(&mut self, parameter_index: usize, value: f64) {
        let v = value;

        match parameter_index {
            0 => self.master_volume.set_value(v),
            1 => self.master_frequency.set_value(v),
            2 => self.operator_1.volume.set_value(v),
            3 => self.operator_1.mute_button.set_value(v),
            4 => {
                self.operator_1.mix.set_value(v);
                self.modulation_matrix.set_operator_1_mix(value);
            }
            5 => self.operator_1.panning.set_value(v),
            6 => self.operator_1.wave_type.set_value(v),
            7 => self.operator_1.feedback.set_value(v),
            8 => self.operator_1.frequency_ratio.set_value(v),
            9 => self.operator_1.frequency_free.set_value(v),
            10 => self.operator_1.frequency_fine.set_value(v),
            11 => self.operator_1.envelope.set_attack_duration(v),
            12 => self.operator_1.envelope.set_attack_end_value(v),
            13 => self.operator_1.envelope.set_decay_duration(v),
            14 => self.operator_1.envelope.set_decay_end_value(v),
            15 => self.operator_1.envelope.set_release_duration(v),
            16 => self.operator_2.volume.set_value(v),
            17 => self.operator_2.mute_button.set_value(v),
            18 => {
                self.operator_2.mix.set_value(v);
                self.modulation_matrix.set_operator_2_mix(value);
            }
            19 => self.operator_2.panning.set_value(v),
            20 => self.operator_2.wave_type.set_value(v),
            21 => {
                match self.operator_2.mod_target.as_mut() {
                    Some(ModTargetPicker::Operator2(p)) => p.set_value(v),
                    _ => {}
                }
                self.modulation_matrix.set_operator_2_target(v);
            }
            22 => {
                if let Some(mod_index) = self.operator_2.mod_index.as_mut() {
                    mod_index.set_value(v)
                }
                self.modulation_matrix.set_operator_2_mod(v);
            }
            23 => self.operator_2.feedback.set_value(v),
            24 => self.operator_2.frequency_ratio.set_value(v),
            25 => self.operator_2.frequency_free.set_value(v),
            26 => self.operator_2.frequency_fine.set_value(v),
            27 => self.operator_2.envelope.set_attack_duration(v),
            28 => self.operator_2.envelope.set_attack_end_value(v),
            29 => self.operator_2.envelope.set_decay_duration(v),
            30 => self.operator_2.envelope.set_decay_end_value(v),
            31 => self.operator_2.envelope.set_release_duration(v),
            32 => self.operator_3.volume.set_value(v),
            33 => self.operator_3.mute_button.set_value(v),
            34 => {
                self.modulation_matrix.set_operator_3_mix(value);
                self.operator_3.mix.set_value(v);
            }
            35 => self.operator_3.panning.set_value(v),
            36 => self.operator_3.wave_type.set_value(v),
            37 => {
                match self.operator_3.mod_target.as_mut() {
                    Some(ModTargetPicker::Operator3(p)) => p.set_value(v),
                    _ => {}
                }
                self.modulation_matrix.set_operator_3_target(v);
            }
            38 => {
                if let Some(mod_index) = self.operator_3.mod_index.as_mut() {
                    mod_index.set_value(v)
                }
                self.modulation_matrix.set_operator_3_mod(v);
            }
            39 => self.operator_3.feedback.set_value(v),
            40 => self.operator_3.frequency_ratio.set_value(v),
            41 => self.operator_3.frequency_free.set_value(v),
            42 => self.operator_3.frequency_fine.set_value(v),
            43 => self.operator_3.envelope.set_attack_duration(v),
            44 => self.operator_3.envelope.set_attack_end_value(v),
            45 => self.operator_3.envelope.set_decay_duration(v),
            46 => self.operator_3.envelope.set_decay_end_value(v),
            47 => self.operator_3.envelope.set_release_duration(v),
            48 => self.operator_4.volume.set_value(v),
            49 => self.operator_4.mute_button.set_value(v),
            50 => {
                self.operator_4.mix.set_value(v);
                self.modulation_matrix.set_operator_4_mix(value);
            }
            51 => self.operator_4.panning.set_value(v),
            52 => self.operator_4.wave_type.set_value(v),
            53 => {
                match self.operator_4.mod_target.as_mut() {
                    Some(ModTargetPicker::Operator4(p)) => p.set_value(v),
                    _ => {}
                }
                self.modulation_matrix.set_operator_4_target(v);
            }
            54 => {
                if let Some(mod_index) = self.operator_4.mod_index.as_mut() {
                    mod_index.set_value(v)
                }
                self.modulation_matrix.set_operator_4_mod(v);
            }
            55 => self.operator_4.feedback.set_value(v),
            56 => self.operator_4.frequency_ratio.set_value(v),
            57 => self.operator_4.frequency_free.set_value(v),
            58 => self.operator_4.frequency_fine.set_value(v),
            59 => self.operator_4.envelope.set_attack_duration(v),
            60 => self.operator_4.envelope.set_attack_end_value(v),
            61 => self.operator_4.envelope.set_decay_duration(v),
            62 => self.operator_4.envelope.set_decay_end_value(v),
            63 => self.operator_4.envelope.set_release_duration(v),
            64 => self.lfo_1.target.set_value(v),
            65 => self.lfo_1.bpm_sync.set_value(v),
            66 => self.lfo_1.frequency_ratio.set_value(v),
            67 => self.lfo_1.frequency_free.set_value(v),
            68 => self.lfo_1.mode.set_value(v),
            69 => self.lfo_1.shape.set_value(v),
            70 => self.lfo_1.amount.set_value(v),
            71 => self.lfo_2.target.set_value(v),
            72 => self.lfo_2.bpm_sync.set_value(v),
            73 => self.lfo_2.frequency_ratio.set_value(v),
            74 => self.lfo_2.frequency_free.set_value(v),
            75 => self.lfo_2.mode.set_value(v),
            76 => self.lfo_2.shape.set_value(v),
            77 => self.lfo_2.amount.set_value(v),
            78 => self.lfo_3.target.set_value(v),
            79 => self.lfo_3.bpm_sync.set_value(v),
            80 => self.lfo_3.frequency_ratio.set_value(v),
            81 => self.lfo_3.frequency_free.set_value(v),
            82 => self.lfo_3.mode.set_value(v),
            83 => self.lfo_3.shape.set_value(v),
            84 => self.lfo_3.amount.set_value(v),
            85 => self.lfo_4.target.set_value(v),
            86 => self.lfo_4.bpm_sync.set_value(v),
            87 => self.lfo_4.frequency_ratio.set_value(v),
            88 => self.lfo_4.frequency_free.set_value(v),
            89 => self.lfo_4.mode.set_value(v),
            90 => self.lfo_4.shape.set_value(v),
            91 => self.lfo_4.amount.set_value(v),
            _ => (),
        }
    }

    fn update_widgets_from_parameters(&mut self) {
        let opt_changes = self.sync_handle.get_changed_parameters();

        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate() {
                if let Some(new_value) = opt_new_value {
                    self.set_value(index, *new_value);
                }
            }
        }
    }

    fn save_settings(&self) {
        let settings = Settings {
            schema_version: 1,
            gui: GuiSettings { theme: self.style },
        };

        let builder = ::std::thread::Builder::new();

        let spawn_result = builder.spawn(move || {
            if let Err(err) = settings.save() {
                ::log::error!("Couldn't save settings: {}", err)
            }
        });

        if let Err(err) = spawn_result {
            ::log::error!("Couldn't spawn thread for saving settings: {}", err)
        }
    }
}

impl<H: GuiSyncHandle> Application for OctaSineIcedApplication<H> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = H;

    fn new(sync_handle: Self::Flags) -> (Self, Command<Self::Message>) {
        let style = sync_handle.get_gui_settings().theme;

        let master_volume = knob::master_volume(&sync_handle, style);
        let master_frequency = knob::master_frequency(&sync_handle, style);
        let modulation_matrix = ModulationMatrix::new(&sync_handle, style);
        let patch_picker = PatchPicker::new(&sync_handle, style);

        let operator_1 = OperatorWidgets::new(&sync_handle, 0, style);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1, style);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2, style);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3, style);

        let lfo_1 = LfoWidgets::new(&sync_handle, 0, style);
        let lfo_2 = LfoWidgets::new(&sync_handle, 1, style);
        let lfo_3 = LfoWidgets::new(&sync_handle, 2, style);
        let lfo_4 = LfoWidgets::new(&sync_handle, 3, style);

        let app = Self {
            sync_handle,
            style,
            toggle_info_state: button::State::default(),
            toggle_style_state: button::State::default(),
            show_version: false,
            master_volume,
            master_frequency,
            modulation_matrix,
            patch_picker,
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
        window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        window_subs.on_frame = Some(Message::Frame);

        Subscription::none()
    }

    #[cfg(feature = "gui_wgpu")]
    fn renderer_settings() -> iced_wgpu::settings::Settings {
        iced_wgpu::settings::Settings {
            present_mode: iced_wgpu::wgpu::PresentMode::Immediate,
            default_font: Some(FONT_REGULAR),
            default_text_size: FONT_SIZE,
            antialiasing: Some(iced_wgpu::settings::Antialiasing::MSAAx8),
            ..Default::default()
        }
    }

    /// Renderer settings with glow
    #[cfg(feature = "gui_glow")]
    fn renderer_settings() -> iced_glow::settings::Settings {
        iced_glow::settings::Settings {
            default_font: Some(FONT_REGULAR),
            default_text_size: FONT_SIZE,
            antialiasing: Some(iced_glow::settings::Antialiasing::MSAAx8),
            text_multithreading: false,
        }
    }

    fn update(
        &mut self,
        _window_queue: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::Frame => {
                if self.sync_handle.have_patches_changed() {
                    self.patch_picker = PatchPicker::new(&self.sync_handle, self.style);
                }
                self.update_widgets_from_parameters();
            }
            Message::ToggleInfo => {
                self.show_version = !self.show_version;
            }
            Message::EnvelopeZoomIn(operator_index) => match operator_index {
                0 => self.operator_1.envelope.zoom_in(),
                1 => self.operator_2.envelope.zoom_in(),
                2 => self.operator_3.envelope.zoom_in(),
                3 => self.operator_4.envelope.zoom_in(),
                _ => unreachable!(),
            },
            Message::EnvelopeZoomOut(operator_index) => match operator_index {
                0 => self.operator_1.envelope.zoom_out(),
                1 => self.operator_2.envelope.zoom_out(),
                2 => self.operator_3.envelope.zoom_out(),
                3 => self.operator_4.envelope.zoom_out(),
                _ => unreachable!(),
            },
            Message::EnvelopeZoomToFit(operator_index) => match operator_index {
                0 => self.operator_1.envelope.zoom_to_fit(),
                1 => self.operator_2.envelope.zoom_to_fit(),
                2 => self.operator_3.envelope.zoom_to_fit(),
                3 => self.operator_4.envelope.zoom_to_fit(),
                _ => unreachable!(),
            },
            Message::EnvelopeSyncViewports {
                viewport_factor,
                x_offset,
            } => {
                self.operator_1
                    .envelope
                    .set_viewport(viewport_factor, x_offset);
                self.operator_2
                    .envelope
                    .set_viewport(viewport_factor, x_offset);
                self.operator_3
                    .envelope
                    .set_viewport(viewport_factor, x_offset);
                self.operator_4
                    .envelope
                    .set_viewport(viewport_factor, x_offset);
            }
            Message::ChangeSingleParameterBegin(index) => {
                self.sync_handle.begin_edit(index);
            }
            Message::ChangeSingleParameterEnd(index) => {
                self.sync_handle.end_edit(index);
            }
            Message::ChangeSingleParameterSetValue(index, value) => {
                self.set_value(index, value);

                self.sync_handle.set_parameter(index, value);
            }
            Message::ChangeSingleParameterImmediate(index, value) => {
                self.set_value(index, value);

                self.sync_handle.begin_edit(index);
                self.sync_handle.set_parameter(index, value);
                self.sync_handle.end_edit(index);
            }
            Message::ChangeTwoParametersBegin((index_1, index_2)) => {
                self.sync_handle.begin_edit(index_1);
                self.sync_handle.begin_edit(index_2);
            }
            Message::ChangeTwoParametersEnd((index_1, index_2)) => {
                self.sync_handle.end_edit(index_1);
                self.sync_handle.end_edit(index_2);
            }
            Message::ChangeTwoParametersSetValues((index_1, value_1), (index_2, value_2)) => {
                self.set_value(index_1, value_1);
                self.set_value(index_2, value_2);

                self.sync_handle.set_parameter(index_1, value_1);
                self.sync_handle.set_parameter(index_2, value_2);
            }
            Message::PatchChange(index) => {
                self.sync_handle.set_patch_index(index);
            }
            Message::ToggleColorMode => {
                let style = if let Theme::Light = self.style {
                    Theme::Dark
                } else {
                    Theme::Light
                };

                self.style = style;
                self.master_volume.style = style;
                self.master_frequency.style = style;
                self.modulation_matrix.set_style(style);
                self.patch_picker.style = style;
                self.operator_1.set_style(style);
                self.operator_2.set_style(style);
                self.operator_3.set_style(style);
                self.operator_4.set_style(style);
                self.lfo_1.set_style(style);
                self.lfo_2.set_style(style);
                self.lfo_3.set_style(style);
                self.lfo_4.set_style(style);

                self.save_settings();
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let master_volume = self.master_volume.view();
        let master_frequency = self.master_frequency.view();
        let modulation_matrix = self.modulation_matrix.view();
        let patch_picker = self.patch_picker.view();
        let operator_1 = self.operator_1.view();
        let operator_2 = self.operator_2.view();
        let operator_3 = self.operator_3.view();
        let operator_4 = self.operator_4.view();
        let lfo_1 = self.lfo_1.view();
        let lfo_2 = self.lfo_2.view();
        let lfo_3 = self.lfo_3.view();
        let lfo_4 = self.lfo_4.view();

        let info_text_color = if self.show_version {
            self.style.text_color()
        } else {
            Color::TRANSPARENT
        };

        let all = Column::new()
            /*
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .height(Length::Units(LINE_HEIGHT * 4))
                    .push(
                        Column::new().width(Length::FillPortion(10)).push(
                            Container::new(
                                Row::new()
                                    .push(
                                        Button::new(
                                            &mut self.toggle_style_state,
                                            Text::new("MODE"),
                                        )
                                        .on_press(Message::ToggleColorMode)
                                        .style(self.style),
                                    )
                                    .push(Space::with_width(Length::Units(3)))
                                    .push(
                                        Button::new(&mut self.toggle_info_state, Text::new("INFO"))
                                            .on_press(Message::ToggleInfo)
                                            .style(self.style),
                                    )
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                                    .push(
                                        Text::new(get_info_text())
                                            .size(LINE_HEIGHT)
                                            .color(info_text_color)
                                            .vertical_alignment(Vertical::Center),
                                    ),
                            )
                            .height(Length::Units(LINE_HEIGHT * 4))
                            .padding(LINE_HEIGHT)
                            .align_y(Vertical::Center),
                        ),
                    )
                    .push(
                        Container::new(
                            Text::new("OctaSine")
                                .font(FONT_VERY_BOLD)
                                .color(self.style.heading_color())
                                .size(FONT_SIZE * 2 + FONT_SIZE / 2)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .width(Length::FillPortion(4))
                        .align_x(Horizontal::Center),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(10))
                            .align_items(Alignment::End)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(
                                Row::new()
                                    .push(patch_picker)
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT))),
                            ),
                    ),
            )
            */
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(operator_4)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(operator_3)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(operator_2)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(operator_1)
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(lfo_1)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(lfo_2),
                    )
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(
                        Column::new()
                            .push(lfo_3)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(lfo_4),
                    )
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(
                        Column::new()
                            .push(container_l1(
                                self.style,
                                Row::new()
                                    .push(
                                        Container::new(
                                            Text::new("Master")
                                                .size(FONT_SIZE * 3 / 2)
                                                .height(Length::Units(LINE_HEIGHT * 8))
                                                .font(FONT_VERY_BOLD)
                                                .color(self.style.heading_color())
                                                .horizontal_alignment(Horizontal::Center)
                                                .vertical_alignment(Vertical::Center),
                                        )
                                        .width(Length::Units(LINE_HEIGHT * 8))
                                        .height(Length::Units(LINE_HEIGHT * 8))
                                        .align_x(Horizontal::Center)
                                        .align_y(Vertical::Center),
                                    )
                                    .push(container_l2(
                                        self.style,
                                        Row::new()
                                            .push(container_l3(self.style, master_volume))
                                            .push(space_l3())
                                            .push(container_l3(self.style, master_frequency)),
                                    )),
                            ))
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(
                                Row::new()
                                    .push(container_l1(
                                        self.style,
                                        Row::new().push(
                                            Container::new(
                                                Column::new()
                                                    .align_items(Alignment::Center)
                                                    .push(
                                                        Text::new("Patch")
                                                            .size(FONT_SIZE * 3 / 2)
                                                            .width(Length::Units(LINE_HEIGHT * 10))
                                                            .font(FONT_VERY_BOLD)
                                                            .color(self.style.heading_color())
                                                            .horizontal_alignment(
                                                                Horizontal::Center,
                                                            )
                                                            .vertical_alignment(Vertical::Center),
                                                    )
                                                    .push(Space::with_height(Length::Units(
                                                        LINE_HEIGHT,
                                                    )))
                                                    .push(patch_picker),
                                            )
                                            .width(Length::Units(LINE_HEIGHT * 10))
                                            .height(Length::Units(LINE_HEIGHT * 8))
                                            .align_x(Horizontal::Center)
                                            .align_y(Vertical::Center),
                                        ),
                                    ))
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                                    .push(container_l1(
                                        self.style,
                                        Row::new().push(
                                            Container::new(
                                                Column::new()
                                                    .align_items(Alignment::Center)
                                                    .push(
                                                        Text::new("OctaSine")
                                                            .size(FONT_SIZE * 3 / 2)
                                                            .width(Length::Units(LINE_HEIGHT * 8))
                                                            .font(FONT_VERY_BOLD)
                                                            .color(self.style.heading_color())
                                                            .horizontal_alignment(
                                                                Horizontal::Center,
                                                            )
                                                            .vertical_alignment(Vertical::Center),
                                                    )
                                                    .push(Space::with_height(Length::Units(
                                                        LINE_HEIGHT,
                                                    )))
                                                    .push(
                                                        Row::new()
                                                            .push(
                                                                Button::new(
                                                                    &mut self.toggle_info_state,
                                                                    Text::new("INFO"),
                                                                )
                                                                .on_press(Message::ToggleInfo)
                                                                .style(self.style),
                                                            )
                                                            .push(Space::with_width(Length::Units(
                                                                LINE_HEIGHT / 2,
                                                            )))
                                                            .push(
                                                                Button::new(
                                                                    &mut self.toggle_style_state,
                                                                    Text::new("THEME"),
                                                                )
                                                                .on_press(Message::ToggleColorMode)
                                                                .style(self.style),
                                                            ),
                                                    ),
                                            )
                                            .width(Length::Units(LINE_HEIGHT * 10))
                                            .height(Length::Units(LINE_HEIGHT * 8))
                                            .align_x(Horizontal::Center)
                                            .align_y(Vertical::Center),
                                        ),
                                    )),
                            ),
                    ),
            );

        Container::new(all)
            .height(Length::Fill)
            .style(self.style)
            .into()
    }
}
