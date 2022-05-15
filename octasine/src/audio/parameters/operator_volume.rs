use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{OperatorVolumeValue, ParameterValue};

use super::common::{AudioParameter, Interpolator};

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
    fn set_from_patch(&mut self, value: f64) {
        self.0
            .set_value(Self::ParameterValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition / 2.0)
        } else {
            self.get_value()
        }
    }
}
