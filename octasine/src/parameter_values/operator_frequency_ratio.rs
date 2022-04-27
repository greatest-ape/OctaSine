use std::f64::consts::PI;

use super::utils::*;
use super::ParameterValue;

// pub const OPERATOR_RATIO_STEPS: [f64; 28] = [
//     0.0625, 0.125, 0.2, 0.25, 0.33, 0.5, 0.55, 0.66, 0.6896, 0.8, 0.8333, 1.0, 1.2, 1.25, 1.33,
//     1.45, 1.5, 1.8, 1.875, 2.0, 2.5, 3.0, 3.5, 4.0, 8.0, 16.0, 32.0, 64.0,
// ];
pub const OPERATOR_RATIO_STEPS: &[f64] = &[
    1.0 / 8.0, // 0.125
    1.0 / 6.0, // 0.167
    1.0 / 5.0, // 0.20
    1.0 / 4.0, // 0.25
    1.0 / 3.0, // 0.33 (perfect 5th)
    3.0 / 8.0, // 0.375 (perfect 4th)
    1.0 / 2.0, // 0.50
    2.0 / 3.0, // 0.67 (perfect 5th)
    3.0 / 4.0, // 0.75 (perfect 4th)
    4.0 / 5.0, // 0.80
    5.0 / 6.0, // 0.83
    1.0,
    75. / 64., // 1.17 (just augmented second)
    6.0 / 5.0, // 1.20 (minor 3rd)
    5.0 / 4.0, // 1.25 (major 3rd)
    4.0 / 3.0, // 1.33 (perfect 4th)
    1.4142135, // 1.41 (√2)
    3.0 / 2.0, // 1.50 (perfect 5th)
    5.0 / 3.0, // 1.67 (major 6th)
    2.0,       // 2.00
    5.0 / 2.0, // 2.50 (major 3rd)
    3.0,       // 3.00
    PI,        // 3.14
    10. / 3.0, // 3.33 (major 6th)
    4.0,       // 4.00
    5.0 / 1.0, // 5.00 (major 3rd)
    6.0,       // 6.00
    8.0,       // 8.00
];

#[derive(Debug, Clone, Copy)]
pub struct OperatorFrequencyRatioValue(f64);

impl Default for OperatorFrequencyRatioValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorFrequencyRatioValue {
    type Value = f64;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(map_parameter_value_to_step(
            &OPERATOR_RATIO_STEPS[..],
            value,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], self.0)
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
    fn new_from_text(text: String) -> Option<Self> {
        text.parse::<f64>()
            .ok()
            .map(|value| Self(round_to_step(&OPERATOR_RATIO_STEPS[..], value)))
    }
}
