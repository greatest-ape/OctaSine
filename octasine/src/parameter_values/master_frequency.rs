use super::utils::*;
use super::ParameterValue;

const MASTER_FREQUENCY_STEPS: [f64; 12] = [
    20.0, 110.0, 220.0, 400.0, 435.0, 438.0, 440.0, 442.0, 445.0, 480.0, 880.0, 20_000.0,
];
const DEFAULT_MASTER_FREQUENCY: f64 = 440.0;

#[derive(Debug, Clone, Copy)]
pub struct MasterFrequencyValue(f64);

impl Default for MasterFrequencyValue {
    fn default() -> Self {
        Self(DEFAULT_MASTER_FREQUENCY)
    }
}

impl ParameterValue for MasterFrequencyValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_value_with_steps(
            &MASTER_FREQUENCY_STEPS,
            sync,
        ))
    }
    fn to_patch(self) -> f64 {
        map_value_to_parameter_value_with_steps(&MASTER_FREQUENCY_STEPS, self.0)
    }
    fn get_formatted(self) -> String {
        if self.0 < 10000.0 {
            format!("{:.02} Hz", self.0)
        } else {
            format!("{:.02}", self.0)
        }
    }
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
}
