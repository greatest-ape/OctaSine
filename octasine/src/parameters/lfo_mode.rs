use compact_str::CompactString;

use super::utils::*;
use super::ParameterValue;
use super::SerializableRepresentation;

const LFO_MODE_STEPS: [LfoMode; 2] = [LfoMode::Forever, LfoMode::Once];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum LfoMode {
    Once,
    #[default]
    Forever,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LfoModeValue(pub LfoMode);

impl ParameterValue for LfoModeValue {
    type Value = LfoMode;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.to_lowercase().as_ref() {
            "once" => Some(Self(LfoMode::Once)),
            "forever" => Some(Self(LfoMode::Forever)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(&LFO_MODE_STEPS[..], value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(&LFO_MODE_STEPS[..], self.0)
    }
    fn get_formatted(self) -> CompactString {
        match self.0 {
            LfoMode::Once => "ONCE".into(),
            LfoMode::Forever => "LOOP".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
