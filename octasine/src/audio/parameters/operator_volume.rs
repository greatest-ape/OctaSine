use crate::audio::interpolation::{InterpolationDuration, Interpolator};
use crate::common::SampleRate;
use crate::math::exp2_fast;
use crate::parameters::{OperatorVolumeValue, ParameterValue};

use super::common::AudioParameter;

#[derive(Debug, Clone)]
pub struct OperatorVolumeAudioParameter(Interpolator);

impl Default for OperatorVolumeAudioParameter {
    fn default() -> Self {
        Self(Interpolator::new(
            OperatorVolumeValue::default().get(),
            InterpolationDuration::approx_1ms(),
        ))
    }
}

impl AudioParameter for OperatorVolumeAudioParameter {
    type ParameterValue = OperatorVolumeValue;

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
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * exp2_fast(lfo_addition / 2.0)
        } else {
            self.get_value()
        }
    }
}
