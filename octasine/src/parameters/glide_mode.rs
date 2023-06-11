use compact_str::{format_compact, CompactString};

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue, SerializableRepresentation,
};

pub const GLIDE_MODE_STEPS: &[GlideMode] = &[GlideMode::Lct, GlideMode::Lcr];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlideMode {
    #[default]
    Lct,
    Lcr,
}

impl ::std::fmt::Display for GlideMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Lct => "LCT",
            Self::Lcr => "LCR",
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GlideModeValue(GlideMode);

impl ParameterValue for GlideModeValue {
    type Value = GlideMode;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.trim().to_lowercase().as_str() {
            "lct" => Some(Self(GlideMode::Lct)),
            "lcr" => Some(Self(GlideMode::Lcr)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&GLIDE_MODE_STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&GLIDE_MODE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
