use compact_str::{format_compact, CompactString};

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue, SerializableRepresentation,
};

pub const PORTAMENTO_MODE_STEPS: &[GlideActive] =
    &[GlideActive::Off, GlideActive::Auto, GlideActive::On];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlideActive {
    #[default]
    Off,
    Auto,
    On,
}

impl ::std::fmt::Display for GlideActive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Off => "OFF",
            Self::Auto => "AUTO",
            Self::On => "ON",
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GlideActiveValue(GlideActive);

impl ParameterValue for GlideActiveValue {
    type Value = GlideActive;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.trim().to_lowercase().as_str() {
            "off" => Some(Self(GlideActive::Off)),
            "auto" => Some(Self(GlideActive::Auto)),
            "always" => Some(Self(GlideActive::On)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&PORTAMENTO_MODE_STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&PORTAMENTO_MODE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
