use compact_str::{format_compact, CompactString};

use super::{
    utils::{map_audio_to_patch_value_with_steps, map_patch_to_audio_value_with_steps},
    ParameterValue, SerializableRepresentation,
};

const STEPS: &[f32] = &[0.0, 4.0];

#[derive(Debug, Clone, Copy)]
pub struct GlideTimeValue(f32);

impl Default for GlideTimeValue {
    fn default() -> Self {
        Self(0.1)
    }
}

impl ParameterValue for GlideTimeValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        text.parse::<Self::Value>()
            .ok()
            .map(|time| Self(time.clamp(0.0, *STEPS.last().unwrap())))
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_to_audio_value_with_steps(&STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_audio_to_patch_value_with_steps(&STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{:.4}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0 as f64)
    }
}
