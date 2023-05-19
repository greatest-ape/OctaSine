use compact_str::CompactString;

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue, SerializableRepresentation,
};

const STEPS: &[PortamentoMode] = &[
    PortamentoMode::Off,
    PortamentoMode::Auto,
    PortamentoMode::Always,
];

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PortamentoMode {
    #[default]
    Off,
    Auto,
    Always,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PortamentoModeValue(PortamentoMode);

impl ParameterValue for PortamentoModeValue {
    type Value = PortamentoMode;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.trim().to_lowercase().as_str() {
            "off" => Some(Self(PortamentoMode::Off)),
            "auto" => Some(Self(PortamentoMode::Auto)),
            "always" => Some(Self(PortamentoMode::Always)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            PortamentoMode::Off => "Off".into(),
            PortamentoMode::Auto => "Auto".into(),
            PortamentoMode::Always => "Always".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
