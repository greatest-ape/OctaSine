use compact_str::{format_compact, CompactString};

use super::{utils::parse_valid_f32, ParameterValue};

#[derive(Default, Debug, Clone, Copy)]
pub struct OperatorMixOutValue(f32);

impl OperatorMixOutValue {
    pub fn new(index: usize) -> Self {
        if index == 0 {
            Self(1.0)
        } else {
            Self(0.0)
        }
    }
}

impl ParameterValue for OperatorMixOutValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        parse_valid_f32(text, 0.0, 2.0).map(Self)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value * 2.0)
    }
    fn to_patch(self) -> f32 {
        self.0 / 2.0
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{:.04}", self.0)
    }
}
