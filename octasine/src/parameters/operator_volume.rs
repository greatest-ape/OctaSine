use super::{utils::parse_valid_f32, ParameterValue};

#[derive(Debug, Clone, Copy)]
pub struct OperatorVolumeValue(f32);

impl Default for OperatorVolumeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for OperatorVolumeValue {
    type Value = f32;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn new_from_text(text: String) -> Option<Self> {
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
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
