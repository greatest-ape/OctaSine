use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct OperatorPanningValue(f32);

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
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f32) -> Self {
        Self(value)
    }
    fn to_patch(self) -> f32 {
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
}
