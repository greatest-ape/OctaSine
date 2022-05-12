use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{OperatorActiveValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct OperatorActiveAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for OperatorActiveAudioParameter {
    fn default() -> Self {
        Self {
            value: InterpolatableAudioValue::new(1.0, InterpolationDuration::exactly_50ms()),
        }
    }
}

impl AudioParameter for OperatorActiveAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.value.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorActiveValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, _lfo_addition: Option<f64>) -> Self::Value {
        self.get_value()
    }
}
