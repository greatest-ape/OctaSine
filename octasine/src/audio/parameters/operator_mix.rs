use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{OperatorMixOutValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct OperatorMixAudioParameter {
    value: InterpolatableAudioValue,
}

impl OperatorMixAudioParameter {
    pub fn new(operator_index: usize) -> Self {
        let value = OperatorMixOutValue::new(operator_index).get();

        Self {
            value: InterpolatableAudioValue::new(value, InterpolationDuration::approx_1ms()),
        }
    }
}

impl AudioParameter for OperatorMixAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.value.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(OperatorMixOutValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = OperatorMixOutValue::new_from_audio(self.get_value()).to_patch();

            OperatorMixOutValue::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0))
                .get()
        } else {
            self.get_value()
        }
    }
}
