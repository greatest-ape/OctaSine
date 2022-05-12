use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::{LfoAmountValue, ParameterValue};

use super::common::{AudioParameter, InterpolatableAudioValue};

#[derive(Debug, Clone)]
pub struct LfoAmountAudioParameter {
    value: InterpolatableAudioValue,
}

impl Default for LfoAmountAudioParameter {
    fn default() -> Self {
        let default = LfoAmountValue::default().get();

        Self {
            value: InterpolatableAudioValue::new(default, InterpolationDuration::approx_1ms()),
        }
    }
}

impl AudioParameter for LfoAmountAudioParameter {
    type Value = f64;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.value.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> Self::Value {
        self.value.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value
            .set_value(LfoAmountValue::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}
