use iced_baseview::{
    button, renderer, Button, Color, Column, Container, Element, Font, HorizontalAlignment, Length,
    Point, Row, Rule, Space, Text, VerticalAlignment,
};
use iced_baseview::{executor, Align, Application, Command, Subscription, WindowSubs};

use crate::parameters::values::{MasterFrequencyValue, MasterVolumeValue};
use crate::{get_version_info, GuiSyncHandle};

mod boolean_picker;
mod envelope;
mod knob;
mod lfo;
mod lfo_target_picker;
mod mod_matrix;
mod operator;
mod preset_picker;
pub mod style;

use knob::OctaSineKnob;
use lfo::LfoWidgets;
use mod_matrix::ModulationMatrix;
use operator::OperatorWidgets;
use preset_picker::PresetPicker;
use style::Theme;

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
        "Copyright © 2019-2021 Joakim Frostegård\nSite: OctaSine.com. Build: {}",
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
    ParameterChange(usize, f64),
    ParameterChanges(Vec<(usize, f64)>),
    ToggleInfo,
    PresetChange(usize),
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

impl<H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(&mut self, parameter_index: usize, value: f64) {
        let v = value;

        match parameter_index {
            0 => self.master_volume.set_value(v),
            1 => self.master_frequency.set_value(v),
            2 => {
                self.operator_1.volume.set_value(v);
                self.modulation_matrix.set_operator_1_volume(value);
            }
            3 => self.operator_1.panning.set_value(v),
            4 => self.operator_1.wave_type.set_value(v),
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
            15 => {
                self.operator_2.volume.set_value(v);
                self.modulation_matrix.set_operator_2_volume(value);
            }
            16 => self.operator_2.panning.set_value(v),
            17 => self.operator_2.wave_type.set_value(v),
            18 => {
                self.operator_2.additive.as_mut().unwrap().set_value(v);
                self.modulation_matrix.set_operator_2_additive(v);
            }
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
            29 => {
                self.modulation_matrix.set_operator_3_volume(value);
                self.operator_3.volume.set_value(v);
            }
            30 => self.operator_3.panning.set_value(v),
            31 => self.operator_3.wave_type.set_value(v),
            32 => {
                self.operator_3.additive.as_mut().unwrap().set_value(v);
                self.modulation_matrix.set_operator_3_additive(v);
            }
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
            44 => {
                self.operator_4.volume.set_value(v);
                self.modulation_matrix.set_operator_4_volume(value);
            }
            45 => self.operator_4.panning.set_value(v),
            46 => self.operator_4.wave_type.set_value(v),
            47 => {
                self.operator_4.additive.as_mut().unwrap().set_value(v);
                self.modulation_matrix.set_operator_4_additive(v);
            }
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
        let preset_picker = PresetPicker::new(&sync_handle, style);

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
        window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        window_subs.on_frame = Some(Message::Frame);

        Subscription::none()
    }

    #[cfg(feature = "gui_wgpu")]
    fn renderer_settings() -> renderer::Settings {
        renderer::Settings {
            present_mode: iced_wgpu::wgpu::PresentMode::Immediate,
            default_font: Some(FONT_REGULAR),
            default_text_size: FONT_SIZE,
            antialiasing: Some(renderer::Antialiasing::MSAAx4),
            ..Default::default()
        }
    }

    #[cfg(feature = "gui_glow")]
    fn renderer_settings() -> (raw_gl_context::GlConfig, iced_glow::settings::Settings) {
        (
            raw_gl_context::GlConfig {
                samples: Some(8),
                ..Default::default()
            },
            iced_glow::settings::Settings {
                default_font: Some(FONT_REGULAR),
                default_text_size: FONT_SIZE,
                antialiasing: Some(renderer::settings::Antialiasing::MSAAx8),
            },
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Frame => {
                if self.sync_handle.have_presets_changed() {
                    self.preset_picker = PresetPicker::new(&self.sync_handle, self.style);
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
            Message::ParameterChange(index, value) => {
                self.set_value(index, value);

                self.sync_handle.set_parameter(index, value);
            }
            Message::ParameterChanges(changes) => {
                for (index, value) in changes {
                    self.set_value(index, value);
                    self.sync_handle.set_parameter(index, value);
                }
            }
            Message::PresetChange(index) => {
                self.sync_handle.set_preset_index(index);
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
                self.preset_picker.style = style;
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
            .color(self.style.heading_color())
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center);

        let info_text_color = if self.show_version {
            self.style.text_color()
        } else {
            Color::TRANSPARENT
        };

        let all = Column::new()
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .align_items(Align::Center)
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
                                            .vertical_alignment(VerticalAlignment::Center),
                                    ),
                            )
                            .height(Length::Units(LINE_HEIGHT * 4))
                            .padding(LINE_HEIGHT)
                            .align_y(Align::Center),
                        ),
                    )
                    .push(
                        Container::new(
                            Text::new("OctaSine")
                                .font(FONT_VERY_BOLD)
                                .color(self.style.heading_color())
                                .size(FONT_SIZE * 2 + FONT_SIZE / 2)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::FillPortion(4))
                        .align_x(Align::Center),
                    )
                    .push(
                        Column::new()
                            .width(Length::FillPortion(10))
                            .align_items(Align::End)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(
                                Row::new()
                                    .push(preset_picker)
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT))),
                            ),
                    ),
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
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
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(lfo_1)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                            .push(
                                Container::new(Rule::vertical(1).style(self.style))
                                    .align_x(Align::Center)
                                    .width(Length::Units(LINE_HEIGHT * 2))
                                    .height(Length::Units(LINE_HEIGHT * 17)),
                            ),
                    )
                    .push(lfo_2)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                            .push(
                                Container::new(Rule::vertical(1).style(self.style))
                                    .align_x(Align::Center)
                                    .width(Length::Units(LINE_HEIGHT * 2))
                                    .height(Length::Units(LINE_HEIGHT * 17)),
                            ),
                    )
                    .push(lfo_3)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                            .push(
                                Container::new(Rule::vertical(1).style(self.style))
                                    .align_x(Align::Center)
                                    .width(Length::Units(LINE_HEIGHT * 2))
                                    .height(Length::Units(LINE_HEIGHT * 17)),
                            ),
                    )
                    .push(lfo_4)
                    .push(
                        Column::new()
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                            .push(
                                Container::new(Rule::vertical(1).style(self.style))
                                    .align_x(Align::Center)
                                    .width(Length::Units(LINE_HEIGHT * 2))
                                    .height(Length::Units(LINE_HEIGHT * 17)),
                            ),
                    )
                    .push(
                        Column::new()
                            .width(Length::Units(LINE_HEIGHT * 8))
                            .push(
                                Row::new().push(
                                    Container::new(master_title)
                                        .width(Length::Units(LINE_HEIGHT * 8))
                                        .height(Length::Units(LINE_HEIGHT * 2))
                                        .align_x(Align::Center)
                                        .align_y(Align::Center),
                                ),
                            )
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                            .push(Row::new().push(master_volume).push(master_frequency))
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 4)))
                            .push(Row::new().push(modulation_matrix)),
                    ),
            );

        Container::new(all)
            .height(Length::Fill)
            .style(self.style)
            .into()
    }
}
