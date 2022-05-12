use crate::common::SampleRate;
use crate::parameter_values::ParameterValue;

use super::common::AudioParameter;

pub struct FreeFrequencyAudioParameter<P: ParameterValue<Value = f64>> {
    pub value: <P as ParameterValue>::Value,
}

impl<P: ParameterValue<Value = f64> + Default> Default for FreeFrequencyAudioParameter<P> {
    fn default() -> Self {
        Self {
            value: P::default().get(),
        }
    }
}

impl<P> AudioParameter for FreeFrequencyAudioParameter<P>
where
    P: ParameterValue<Value = f64>,
{
    type Value = <P as ParameterValue>::Value;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> Self::Value {
        self.value
    }
    fn set_from_patch(&mut self, value: f64) {
        self.value = P::new_from_patch(value).get();
    }
    fn get_value_with_lfo_addition(&mut self, lfo_addition: Option<f64>) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            self.get_value() * 2.0f64.powf(lfo_addition)
        } else {
            self.get_value()
        }
    }
}
