use compact_str::{format_compact, CompactString};

use super::{ParameterValue, SerializableRepresentation};

#[derive(Debug, Clone, Copy)]
pub struct GlideRetriggerValue(bool);

impl Default for GlideRetriggerValue {
    fn default() -> Self {
        Self(false)
    }
}

impl ParameterValue for GlideRetriggerValue {
    type Value = bool;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: &str) -> Option<Self> {
        match text.trim().to_lowercase().as_str() {
            "off" => Some(Self(false)),
            "on" => Some(Self(true)),
            _ => None,
        }
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value > 0.5)
    }
    fn to_patch(self) -> f32 {
        if self.0 {
            1.0
        } else {
            0.0
        }
    }
    fn get_formatted(self) -> CompactString {
        format_compact!("{}", if self.0 { "ON" } else { "OFF" })
    }

    fn get_serializable(&self) -> SerializableRepresentation {
        SerializableRepresentation::Other(self.get_formatted())
    }
}
