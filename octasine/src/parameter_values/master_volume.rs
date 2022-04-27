use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct MasterVolumeValue(f64);

impl Default for MasterVolumeValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ParameterValue for MasterVolumeValue {
    type Value = f64;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(value * 2.0)
    }
    fn to_patch(self) -> f64 {
        self.0 / 2.0
    }
    fn get_formatted(self) -> String {
        format!("{:.2} dB", 20.0 * self.0.log10())
    }
}
