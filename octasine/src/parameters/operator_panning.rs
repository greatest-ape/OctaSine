use std::f32::consts::FRAC_PI_2;

use crate::math::{cos, sin};

use super::{utils::parse_valid_f32, ParameterValue};

#[derive(Debug, Clone, Copy)]
pub struct OperatorPanningValue(f32);

impl OperatorPanningValue {
    pub fn calculate_left_and_right(&self) -> [f32; 2] {
        let pan_phase = self.0 * FRAC_PI_2;

        [cos(pan_phase), sin(pan_phase)]
    }
}

impl Default for OperatorPanningValue {
    fn default() -> Self {
        Self(0.5)
    }
}

impl ParameterValue for OperatorPanningValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: String) -> Option<Self> {
        let text = text.trim().to_lowercase();

        if text.as_str() == "c" || text.as_str() == "0" {
            Some(Self(0.5))
        } else if let Some(index) = text.rfind("r") {
            let mut text = text;

            text.remove(index);

            let value = parse_valid_f32(text, 0.0, 50.0)?;

            Some(Self((0.5 + value / 100.0).min(1.0).max(0.0)))
        } else if let Some(index) = text.rfind("l") {
            let mut text = text;

            text.remove(index);

            let value = parse_valid_f32(text, 0.0, 50.0)?;

            Some(Self((0.5 - value / 100.0).min(1.0).max(0.0)))
        } else {
            None
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value)
    }
    fn to_patch(self) -> f32 {
        self.0
    }
    fn get_formatted(self) -> String {
        let pan = ((self.0 - 0.5) * 100.0).round() as isize;

        match pan.cmp(&0) {
            std::cmp::Ordering::Greater => format!("{}R", pan),
            std::cmp::Ordering::Less => format!("{}L", pan.abs()),
            std::cmp::Ordering::Equal => "C".to_string(),
        }
    }
}
