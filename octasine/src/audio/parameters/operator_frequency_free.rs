use crate::common::SampleRate;
use crate::parameter_values::{OperatorFrequencyFreeValue, ParameterValue};

use super::common::AudioParameter;

pub struct OperatorFrequencyFreeAudioParameter {
    pub value: f64,
}

impl Default for OperatorFrequencyFreeAudioParameter {
    fn default() -> Self {
        Self {
            value: OperatorFrequencyFreeValue::default().get(),
        }
    }
}

impl AudioParameter for OperatorFrequencyFreeAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value = OperatorFrequencyFreeValue::new_from_patch(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}
