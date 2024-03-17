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
mod value_text;
mod wave_display;
mod wave_picker;

use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use cfg_if::cfg_if;
use compact_str::CompactString;
use iced_aw::native::{Card, Modal};
use iced_baseview::alignment::Horizontal;
use iced_baseview::command::Action;
use iced_baseview::widget::{Button, PickList, Text};
use iced_baseview::{executor, window::WindowSubs, Application, Command, Subscription};
use iced_baseview::{
    widget::Column, widget::Container, widget::Row, widget::Space, window::WindowQueue, Element,
    Length, Point,
};
use serde::{Deserialize, Serialize};

use crate::common::NUM_OPERATORS;
use crate::parameters::*;
use crate::sync::GuiSyncHandle;

use lfo::LfoWidgets;
use operator::OperatorWidgets;
use patch_picker::PatchPicker;
use style::Theme;

use self::corner::CornerWidgets;
use self::operator::ModTargetPicker;
use self::style::container::ContainerStyle;

use crate::settings::Settings;

pub const GUI_WIDTH: usize = 12 * 82;
pub const GUI_HEIGHT: usize = 12 * 55;

const FONT_SIZE: u16 = 12;
const LINE_HEIGHT: u16 = 12;

const OPEN_SANS_BYTES_REGULAR: &[u8] =
    include_bytes!("../../../contrib/open-sans/OpenSans-Regular.ttf");
const OPEN_SANS_BYTES_SEMI_BOLD: &[u8] =
    include_bytes!("../../../contrib/open-sans/OpenSans-SemiBold.ttf");
const OPEN_SANS_BYTES_BOLD: &[u8] = include_bytes!("../../../contrib/open-sans/OpenSans-Bold.ttf");
const OPEN_SANS_BYTES_EXTRA_BOLD: &[u8] =
    include_bytes!("../../../contrib/open-sans/OpenSans-ExtraBold.ttf");

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

pub struct GuiSettings {
    pub theme: style::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    Frame,
    ChangeSingleParameterBegin(WrappedParameter),
    ChangeSingleParameterEnd(WrappedParameter),
    ChangeSingleParameterSetValue(WrappedParameter, f32),
    ChangeSingleParameterImmediate(WrappedParameter, f32),
    /// End envelope edit.
    ///
    /// Call host.begin_edit, host.automate and host.end_edit.
    ChangeEnvelopeParametersEnd {
        operator_index: u8,
        parameter_1: (WrappedParameter, f32),
        parameter_2: Option<(WrappedParameter, f32)>,
    },
    /// Set envelope parameters (but don't automate host for performance
    /// reasons). Broadcast all envelope values to group members.
    ///
    /// Remember to wrap calls with appropriate begin/end messages
    ChangeEnvelopeParametersSetValue {
        operator_index: u8,
        parameter_1: (WrappedParameter, f32),
        parameter_2: Option<(WrappedParameter, f32)>,
    },
    ChangePatch(usize),
    /// Set viewport, broadcast it to group members
    EnvelopeChangeViewport {
        operator_index: u8,
        viewport_factor: f32,
        x_offset: f32,
    },
    /// Distribute viewport to all envelopes
    EnvelopeDistributeViewports {
        viewport_factor: f32,
        x_offset: f32,
    },
    SwitchTheme,
    ToggleAlternativeControls,
    SavePatch,
    SaveBank,
    LoadBankOrPatch,
    RenamePatch,
    ClearPatch,
    ClearBank,
    SaveBankOrPatchToFile(PathBuf, Vec<u8>),
    LoadBankOrPatchesFromPaths(Vec<PathBuf>),
    ChangeParameterByTextInput {
        parameter: WrappedParameter,
        value_text: CompactString,
    },
    ModalOpen(ModalAction),
    ModalClose,
    ModalYes,
    /// Currently not used
    ModalSetParameterByChoicesUpdate(CompactString),
}

#[derive(Debug, Clone)]
pub enum ModalAction {
    ClearPatch,
    ClearBank,
    /// Currently not used
    SetParameterByChoices {
        parameter: WrappedParameter,
        options: Vec<CompactString>,
        choice: CompactString,
    },
}

pub struct OctaSineIcedApplication<H: GuiSyncHandle> {
    sync_handle: H,
    theme: style::Theme,
    operator_1: OperatorWidgets,
    operator_2: OperatorWidgets,
    operator_3: OperatorWidgets,
    operator_4: OperatorWidgets,
    lfo_1: LfoWidgets,
    lfo_2: LfoWidgets,
    lfo_3: LfoWidgets,
    lfo_4: LfoWidgets,
    corner: CornerWidgets,
    modal_action: Option<ModalAction>,
}

impl<H: GuiSyncHandle> OctaSineIcedApplication<H> {
    fn set_value(&mut self, parameter: Parameter, v: f32, internal: bool) {
        match parameter {
            Parameter::None => (),
            Parameter::Master(MasterParameter::Volume) => self.corner.master_volume.set_value(v),
            Parameter::Master(MasterParameter::Frequency) => {
                self.corner.master_frequency.set_value(v)
            }
            Parameter::Master(MasterParameter::PitchBendRangeUp) => {
                self.corner.master_pitch_bend_up.set_value(v)
            }
            Parameter::Master(MasterParameter::PitchBendRangeDown) => {
                self.corner.master_pitch_bend_down.set_value(v)
            }
            Parameter::Master(MasterParameter::VelocitySensitivityVolume) => {
                self.corner.volume_velocity_sensitivity.set_value(v)
            }
            Parameter::Master(MasterParameter::VoiceMode) => {
                self.corner.patch_picker.voice_mode_button.set_value(v)
            }
            Parameter::Master(MasterParameter::GlideActive) => {
                self.corner.glide_active = v;
            }
            Parameter::Master(MasterParameter::GlideTime) => self.corner.glide_time.set_value(v),
            Parameter::Master(MasterParameter::GlideBpmSync) => {
                self.corner.glide_bpm_sync.set_value(v)
            }
            Parameter::Master(MasterParameter::GlideMode) => self.corner.glide_mode.set_value(v),
            Parameter::Master(MasterParameter::GlideRetrigger) => {
                self.corner.glide_retrigger.set_value(v)
            }
            outer_p @ Parameter::Operator(index, p) => {
                self.operator_1.wave_display.set_value(outer_p, v);
                self.operator_2.wave_display.set_value(outer_p, v);
                self.operator_3.wave_display.set_value(outer_p, v);
                self.operator_4.wave_display.set_value(outer_p, v);

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
                    OperatorParameter::AttackDuration => {
                        operator.envelope.widget.set_attack_duration(v, internal);

                        if !internal {
                            self.update_envelope_group_statuses();
                        }
                    }
                    OperatorParameter::DecayDuration => {
                        operator.envelope.widget.set_decay_duration(v, internal);

                        if !internal {
                            self.update_envelope_group_statuses();
                        }
                    }
                    OperatorParameter::SustainVolume => {
                        operator.envelope.widget.set_sustain_volume(v, internal);

                        if !internal {
                            self.update_envelope_group_statuses();
                        }
                    }
                    OperatorParameter::ReleaseDuration => {
                        operator.envelope.widget.set_release_duration(v, internal);

                        if !internal {
                            self.update_envelope_group_statuses();
                        }
                    }
                    OperatorParameter::EnvelopeLockGroup => {
                        operator.envelope.set_group(v, internal);

                        // Group buttons don't send message triggering update by themselves
                        self.update_envelope_group_statuses();
                    }
                    OperatorParameter::VelocitySensitivityModOut => {
                        operator.mod_out_velocity_sensitivity.set_value(v)
                    }
                    OperatorParameter::VelocitySensitivityFeedback => {
                        operator.feedback_velocity_sensitivity.set_value(v)
                    }
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
                    LfoParameter::KeySync => lfo.key_sync.set_value(v),
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
                        self.set_value(parameter, *new_value, false);
                    }
                }
            }
        }
    }

    fn save_settings(&self) {
        let settings = Settings {
            schema_version: 1,
            gui: GuiSettings { theme: self.theme },
        };

        if let Err(err) = settings.save() {
            ::log::error!("Couldn't save settings: {:#}", err)
        }
    }

    fn get_envelope_by_index(&mut self, operator_index: u8) -> &mut envelope::Envelope {
        match operator_index {
            0 => &mut self.operator_1.envelope,
            1 => &mut self.operator_2.envelope,
            2 => &mut self.operator_3.envelope,
            3 => &mut self.operator_4.envelope,
            _ => unreachable!(),
        }
    }

    /// Broadcast envelope changes to other group members, and optionally to host
    fn sync_envelopes(&mut self, sending_operator_index: u8, automate_host: bool) {
        let sending_envelope = self.get_envelope_by_index(sending_operator_index);

        let group = sending_envelope.get_group();
        let values = sending_envelope.widget.get_envelope_values();

        for index in 0..NUM_OPERATORS {
            let envelope = self.get_envelope_by_index(index as u8);

            if !envelope.is_group_member(group) || index == sending_operator_index as usize {
                continue;
            }

            envelope
                .widget
                .set_viewport(values.viewport_factor, values.x_offset);

            let parameters: [(WrappedParameter, f32); 4] = [
                (
                    Parameter::Operator(index as u8, OperatorParameter::AttackDuration).into(),
                    values.attack,
                ),
                (
                    Parameter::Operator(index as u8, OperatorParameter::DecayDuration).into(),
                    values.decay,
                ),
                (
                    Parameter::Operator(index as u8, OperatorParameter::SustainVolume).into(),
                    values.sustain,
                ),
                (
                    Parameter::Operator(index as u8, OperatorParameter::ReleaseDuration).into(),
                    values.release,
                ),
            ];

            for (p, v) in parameters {
                self.set_value(p.parameter(), v, true);

                if automate_host {
                    self.sync_handle.begin_edit(p);
                    self.sync_handle.set_parameter(p, v);
                    self.sync_handle.end_edit(p);
                } else {
                    self.sync_handle.set_parameter_audio_only(p, v);
                }
            }
        }

        self.update_envelope_group_statuses();
    }

    fn update_envelope_group_statuses(&mut self) {
        for group in [OperatorEnvelopeGroupValue::A, OperatorEnvelopeGroupValue::B] {
            let mut any_modified_by_automation = false;

            for i in 0..NUM_OPERATORS {
                let envelope = self.get_envelope_by_index(i as u8);

                if envelope.is_group_member(group) {
                    any_modified_by_automation |= envelope.widget.get_modified_by_automation();
                }
            }

            let mut opt_values = None;
            let mut group_synced = true;

            for i in 0..NUM_OPERATORS {
                let envelope = self.get_envelope_by_index(i as u8);

                if envelope.is_group_member(group) {
                    let values = envelope.widget.get_envelope_values();

                    match &mut opt_values {
                        Some(previous_values) => {
                            if any_modified_by_automation && values != *previous_values {
                                group_synced = false;

                                break;
                            }
                        }
                        opt_values @ None => *opt_values = Some(values),
                    }
                }
            }

            for i in 0..NUM_OPERATORS {
                let envelope = self.get_envelope_by_index(i as u8);

                if envelope.is_group_member(group) {
                    envelope.set_group_synced(group_synced);
                }
            }
        }

        for i in 0..NUM_OPERATORS {
            let envelope = self.get_envelope_by_index(i as u8);

            if let OperatorEnvelopeGroupValue::Off = envelope.get_group() {
                envelope.set_group_synced(true);
            }
        }
    }
}

impl<H: GuiSyncHandle> Application for OctaSineIcedApplication<H> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = H;
    type Theme = Theme;

    fn new(sync_handle: Self::Flags) -> (Self, Command<Self::Message>) {
        let style = sync_handle.get_gui_settings().theme;

        let operator_1 = OperatorWidgets::new(&sync_handle, 0);
        let operator_2 = OperatorWidgets::new(&sync_handle, 1);
        let operator_3 = OperatorWidgets::new(&sync_handle, 2);
        let operator_4 = OperatorWidgets::new(&sync_handle, 3);

        let lfo_1 = LfoWidgets::new(&sync_handle, 0);
        let lfo_2 = LfoWidgets::new(&sync_handle, 1);
        let lfo_3 = LfoWidgets::new(&sync_handle, 2);
        let lfo_4 = LfoWidgets::new(&sync_handle, 3);

        let corner = CornerWidgets::new(&sync_handle);

        let app = Self {
            sync_handle,
            theme: style,
            operator_1,
            operator_2,
            operator_3,
            operator_4,
            lfo_1,
            lfo_2,
            lfo_3,
            lfo_4,
            corner,
            modal_action: None,
        };

        (app, Command::none())
    }

    fn subscription(
        &self,
        window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        window_subs.on_frame = Some(|| Message::Frame);

        Subscription::none()
    }

    #[cfg(feature = "wgpu")]
    fn renderer_settings() -> iced_baseview::renderer::Settings {
        iced_baseview::renderer::Settings {
            default_font: Some(OPEN_SANS_BYTES_SEMI_BOLD),
            default_text_size: FONT_SIZE.into(),
            antialiasing: Some(iced_baseview::renderer::settings::Antialiasing::MSAAx4),
            ..Default::default()
        }
    }

    /// Renderer settings with glow
    #[cfg(feature = "glow")]
    fn renderer_settings() -> iced_baseview::renderer::Settings {
        iced_baseview::renderer::Settings {
            default_font: Some(OPEN_SANS_BYTES_SEMI_BOLD),
            default_text_size: FONT_SIZE.into(),
            #[cfg(target_os = "linux")]
            antialiasing: Some(iced_baseview::renderer::settings::Antialiasing::MSAAx4),
            #[cfg(not(target_os = "linux"))]
            antialiasing: Some(iced_baseview::renderer::settings::Antialiasing::MSAAx8),
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
                    self.corner.patch_picker = PatchPicker::new(&self.sync_handle);
                }
                self.update_widgets_from_parameters();
            }
            Message::NoOp => {}
            Message::EnvelopeChangeViewport {
                operator_index,
                viewport_factor,
                x_offset,
            } => {
                self.get_envelope_by_index(operator_index)
                    .widget
                    .set_viewport(viewport_factor, x_offset);

                self.sync_envelopes(operator_index, false);
            }
            Message::EnvelopeDistributeViewports {
                viewport_factor,
                x_offset,
            } => {
                for operator_index in 0..NUM_OPERATORS {
                    self.get_envelope_by_index(operator_index as u8)
                        .widget
                        .set_viewport(viewport_factor, x_offset);
                }
            }
            Message::ChangeSingleParameterBegin(parameter) => {
                self.sync_handle.begin_edit(parameter);
            }
            Message::ChangeSingleParameterEnd(parameter) => {
                self.sync_handle.end_edit(parameter);
            }
            Message::ChangeSingleParameterSetValue(parameter, value) => {
                self.set_value(parameter.parameter(), value, true);

                self.sync_handle.set_parameter(parameter, value);
            }
            Message::ChangeSingleParameterImmediate(parameter, value) => {
                self.set_value(parameter.parameter(), value, true);

                self.sync_handle.set_parameter_immediate(parameter, value);
            }
            Message::ChangeEnvelopeParametersEnd {
                operator_index,
                parameter_1,
                parameter_2,
            } => {
                self.set_value(parameter_1.0.parameter(), parameter_1.1, true);

                self.sync_handle
                    .set_parameter_immediate(parameter_1.0, parameter_1.1);

                if let Some((p, v)) = parameter_2 {
                    self.set_value(p.parameter(), v, true);

                    self.sync_handle.set_parameter_immediate(p, v);
                }

                self.sync_envelopes(operator_index, true);
            }
            Message::ChangeEnvelopeParametersSetValue {
                operator_index,
                parameter_1,
                parameter_2,
            } => {
                self.set_value(parameter_1.0.parameter(), parameter_1.1, true);

                self.sync_handle
                    .set_parameter_audio_only(parameter_1.0, parameter_1.1);

                if let Some((p, v)) = parameter_2 {
                    self.set_value(p.parameter(), v, true);

                    self.sync_handle.set_parameter_audio_only(p, v);
                }

                self.sync_envelopes(operator_index, false);
            }
            Message::ChangePatch(index) => {
                self.sync_handle.set_patch_index(index);
            }
            Message::SwitchTheme => {
                let style = if let Theme::Light = self.theme {
                    Theme::Dark
                } else {
                    Theme::Light
                };

                self.theme = style;
                self.corner.theme_changed();
                self.lfo_1.theme_changed();
                self.lfo_2.theme_changed();
                self.lfo_3.theme_changed();
                self.lfo_4.theme_changed();
                self.operator_1.theme_changed();
                self.operator_2.theme_changed();
                self.operator_3.theme_changed();
                self.operator_4.theme_changed();

                self.save_settings();
            }
            Message::ToggleAlternativeControls => {
                for operator in [
                    &mut self.operator_1,
                    &mut self.operator_2,
                    &mut self.operator_3,
                    &mut self.operator_4,
                ] {
                    operator.alternative_controls = !operator.alternative_controls;
                }

                self.corner.alternative_controls = !self.corner.alternative_controls;
            }
            Message::LoadBankOrPatch => {
                const TITLE: &str = "Load OctaSine patch bank or patches";

                return Command::single(Action::Future(Box::pin(async move {
                    cfg_if!(
                        if #[cfg(target_os = "macos")] {
                            let mut builder = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch", &["fxp"])
                                .add_filter("Patch bank", &["fxb"]);

                            if let Some(h) = CurrentWindowHandle::get() {
                                builder = builder.set_parent(&h);
                            }

                            let opt_paths = builder
                                .pick_files()
                                .await
                                .map(|handles|
                                    handles.into_iter()
                                        .map(|h| h.path().to_owned())
                                        .collect::<Vec<PathBuf>>()
                                );
                        } else if #[cfg(target_os = "windows")] {
                            let opt_paths = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch", &["fxp"])
                                .add_filter("Patch bank", &["fxb"])
                                .pick_files()
                                .await
                                .map(|handles|
                                    handles.into_iter()
                                        .map(|h| h.path().to_owned())
                                        .collect::<Vec<PathBuf>>()
                                );
                        } else {
                            let opt_paths = tinyfiledialogs::open_file_dialog_multi(
                                TITLE,
                                "",
                                Some((&["*.fxp", "*.fxb"], "Patch bank or patch files"))
                            ).map(|strings|
                                strings.into_iter()
                                    .map(|s| s.into())
                                    .collect::<Vec<PathBuf>>()
                            );
                        }
                    );

                    if let Some(paths) = opt_paths {
                        Message::LoadBankOrPatchesFromPaths(paths)
                    } else {
                        Message::NoOp
                    }
                })));
            }
            Message::SavePatch => {
                const TITLE: &str = "Save OctaSine patch";

                let (patch_filename, patch_bytes) = self.sync_handle.export_patch();

                return Command::single(Action::Future(Box::pin(async move {
                    cfg_if!(
                        if #[cfg(target_os = "macos")] {
                            let mut builder = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch", &["fxp"])
                                .set_file_name(&patch_filename);

                            if let Some(h) = CurrentWindowHandle::get() {
                                builder = builder.set_parent(&h);
                            }

                            let opt_path_buf = builder
                                .save_file()
                                .await
                                .map(|handle| handle.path().to_owned());
                        }
                        else if #[cfg(target_os = "windows")] {
                            let opt_path_buf = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch", &["fxp"])
                                .set_file_name(&patch_filename)
                                .save_file()
                                .await
                                .map(|handle| handle.path().to_owned());
                        } else {
                            let opt_path_buf = tinyfiledialogs::save_file_dialog_with_filter(
                                TITLE,
                                &patch_filename,
                                &["*.fxp"],
                                "Patch"
                            ).map(|s| s.into());
                        }
                    );

                    if let Some(path_buf) = opt_path_buf {
                        Message::SaveBankOrPatchToFile(path_buf, patch_bytes)
                    } else {
                        Message::NoOp
                    }
                })));
            }
            Message::SaveBank => {
                const TITLE: &str = "Save OctaSine bank";
                const FILENAME: &str = "OctaSine bank.fxb";

                let bank_bytes = self.sync_handle.export_bank();

                return Command::single(Action::Future(Box::pin(async move {
                    cfg_if!(
                        if #[cfg(target_os = "macos")] {
                            let mut builder = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch bank", &["fxb"])
                                .set_file_name(FILENAME);

                            if let Some(h) = CurrentWindowHandle::get() {
                                builder = builder.set_parent(&h);
                            }

                            let opt_path_buf = builder
                                .save_file()
                                .await
                                .map(|handle| handle.path().to_owned());
                        } else if #[cfg(target_os = "windows")] {
                            let opt_path_buf = rfd::AsyncFileDialog::new()
                                .set_title(TITLE)
                                .add_filter("Patch bank", &["fxb"])
                                .set_file_name(FILENAME)
                                .save_file()
                                .await
                                .map(|handle| handle.path().to_owned());
                        } else  {
                            let opt_path_buf = tinyfiledialogs::save_file_dialog_with_filter(
                                TITLE,
                                FILENAME,
                                &["*.fxb"],
                                ""
                            ).map(|s| s.into());
                        }
                    );

                    if let Some(path_buf) = opt_path_buf {
                        Message::SaveBankOrPatchToFile(path_buf, bank_bytes)
                    } else {
                        Message::NoOp
                    }
                })));
            }
            Message::RenamePatch => {
                if let Some(name) = tinyfiledialogs::input_box(
                    "Change OctaSine patch name",
                    "Please provide a new name for this patch",
                    &self.sync_handle.get_current_patch_name(),
                ) {
                    self.sync_handle.set_current_patch_name(&name);
                }
            }
            Message::ClearPatch => {
                self.modal_action = Some(ModalAction::ClearPatch);
            }
            Message::ClearBank => {
                self.modal_action = Some(ModalAction::ClearBank);
            }
            Message::SaveBankOrPatchToFile(path_buf, bytes) => {
                if let Err(err) = save_data_to_file(path_buf, bytes) {
                    ::log::error!("Error saving patch/patch bank to file: {:#}", err)
                }
            }
            Message::LoadBankOrPatchesFromPaths(paths) => {
                self.sync_handle.import_bank_or_patches_from_paths(&paths);
            }
            Message::ChangeParameterByTextInput {
                parameter,
                value_text,
            } => {
                if let Some(new_text_value) = tinyfiledialogs::input_box(
                    "Change OctaSine parameter value",
                    &format!(
                        "Please provide a new value for {}",
                        parameter.parameter().name()
                    ),
                    &value_text,
                ) {
                    if let Some(value_patch) = self
                        .sync_handle
                        .parse_parameter_from_text(parameter, &new_text_value)
                    {
                        self.sync_handle
                            .set_parameter_immediate(parameter, value_patch);
                        self.set_value(parameter.parameter(), value_patch, true);
                    }
                }
            }
            Message::ModalOpen(action) => {
                self.modal_action = Some(action);
            }
            Message::ModalClose => {
                self.modal_action = None;
            }
            Message::ModalYes => match self.modal_action.take() {
                Some(ModalAction::ClearBank) => {
                    self.sync_handle.clear_bank();
                }
                Some(ModalAction::ClearPatch) => {
                    self.sync_handle.clear_patch();
                }
                Some(ModalAction::SetParameterByChoices {
                    parameter, choice, ..
                }) => {
                    if let Some(value_patch) = self
                        .sync_handle
                        .parse_parameter_from_text(parameter, choice.as_str())
                    {
                        self.sync_handle
                            .set_parameter_immediate(parameter, value_patch);

                        self.set_value(parameter.parameter(), value_patch, true);
                    }
                }
                None => (),
            },
            Message::ModalSetParameterByChoicesUpdate(new_choice) => {
                if let Some(ModalAction::SetParameterByChoices { choice, .. }) =
                    self.modal_action.as_mut()
                {
                    *choice = new_choice.into();
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme> {
        let content = Container::new(
            Column::new()
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(self.operator_4.view(&self.theme))
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(self.operator_3.view(&self.theme))
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(self.operator_2.view(&self.theme))
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(self.operator_1.view(&self.theme))
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(
                    Row::new()
                        .push(
                            Column::new()
                                .push(self.lfo_4.view(&self.theme))
                                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                                .push(self.lfo_3.view(&self.theme)),
                        )
                        .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                        .push(
                            Column::new()
                                .push(self.lfo_2.view(&self.theme))
                                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                                .push(self.lfo_1.view(&self.theme)),
                        )
                        .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                        .push(self.corner.view(&self.theme)),
                ),
        )
        .height(Length::Fill)
        .style(ContainerStyle::L0);

        Modal::new(self.modal_action.is_some(), content, || {
            let modal_action = if let Some(modal_action) = self.modal_action.as_ref() {
                modal_action
            } else {
                return Row::new().into();
            };

            let heading = match modal_action {
                ModalAction::ClearBank => "CLEAR ENTIRE PATCH BANK?".into(),
                ModalAction::ClearPatch => "CLEAR CURRENT PATCH?".into(),
                ModalAction::SetParameterByChoices { parameter, .. } => {
                    format!("SET {}", parameter.parameter().name().to_uppercase())
                }
            };

            match modal_action {
                ModalAction::ClearBank | ModalAction::ClearPatch => {
                    let body = Row::new()
                        .spacing(LINE_HEIGHT / 2)
                        .width(Length::Fill)
                        .push(
                            Button::new(Text::new("YES").horizontal_alignment(Horizontal::Center))
                                .width(Length::Fill)
                                .on_press(Message::ModalYes),
                        )
                        .push(
                            Button::new(Text::new("NO").horizontal_alignment(Horizontal::Center))
                                .width(Length::Fill)
                                .on_press(Message::ModalClose),
                        );

                    Card::new(Text::new(heading), body)
                        .max_width(LINE_HEIGHT as f32 * 16.0)
                        .padding(LINE_HEIGHT as f32)
                        .into()
                }
                ModalAction::SetParameterByChoices {
                    options, choice, ..
                } => {
                    let body = Column::new()
                        .spacing(LINE_HEIGHT)
                        .push(
                            PickList::new(options, Some(choice.clone()), |choice| {
                                Message::ModalSetParameterByChoicesUpdate(choice)
                            })
                            .width(Length::Fill),
                        )
                        .push(
                            Row::new()
                                .spacing(LINE_HEIGHT / 2)
                                .width(Length::Fill)
                                .push(
                                    Button::new(
                                        Text::new("OK").horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(Message::ModalYes),
                                )
                                .push(
                                    Button::new(
                                        Text::new("CANCEL")
                                            .horizontal_alignment(Horizontal::Center),
                                    )
                                    .width(Length::Fill)
                                    .on_press(Message::ModalClose),
                                ),
                        );

                    Card::new(Text::new(heading), body)
                        .max_width(LINE_HEIGHT as f32 * 16.0)
                        .padding(LINE_HEIGHT as f32)
                        .into()
                }
            }
        })
        .backdrop(Message::ModalClose)
        .on_esc(Message::ModalClose)
        .into()
    }

    fn title(&self) -> String {
        crate::plugin::common::PLUGIN_SEMVER_NAME.into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme
    }
}

fn save_data_to_file(path_buf: PathBuf, mut bytes: Vec<u8>) -> anyhow::Result<()> {
    let mut file = ::std::fs::File::create(&path_buf)
        .with_context(|| format!("create file {}", path_buf.display()))?;

    file.write_all(&mut bytes)
        .with_context(|| format!("write to file {}", path_buf.display()))?;

    Ok(())
}

pub fn get_iced_baseview_settings<H: GuiSyncHandle>(
    sync_handle: H,
    plugin_name: String,
) -> iced_baseview::Settings<H> {
    iced_baseview::Settings {
        window: iced_baseview::baseview::WindowOpenOptions {
            size: iced_baseview::baseview::Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
            #[cfg(not(target_os = "windows"))]
            scale: iced_baseview::baseview::WindowScalePolicy::SystemScaleFactor,
            // Windows currently needs scale factor 1.0, or GUI contents
            // will be too large for window
            #[cfg(target_os = "windows")]
            scale: iced_baseview::baseview::WindowScalePolicy::ScaleFactor(1.0),
            title: plugin_name,
            #[cfg(feature = "glow")]
            gl_config: Some(iced_baseview::baseview::gl::GlConfig {
                // 4x MSAA on Linux solves crashes for some people
                #[cfg(target_os = "linux")]
                samples: Some(4),
                #[cfg(not(target_os = "linux"))]
                samples: Some(8),
                ..Default::default()
            }),
        },
        iced_baseview: iced_baseview::settings::IcedBaseviewSettings {
            ignore_non_modifier_keys: true,
            always_redraw: true,
        },
        flags: sync_handle,
    }
}

#[cfg(target_os = "macos")]
struct CurrentWindowHandle(raw_window_handle::RawWindowHandle);

#[cfg(target_os = "macos")]
impl CurrentWindowHandle {
    fn get() -> Option<Self> {
        use objc::{class, msg_send, runtime::Object, sel, sel_impl};

        unsafe {
            let ns_app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            if ns_app.is_null() {
                return None;
            }

            let ns_window: *mut Object = msg_send![ns_app, keyWindow];
            if ns_window.is_null() {
                return None;
            }

            let ns_view: *mut Object = msg_send![ns_window, contentView];
            if ns_view.is_null() {
                return None;
            }

            let mut handle = raw_window_handle::AppKitWindowHandle::empty();

            handle.ns_window = ns_window as *mut core::ffi::c_void;
            handle.ns_view = ns_view as *mut core::ffi::c_void;

            Some(Self(raw_window_handle::RawWindowHandle::AppKit(handle)))
        }
    }
}

#[cfg(target_os = "macos")]
unsafe impl raw_window_handle::HasRawWindowHandle for CurrentWindowHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0
    }
}
