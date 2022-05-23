use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct LfoAmountValue(pub f32);

impl Default for LfoAmountValue {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ParameterValue for LfoAmountValue {
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
        self.0 * 0.5
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
