use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct LfoAmountValue(pub f64);

impl Default for LfoAmountValue {
    fn default() -> Self {
        Self(0.0)
    }
}

impl ParameterValue for LfoAmountValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(sync * 2.0)
    }
    fn to_patch(self) -> f64 {
        self.0 * 0.5
    }
    fn get_formatted(self) -> String {
        format!("{:.04}", self.0)
    }
}
