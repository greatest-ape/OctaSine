use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameters::{OperatorMixOutValue, ParameterValue};

use super::common::{AudioParameter, Interpolator};

#[derive(Debug, Clone)]
pub struct OperatorMixAudioParameter(Interpolator);

impl OperatorMixAudioParameter {
    pub fn new(operator_index: usize) -> Self {
        Self(Interpolator::new(
            OperatorMixOutValue::new(operator_index).get(),
            InterpolationDuration::approx_1ms(),
        ))
    }
}

impl AudioParameter for OperatorMixAudioParameter {
    type ParameterValue = OperatorMixOutValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.0.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.0.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.0
            .set_value(Self::ParameterValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = Self::ParameterValue::new_from_audio(self.get_value()).to_patch();

            Self::ParameterValue::new_from_patch(
                (patch_value + lfo_addition as f64).min(1.0).max(0.0),
            )
            .get()
        } else {
            self.get_value()
        }
    }
}
