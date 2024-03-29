use crate::audio::interpolation::{InterpolationDuration, Interpolator};
use crate::common::SampleRate;
use crate::parameters::{LfoActiveValue, ParameterValue};

use super::common::AudioParameter;

#[derive(Debug, Clone)]
pub struct LfoActiveAudioParameter(Interpolator);

impl Default for LfoActiveAudioParameter {
    fn default() -> Self {
        Self(Interpolator::new(
            LfoActiveValue::default().get(),
            InterpolationDuration::exactly_50ms(),
        ))
    }
}

impl AudioParameter for LfoActiveAudioParameter {
    type ParameterValue = LfoActiveValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.0.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.0.get_value()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.0
            .set_value(Self::ParameterValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        _lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        self.get_value()
    }
}
