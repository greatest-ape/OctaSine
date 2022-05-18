use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameters::{OperatorActiveValue, ParameterValue};

use super::common::{AudioParameter, Interpolator};

#[derive(Debug, Clone)]
pub struct OperatorActiveAudioParameter(Interpolator);

impl Default for OperatorActiveAudioParameter {
    fn default() -> Self {
        Self(Interpolator::new(
            OperatorActiveValue::default().get(),
            InterpolationDuration::exactly_50ms(),
        ))
    }
}

impl AudioParameter for OperatorActiveAudioParameter {
    type ParameterValue = OperatorActiveValue;

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
        _lfo_addition: Option<f64>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        self.get_value()
    }
}
