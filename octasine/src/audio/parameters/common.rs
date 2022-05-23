use std::marker::PhantomData;

use crate::audio::common::{InterpolationDuration, Interpolator};
use crate::common::SampleRate;
use crate::parameters::*;

/// Parameter storage for audio generation. Not thread-safe.
pub trait AudioParameter {
    type ParameterValue: ParameterValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate);
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value;
    fn set_from_patch(&mut self, value: f32);
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value;

    fn get_parameter_value(&self) -> Self::ParameterValue {
        Self::ParameterValue::new_from_audio(self.get_value())
    }
}

#[derive(Debug, Clone)]
pub struct InterpolatableAudioParameter<V: ParameterValue> {
    interpolator: Interpolator,
    phantom_data: PhantomData<V>,
}

impl<V> Default for InterpolatableAudioParameter<V>
where
    V: ParameterValue<Value = f32> + Default,
{
    fn default() -> Self {
        Self {
            interpolator: Interpolator::new(
                V::default().get(),
                InterpolationDuration::approx_1ms(),
            ),
            phantom_data: Default::default(),
        }
    }
}

impl<V> AudioParameter for InterpolatableAudioParameter<V>
where
    V: ParameterValue<Value = f32>,
{
    type ParameterValue = V;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.interpolator
            .advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.interpolator.get_value()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.interpolator.set_value(V::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            let patch_value = V::new_from_audio(self.get_value()).to_patch();

            V::new_from_patch((patch_value + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

pub struct SimpleAudioParameter<V: ParameterValue> {
    value: V,
    patch_value_cache: f32,
}

impl<V: ParameterValue + Default> Default for SimpleAudioParameter<V> {
    fn default() -> Self {
        Self {
            value: V::default(),
            patch_value_cache: V::default().to_patch(),
        }
    }
}

impl<V: ParameterValue> AudioParameter for SimpleAudioParameter<V> {
    type ParameterValue = V;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.value.get()
    }
    fn set_from_patch(&mut self, value: f32) {
        self.patch_value_cache = value;
        self.value = V::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f32>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            V::new_from_patch((self.patch_value_cache + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}
