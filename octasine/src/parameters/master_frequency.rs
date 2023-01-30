use super::utils::*;
use super::ParameterValue;

const MASTER_FREQUENCY_STEPS: &[f32] = &[
    20.0, 220.0, 400.0, 435.0, 438.0, 440.0, 442.0, 445.0, 480.0, 880.0, 20_000.0,
];

#[derive(Debug, Clone, Copy)]
pub struct MasterFrequencyValue(f64);

impl Default for MasterFrequencyValue {
    fn default() -> Self {
        Self(440.0)
    }
}

impl ParameterValue for MasterFrequencyValue {
    type Value = f64;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        const MIN: f32 = MASTER_FREQUENCY_STEPS[0];
        const MAX: f32 = MASTER_FREQUENCY_STEPS[MASTER_FREQUENCY_STEPS.len() - 1];

        parse_valid_f32(text, MIN, MAX).map(|v| Self(v.into()))
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_patch_to_audio_value_with_steps(&MASTER_FREQUENCY_STEPS, value) as f64)
    }
    fn to_patch(self) -> f32 {
        map_audio_to_patch_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0 as f32)
    }
    fn get_formatted(self) -> String {
        if self.0 < 10000.0 {
            format!("{:.02} Hz", self.0)
        } else {
            format!("{:.02}", self.0)
        }
    }
}
