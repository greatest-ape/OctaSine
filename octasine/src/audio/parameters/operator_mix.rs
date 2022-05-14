use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{OperatorMixOutValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct OperatorMixAudioParameter(InterpolatableAudioValue<OperatorMixOutValue>);

impl OperatorMixAudioParameter {
    pub fn new(operator_index: usize) -> Self {
        let value = OperatorMixOutValue::new(operator_index).get();

        Self(InterpolatableAudioValue::new_with_value(
            value,
            InterpolationDuration::approx_1ms(),
        ))
    }
}

impl AudioParameter for OperatorMixAudioParameter {
    type Value = OperatorMixOutValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.0.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.0.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.0.set_value(Self::Value::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = Self::Value::new_from_audio(self.get_value()).to_patch();

            Self::Value::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}
