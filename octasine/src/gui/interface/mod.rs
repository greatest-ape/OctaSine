mod boolean_button;
mod common;
mod corner;
mod envelope;
mod knob;
mod lfo;
mod lfo_target_picker;
mod mod_matrix;
mod mod_target_picker;
mod operator;
mod patch_picker;
pub mod style;
mod wave_picker;

use iced_baseview::{executor, Application, Command, Subscription, WindowSubs};
use iced_baseview::{Column, Container, Element, Length, Point, Row, Space, WindowQueue};

use crate::parameter_values::*;
use crate::sync::GuiSyncHandle;

use lfo::LfoWidgets;
use operator::OperatorWidgets;
use patch_picker::PatchPicker;
use style::Theme;

use self::corner::CornerWidgets;
use self::operator::ModTargetPicker;

use super::GuiSettings;
use crate::settings::Settings;

pub const FONT_SIZE: u16 = 12;
pub const LINE_HEIGHT: u16 = 12;

const OPEN_SANS_BYTES_REGULAR: &[u8] =
    include_bytes!("../../../../contrib/open-sans/OpenSans-Regular.ttf");
const OPEN_SANS_BYTES_SEMI_BOLD: &[u8] =
    include_bytes!("../../../../contrib/open-sans/OpenSans-SemiBold.ttf");
const OPEN_SANS_BYTES_BOLD: &[u8] =
    include_bytes!("../../../../contrib/open-sans/OpenSans-Bold.ttf");

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
    ChangeSingleParameterBegin(Parameter),
    ChangeSingleParameterEnd(Parameter),
    ChangeSingleParameterSetValue(Parameter, f64),
    ChangeSingleParameterImmediate(Parameter, f64),
    ChangeTwoParametersBegin((Parameter, Parameter)),
    ChangeTwoParametersEnd((Parameter, Parameter)),
    ChangeTwoParametersSetValues((Parameter, f64), (Parameter, f64)),
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
    show_version: bool,
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
    lfo_1: LfoWidgets,
    lfo_2: LfoWidgets,
    lfo_3: LfoWidgets,
    lfo_4: LfoWidgets,
    corner: CornerWidgets,
}

impl<H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(&mut self, parameter: Parameter, v: f64) {
        match parameter {
            Parameter::None => (),
            Parameter::Master(MasterParameter::Volume) => self.corner.master_volume.set_value(v),
            Parameter::Master(MasterParameter::Frequency) => {
                self.corner.master_frequency.set_value(v)
            }
            Parameter::Operator(index, p) => {
                let operator = match index {
                    0 => &mut self.operator_1,
                    1 => &mut self.operator_2,
                    2 => &mut self.operator_3,
                    3 => &mut self.operator_4,
                    _ => panic!("No such operator"),
                };

                match p {
                    OperatorParameter::Active => operator.mute_button.set_value(v),
                    OperatorParameter::WaveType => operator.wave_type.set_value(v),
                    OperatorParameter::Volume => operator.volume.set_value(v),
                    OperatorParameter::Panning => operator.panning.set_value(v),
                    OperatorParameter::MixOut => {
                        operator.mix.set_value(v);

                        match index {
                            0 => self.corner.modulation_matrix.set_operator_1_mix(v),
                            1 => self.corner.modulation_matrix.set_operator_2_mix(v),
                            2 => self.corner.modulation_matrix.set_operator_3_mix(v),
                            3 => self.corner.modulation_matrix.set_operator_4_mix(v),
                            _ => (),
                        }
                    }
                    OperatorParameter::ModOut => {
                        if let Some(mod_index) = operator.mod_index.as_mut() {
                            mod_index.set_value(v)
                        }

                        match index {
                            1 => self.corner.modulation_matrix.set_operator_2_mod(v),
                            2 => self.corner.modulation_matrix.set_operator_3_mod(v),
                            3 => self.corner.modulation_matrix.set_operator_4_mod(v),
                            _ => (),
                        }
                    }
                    OperatorParameter::ModTargets => {
                        match operator.mod_target.as_mut() {
                            Some(ModTargetPicker::Operator2(p)) => p.set_value(v),
                            Some(ModTargetPicker::Operator3(p)) => p.set_value(v),
                            Some(ModTargetPicker::Operator4(p)) => p.set_value(v),
                            _ => (),
                        }
                        match index {
                            1 => self.corner.modulation_matrix.set_operator_2_target(v),
                            2 => self.corner.modulation_matrix.set_operator_3_target(v),
                            3 => self.corner.modulation_matrix.set_operator_4_target(v),
                            _ => (),
                        }
                    }
                    OperatorParameter::Feedback => operator.feedback.set_value(v),
                    OperatorParameter::FrequencyRatio => operator.frequency_ratio.set_value(v),
                    OperatorParameter::FrequencyFree => operator.frequency_free.set_value(v),
                    OperatorParameter::FrequencyFine => operator.frequency_fine.set_value(v),
                    OperatorParameter::AttackDuration => operator.envelope.set_attack_duration(v),
                    OperatorParameter::AttackValue => operator.envelope.set_attack_end_value(v),
                    OperatorParameter::DecayDuration => operator.envelope.set_decay_duration(v),
                    OperatorParameter::DecayValue => operator.envelope.set_decay_end_value(v),
                    OperatorParameter::ReleaseDuration => operator.envelope.set_release_duration(v),
                }
            }
            Parameter::Lfo(index, p) => {
                let lfo = match index {
                    0 => &mut self.lfo_1,
                    1 => &mut self.lfo_2,
                    2 => &mut self.lfo_3,
                    3 => &mut self.lfo_4,
                    _ => panic!("No such LFO"),
                };

                match p {
                    LfoParameter::Target => lfo.target.set_value(v),
                    LfoParameter::BpmSync => lfo.bpm_sync.set_value(v),
                    LfoParameter::FrequencyRatio => lfo.frequency_ratio.set_value(v),
                    LfoParameter::FrequencyFree => lfo.frequency_free.set_value(v),
                    LfoParameter::Mode => lfo.mode.set_value(v),
                    LfoParameter::Shape => lfo.shape.set_value(v),
                    LfoParameter::Amount => lfo.amount.set_value(v),
                    LfoParameter::Active => lfo.active.set_value(v),
                }
            }
        }
    }

    fn update_widgets_from_parameters(&mut self) {
        let opt_changes = self.sync_handle.get_changed_parameters();

        if let Some(changes) = opt_changes {
            for (index, opt_new_value) in changes.iter().enumerate() {
                if let Some(new_value) = opt_new_value {
                    if let Some(parameter) = Parameter::from_index(index) {
                        self.set_value(parameter, *new_value);
                    }
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

        let operator_1 = OperatorWidgets::new(&sync_handle, 0, style);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1, style);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2, style);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3, style);

        let lfo_1 = LfoWidgets::new(&sync_handle, 0, style);
        let lfo_2 = LfoWidgets::new(&sync_handle, 1, style);
        let lfo_3 = LfoWidgets::new(&sync_handle, 2, style);
        let lfo_4 = LfoWidgets::new(&sync_handle, 3, style);

        let corner = CornerWidgets::new(&sync_handle);

        let app = Self {
            sync_handle,
            style,
            show_version: false,
            operator_1,
            operator_2,
            operator_3,
            operator_4,
            lfo_1,
            lfo_2,
            lfo_3,
            lfo_4,
            corner,
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
    fn renderer_settings() -> iced_baseview::backend::Settings {
        iced_baseview::backend::Settings {
            present_mode: iced_baseview::backend::wgpu::PresentMode::Immediate,
            default_font: Some(OPEN_SANS_BYTES_SEMI_BOLD),
            default_text_size: FONT_SIZE,
            antialiasing: Some(iced_baseview::backend::settings::Antialiasing::MSAAx8),
            ..Default::default()
        }
    }

    /// Renderer settings with glow
    #[cfg(feature = "gui_glow")]
    fn renderer_settings() -> iced_baseview::backend::Settings {
        iced_baseview::backend::Settings {
            default_font: Some(OPEN_SANS_BYTES_SEMI_BOLD),
            default_text_size: FONT_SIZE,
            antialiasing: Some(iced_baseview::backend::settings::Antialiasing::MSAAx8),
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
                    self.corner.patch_picker = PatchPicker::new(&self.sync_handle, self.style);
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
            Message::ChangeSingleParameterSetValue(parameter, value) => {
                self.set_value(parameter, value);

                self.sync_handle.set_parameter(parameter, value);
            }
            Message::ChangeSingleParameterImmediate(parameter, value) => {
                self.set_value(parameter, value);

                self.sync_handle.begin_edit(parameter);
                self.sync_handle.set_parameter(parameter, value);
                self.sync_handle.end_edit(parameter);
            }
            Message::ChangeTwoParametersBegin((parameter_1, parameter_2)) => {
                self.sync_handle.begin_edit(parameter_1);
                self.sync_handle.begin_edit(parameter_2);
            }
            Message::ChangeTwoParametersEnd((parameter_1, parameter_2)) => {
                self.sync_handle.end_edit(parameter_1);
                self.sync_handle.end_edit(parameter_2);
            }
            Message::ChangeTwoParametersSetValues(
                (parameter_1, value_1),
                (parameter_2, value_2),
            ) => {
                self.set_value(parameter_1, value_1);
                self.set_value(parameter_2, value_2);

                self.sync_handle.set_parameter(parameter_1, value_1);
                self.sync_handle.set_parameter(parameter_2, value_2);
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
                self.corner.set_style(style);
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
        Container::new(
            Column::new()
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                .push(self.operator_4.view())
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                .push(self.operator_3.view())
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                .push(self.operator_2.view())
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                .push(self.operator_1.view())
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                .push(
                    Row::new()
                        .push(
                            Column::new()
                                .push(self.lfo_1.view())
                                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                                .push(self.lfo_2.view()),
                        )
                        .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                        .push(
                            Column::new()
                                .push(self.lfo_3.view())
                                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                                .push(self.lfo_4.view()),
                        )
                        .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                        .push(self.corner.view()),
                ),
        )
        .height(Length::Fill)
        .style(self.style.container_l0())
        .into()
    }
}
