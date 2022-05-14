use std::marker::PhantomData;

use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameter_values::*;

pub trait AudioParameter {
    type Value: ParameterValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate);
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value;
    fn set_from_patch(&mut self, value: f64);
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value;
}

#[derive(Debug, Clone)]
pub struct InterpolatableAudioParameter<V: ParameterValue>(InterpolatableAudioValue<V>);

impl<V> Default for InterpolatableAudioParameter<V>
where
    V: ParameterValue<Value = f64> + Default,
{
    fn default() -> Self {
        Self(InterpolatableAudioValue::new(
            InterpolationDuration::approx_1ms(),
        ))
    }
}

impl<V> AudioParameter for InterpolatableAudioParameter<V>
where
    V: ParameterValue<Value = f64>,
{
    type Value = V;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.0.advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.0.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.0.set_value(V::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
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
    sync_cache: f64,
}

impl<V: ParameterValue + Default> Default for SimpleAudioParameter<V> {
    fn default() -> Self {
        Self {
            value: V::default(),
            sync_cache: V::default().to_patch(),
        }
    }
}

impl<V: ParameterValue> AudioParameter for SimpleAudioParameter<V> {
    type Value = V;

    fn advance_one_sample(&mut self, _sample_rate: SampleRate) {}
    fn get_value(&self) -> <Self::Value as ParameterValue>::Value {
        self.value.get()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.sync_cache = value;
        self.value = V::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::Value as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            V::new_from_patch((self.sync_cache + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InterpolatableAudioValue<V: ParameterValue> {
    value: f64,
    target_value: f64,
    step_size: f64,
    steps_remaining: usize,
    interpolation_duration: InterpolationDuration,
    sample_rate: SampleRate,
    phantom_data: PhantomData<V>,
}

impl<V> InterpolatableAudioValue<V>
where
    V: ParameterValue<Value = f64>,
{
    pub fn new(interpolation_duration: InterpolationDuration) -> Self {
        Self::new_with_value(V::default().get(), interpolation_duration)
    }
    pub fn new_with_value(value: f64, interpolation_duration: InterpolationDuration) -> Self {
        Self {
            value,
            target_value: value,
            step_size: 0.0,
            steps_remaining: 0,
            interpolation_duration,
            sample_rate: SampleRate::default(),
            phantom_data: Default::default(),
        }
    }

    pub fn advance_one_sample<F: FnMut(f64)>(
        &mut self,
        sample_rate: SampleRate,
        callback_on_advance: &mut F,
    ) {
        if self.steps_remaining == 0 {
            return;
        }
        if sample_rate != self.sample_rate {
            self.sample_rate = sample_rate;

            self.restart_interpolation();

            if self.steps_remaining == 0 {
                return;
            }
        }

        self.steps_remaining -= 1;
        self.value += self.step_size;

        callback_on_advance(self.value);
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    fn restart_interpolation(&mut self) {
        let num_steps = self.interpolation_duration.samples(self.sample_rate);
        let step_size = (self.target_value - self.value) / (num_steps as f64);

        self.steps_remaining = num_steps;
        self.step_size = step_size;
    }

    #[allow(clippy::float_cmp)]
    pub fn set_value(&mut self, target_value: f64) {
        self.target_value = target_value;

        if (target_value - self.value).abs() <= f64::EPSILON {
            self.steps_remaining = 0;
        } else {
            self.restart_interpolation()
        }
    }
}
