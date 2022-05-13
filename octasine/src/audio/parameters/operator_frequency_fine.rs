use crate::common::SampleRate;
use crate::parameter_values::{OperatorFrequencyFineValue, ParameterValue};

use super::common::AudioParameter;

pub struct OperatorFrequencyFineAudioParameter {
    pub value: f64,
}

impl Default for OperatorFrequencyFineAudioParameter {
    fn default() -> Self {
        Self {
            value: OperatorFrequencyFineValue::default().get(),
        }
    }
}

impl AudioParameter for OperatorFrequencyFineAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value = OperatorFrequencyFineValue::new_from_patch(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            // log2(1.5) / 2
            const FACTOR: f64 = 0.5849625007211562 / 2.0;

            self.get_value() * 2.0f64.powf(FACTOR * lfo_addition)
        } else {
            self.get_value()
        }
    }
}
