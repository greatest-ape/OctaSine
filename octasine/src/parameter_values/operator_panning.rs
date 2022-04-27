use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct OperatorPanningValue(f64);

impl Default for OperatorPanningValue {
    fn default() -> Self {
        Self(0.5)
    }
}

impl ParameterValue for OperatorPanningValue {
    type Value = f64;

    fn from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_patch(sync: f64) -> Self {
        Self(sync)
    }
    fn to_patch(self) -> f64 {
        self.0
    }
    fn get_formatted(self) -> String {
        let pan = ((self.0 - 0.5) * 100.0) as isize;

        match pan.cmp(&0) {
            std::cmp::Ordering::Greater => format!("{}R", pan),
            std::cmp::Ordering::Less => format!("{}L", pan),
            std::cmp::Ordering::Equal => "C".to_string(),
        }
    }
    fn convert_patch_to_audio_formatted(value: f64) -> String {
        Self::from_patch(value).get_formatted()
    }
}
