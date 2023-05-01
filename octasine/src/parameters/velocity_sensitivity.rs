use compact_str::{format_compact, CompactString};

use super::{utils::parse_valid_f32, ParameterValue, SerializableRepresentation};

#[derive(Debug, Clone, Copy)]
pub struct VelocitySensitivityValue(f32);

impl Default for VelocitySensitivityValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for VelocitySensitivityValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        parse_valid_f32(text, 0.0, 1.0).map(Self)
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
        format_compact!("{:.04}", self.0)
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Float(self.0.into())
    }
}
