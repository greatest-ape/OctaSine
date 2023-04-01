use std::f32::consts::FRAC_PI_2;

use compact_str::{format_compact, CompactString};

use super::{utils::parse_valid_f32, ParameterValue, SerializableRepresentation};

#[derive(Debug, Clone, Copy)]
pub struct OperatorPanningValue(f32);

impl OperatorPanningValue {
    pub fn calculate_left_and_right(&self) -> [f32; 2] {
        let pan_phase = self.0 * FRAC_PI_2;

        [
            ::sleef_trig::Sleef_cosf1_u35purec_range125(pan_phase),
            ::sleef_trig::Sleef_sinf1_u35purec_range125(pan_phase),
        ]
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
    fn new_from_text(text: &str) -> Option<Self> {
        let text = text.trim().to_lowercase();

        if text.as_str() == "c" || text.as_str() == "0" {
            Some(Self(0.5))
        } else if let Some(index) = text.rfind("r") {
            let mut text = text;

            text.remove(index);

            let value = parse_valid_f32(&text, 0.0, 50.0)?;

            Some(Self((0.5 + value / 100.0).min(1.0).max(0.0)))
        } else if let Some(index) = text.rfind("l") {
            let mut text = text;

            text.remove(index);

            let value = parse_valid_f32(&text, 0.0, 50.0)?;

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
    fn get_formatted(self) -> CompactString {
        let pan = ((self.0 - 0.5) * 100.0).round() as isize;

        match pan.cmp(&0) {
            std::cmp::Ordering::Greater => format_compact!("{}R", pan),
            std::cmp::Ordering::Less => format_compact!("{}L", pan.abs()),
            std::cmp::Ordering::Equal => "C".into(),
        }
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0.into())
    }
}
