use crate::common::SampleRate;
use crate::math::exp2_fast;
use crate::parameters::{OperatorFrequencyFineValue, ParameterValue};

use super::common::AudioParameter;

#[derive(Default)]
pub struct OperatorFrequencyFineAudioParameter(OperatorFrequencyFineValue);

impl AudioParameter for OperatorFrequencyFineAudioParameter {
    type ParameterValue = OperatorFrequencyFineValue;

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
            // log2(1.5) / 2
            const FACTOR: f32 = 0.5849625007211562 / 2.0;

            self.get_value() * exp2_fast(FACTOR * lfo_addition) as f64
        } else {
            self.get_value()
        }
    }
}
