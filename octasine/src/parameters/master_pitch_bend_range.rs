use compact_str::format_compact;
use compact_str::CompactString;

use super::utils::*;
use super::ParameterValue;
use super::SerializableRepresentation;

const STEPS: &[f32] = &[
    0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 24.0, 48.0,
];

#[derive(Debug, Clone, Copy)]
pub struct MasterPitchBendRangeValue(f32);

impl Default for MasterPitchBendRangeValue {
    fn default() -> Self {
        Self(2.0)
    }
}

impl ParameterValue for MasterPitchBendRangeValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        const MIN: f32 = STEPS[0];
        const MAX: f32 = STEPS[STEPS.len() - 1];

        Some(Self(
            round_to_step(STEPS, parse_valid_f32(text, MIN, MAX)?).into(),
        ))
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_value_to_step(STEPS, value))
    }
    fn to_patch(self) -> f32 {
        map_step_to_patch_value(STEPS, self.0)
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{:.0} semis", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0 as f64)
    }
}
