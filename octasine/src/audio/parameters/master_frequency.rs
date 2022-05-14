use crate::common::SampleRate;
use crate::parameter_values::{MasterFrequencyValue, ParameterValue};

use super::common::AudioParameter;

pub struct MasterFrequencyAudioParameter {
    pub value: f64,
}

impl Default for MasterFrequencyAudioParameter {
    fn default() -> Self {
        Self {
            value: MasterFrequencyValue::default().get(),
        }
    }
}

impl AudioParameter for MasterFrequencyAudioParameter {
    type Value = MasterFrequencyValue;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value = Self::Value::new_from_patch(value).get();
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(2.0 * lfo_addition)
        } else {
            self.get_value()
        }
    }
}
