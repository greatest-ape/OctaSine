use crate::common::*;
use crate::constants::*;

use super::ParameterValue;

#[derive(Debug, Clone, Copy)]
pub struct OperatorWaveTypeValue(pub WaveType);

impl Default for OperatorWaveTypeValue {
    fn default() -> Self {
        Self(DEFAULT_OPERATOR_WAVE_TYPE)
    }
}

impl ParameterValue for OperatorWaveTypeValue {
    type Value = WaveType;

    fn from_processing(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn from_sync(sync: f64) -> Self {
        if sync <= 0.5 {
            Self(WaveType::Sine)
        } else {
            Self(WaveType::WhiteNoise)
        }
    }
    fn to_sync(self) -> f64 {
        match self.0 {
            WaveType::Sine => 0.0,
            WaveType::WhiteNoise => 1.0,
        }
    }
    fn format(self) -> String {
        match self.0 {
            WaveType::Sine => "SINE".to_string(),
            WaveType::WhiteNoise => "NOISE".to_string(),
        }
    }
    fn format_sync(value: f64) -> String {
        Self::from_sync(value).format()
    }
    fn from_text(text: String) -> Option<Self> {
        let value = text.to_lowercase();

        if value.contains("sin") {
            Some(OperatorWaveTypeValue(WaveType::Sine))
        } else if value.contains("noise") {
            Some(OperatorWaveTypeValue(WaveType::WhiteNoise))
        } else {
            None
        }
    }
}
