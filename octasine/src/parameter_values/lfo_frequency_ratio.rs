use super::utils::*;
use super::ParameterValue;

const LFO_FREQUENCY_RATIO_STEPS: [f64; 9] = [
    1.0 / 16.0,
    1.0 / 8.0,
    1.0 / 4.0,
    1.0 / 2.0,
    1.0,
    2.0,
    4.0,
    8.0,
    16.0,
];

#[derive(Debug, Clone, Copy)]
pub struct LfoFrequencyRatioValue(pub f64);

impl Default for LfoFrequencyRatioValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for LfoFrequencyRatioValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(map_parameter_value_to_step(
            &LFO_FREQUENCY_RATIO_STEPS,
            sync,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(&LFO_FREQUENCY_RATIO_STEPS, self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
    fn from_text(_: String) -> Option<Self> {
        None // FIXME
    }
}
