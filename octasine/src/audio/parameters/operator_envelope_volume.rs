use std::marker::PhantomData;

use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameters::ParameterValue;

use super::common::{AudioParameter, Interpolator};

#[derive(Debug, Clone)]
pub struct OperatorEnvelopeVolumeAudioParameter<V: ParameterValue> {
    interpolator: Interpolator,
    phantom_data: PhantomData<V>,
}

impl<V> Default for OperatorEnvelopeVolumeAudioParameter<V>
where
    V: ParameterValue<Value = f32> + Default,
{
    fn default() -> Self {
        Self {
            interpolator: Interpolator::new(
                V::default().get(),
                InterpolationDuration::approx_3ms(),
            ),
            phantom_data: Default::default(),
        }
    }
}

impl<V> AudioParameter for OperatorEnvelopeVolumeAudioParameter<V>
where
    V: ParameterValue<Value = f32>,
{
    type ParameterValue = V;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.interpolator
            .advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.interpolator.get_value().min(1.0)
    }
    fn set_from_patch(&mut self, value: f32) {
        self.interpolator.set_value(V::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        _lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        self.get_value()
    }
}
