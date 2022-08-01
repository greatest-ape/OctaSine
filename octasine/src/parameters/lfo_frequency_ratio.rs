use super::utils::*;
use super::ParameterValue;

const LFO_FREQUENCY_RATIO_STEPS: [f32; 9] = [
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

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: String) -> Option<Self> {
        const MIN: f32 = LFO_FREQUENCY_RATIO_STEPS[0];
        const MAX: f32 = LFO_FREQUENCY_RATIO_STEPS[LFO_FREQUENCY_RATIO_STEPS.len() - 1];

        let value = parse_valid_f32(text, MIN, MAX)?;

        Some(Self(
            round_to_step(&LFO_FREQUENCY_RATIO_STEPS[..], value).into(),
        ))
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(map_parameter_value_to_step(&LFO_FREQUENCY_RATIO_STEPS, value) as f64)
    }
    fn to_patch(self) -> f32 {
        map_step_to_parameter_value(&LFO_FREQUENCY_RATIO_STEPS, self.0 as f32)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
