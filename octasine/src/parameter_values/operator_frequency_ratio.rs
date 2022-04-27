use std::f64::consts::PI;

use once_cell::sync::Lazy;

use super::utils::*;
use super::ParameterValue;

pub static OPERATOR_RATIO_STEPS: Lazy<Vec<f64>> = Lazy::new(|| {
    let mut ratios = vec![
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
        3.0 / 2.0, // 1.50 (perfect 5th)
        5.0 / 3.0, // 1.67 (major 6th)
        2.0,       // 2.00
        5.0 / 2.0, // 2.50 (major 3rd)
        3.0,       // 3.00
        10. / 3.0, // 3.33 (major 6th)
        4.0,       // 4.00
        5.0 / 1.0, // 5.00 (major 3rd)
        6.0,       // 6.00
        8.0,       // 8.00
    ];

    // Add DX ratios

    for i in 0..16 {
        let factor = if i == 0 { 0.5 } else { f64::from(i) };

        if i != 5 && i != 15 {
            ratios.push(factor * 2.0f64.sqrt());
        }
        ratios.push(factor * 3.0f64.sqrt());
    }

    for i in 1..8 {
        ratios.push(f64::from(i) * PI);
    }

    for i in [1, 2, 6, 9, 10, 14, 18, 22, 26, 27, 30] {
        ratios.push(f64::from(i) * PI / 4.0);
    }

    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

    ratios
});

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantiate_ratios() {
        assert!(!OPERATOR_RATIO_STEPS.is_empty());
    }
}
