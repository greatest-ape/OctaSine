use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{MasterVolumeValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct MasterVolumeAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for MasterVolumeAudioParameter {
    fn default() -> Self {
        let default = MasterVolumeValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default, InterpolationDuration::approx_1ms()),
        }
    }
}

impl AudioParameter for MasterVolumeAudioParameter {
    type Value = MasterVolumeValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.value.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(Self::Value::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition / 2.0)
        } else {
            self.get_value()
        }
    }
}
