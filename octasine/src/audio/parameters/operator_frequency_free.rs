use crate::math::exp2;
use crate::common::SampleRate;
use crate::parameters::{OperatorFrequencyFreeValue, ParameterValue};

use super::common::AudioParameter;

#[derive(Default)]
pub struct OperatorFrequencyFreeAudioParameter(OperatorFrequencyFreeValue);

impl AudioParameter for OperatorFrequencyFreeAudioParameter {
    type ParameterValue = OperatorFrequencyFreeValue;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.0.get()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.0 = Self::ParameterValue::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * exp2(4.0 * lfo_addition) as f64
        } else {
            self.get_value()
        }
    }
}
