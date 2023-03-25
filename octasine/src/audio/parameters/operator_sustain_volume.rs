use crate::audio::interpolation::{InterpolationDuration, Interpolator};
use crate::common::SampleRate;
use crate::parameters::{OperatorSustainVolumeValue, ParameterValue};

use super::common::AudioParameter;

#[derive(Debug, Clone)]
pub struct OperatorSustainVolumeAudioParameter {
    interpolator: Interpolator,
}

impl Default for OperatorSustainVolumeAudioParameter {
    fn default() -> Self {
        Self {
            interpolator: Interpolator::new(
                OperatorSustainVolumeValue::default().get(),
                InterpolationDuration::approx_3ms(),
            ),
        }
    }
}

impl AudioParameter for OperatorSustainVolumeAudioParameter {
    type ParameterValue = OperatorSustainVolumeValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.interpolator
            .advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> f32 {
        self.interpolator.get_value().min(1.0)
    }
    fn set_from_patch(&mut self, value: f32) {
        self.interpolator
            .set_value(Self::ParameterValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, _lfo_addition: Option<f32>) -> f32 {
        self.get_value()
    }
}
