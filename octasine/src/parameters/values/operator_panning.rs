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

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        Self(sync)
    }
    fn to_sync(self) -> f64 {
        self.0
    }
    fn format(self) -> String {
        let pan = ((self.0 - 0.5) * 100.0) as isize;

        match pan.cmp(&0) {
            std::cmp::Ordering::Greater => format!("{}R", pan),
            std::cmp::Ordering::Less => format!("{}L", pan),
            std::cmp::Ordering::Equal => "C".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
}
