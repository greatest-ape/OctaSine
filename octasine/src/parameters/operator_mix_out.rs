use super::ParameterValue;

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
    fn new_from_text(text: String) -> Option<Self> {
        text.parse::<f32>().map(|v| Self(v.max(0.0).min(2.0))).ok()
    }
}
