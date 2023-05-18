use compact_str::{format_compact, CompactString};

use super::{
    utils::{map_patch_value_to_step, map_step_to_patch_value},
    ParameterValue, SerializableRepresentation,
};

const STEPS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 16, 128];

#[derive(Debug, Clone, Copy)]
pub struct NumVoicesValue(u8);

impl Default for NumVoicesValue {
    fn default() -> Self {
        Self(128)
    }
}

impl ParameterValue for NumVoicesValue {
    type Value = u8;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        let value: u8 = text.parse().ok()?;

        if STEPS.contains(&value) {
            Some(Self(value))
        } else {
            None
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
        format_compact!("{}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0.into())
    }
}
