use std::marker::PhantomData;

use crate::audio::common::InterpolationDuration;
use crate::common::SampleRate;
use crate::parameters::*;

/// Parameter storage for audio generation. Not thread-safe.
pub trait AudioParameter {
    type ParameterValue: ParameterValue;

    fn advance_one_sample(&mut self, sample_rate: SampleRate);
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value;
    fn set_from_patch(&mut self, value: f64);
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
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
    V: ParameterValue<Value = f64> + Default,
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
    V: ParameterValue<Value = f64>,
{
    type ParameterValue = V;

    fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.interpolator
            .advance_one_sample(sample_rate, &mut |_| ())
    }
    fn get_value(&self) -> <Self::ParameterValue as ParameterValue>::Value {
        self.interpolator.get_value()
    }
    fn set_from_patch(&mut self, value: f64) {
        self.interpolator.set_value(V::new_from_patch(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
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
    patch_value_cache: f64,
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
    fn set_from_patch(&mut self, value: f64) {
        self.patch_value_cache = value;
        self.value = V::new_from_patch(value);
    }
    fn get_value_with_lfo_addition(
        &mut self,
        lfo_addition: Option<f64>,
    ) -> <Self::ParameterValue as ParameterValue>::Value {
        if let Some(lfo_addition) = lfo_addition {
            V::new_from_patch((self.patch_value_cache + lfo_addition).min(1.0).max(0.0)).get()
        } else {
            self.get_value()
        }
    }
}

/// Interpolation value factor for increasing precision with very small
/// numbers.
const FACTOR: f64 = 1_000_000_000.0;

/// AudioParameter value interpolator. Supports values >= 0.0 only.
#[derive(Debug, Copy, Clone)]
pub struct Interpolator {
    value: f64,
    target_value: f64,
    step_size: f64,
    steps_remaining: usize,
    interpolation_duration: InterpolationDuration,
    sample_rate: SampleRate,
}

impl Interpolator {
    pub fn new(value: f64, interpolation_duration: InterpolationDuration) -> Self {
        Self {
            value: value * FACTOR,
            target_value: value * FACTOR,
            step_size: 0.0,
            steps_remaining: 0,
            interpolation_duration,
            sample_rate: SampleRate::default(),
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
        // Force value to be at least zero to avoid breaking expectations
        // elsewhere, notable in operator volume/mod out/mix out operator
        // dependency analysis
        (self.value / FACTOR).max(0.0)
    }

    fn restart_interpolation(&mut self) {
        let num_steps = self.interpolation_duration.samples(self.sample_rate);
        let step_size = (self.target_value - self.value) / (num_steps as f64);

        self.steps_remaining = num_steps;
        self.step_size = step_size;
    }

    #[allow(clippy::float_cmp)]
    pub fn set_value(&mut self, target_value: f64) {
        self.target_value = target_value * FACTOR;

        if (target_value - self.value).abs() <= f64::EPSILON {
            self.steps_remaining = 0;
        } else {
            self.restart_interpolation()
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::*;

    use super::*;

    #[test]
    fn test_interpolator() {
        fn prop(duration: InterpolationDuration, set_value: f64) -> TestResult {
            if set_value.is_sign_negative()
                || set_value.is_nan()
                || set_value.is_infinite()
                || set_value > (10.0f64).powf(100.0)
            {
                return TestResult::discard();
            }

            let sample_rate = SampleRate::default();
            let num_samples = duration.samples(sample_rate);

            let mut interpolator = Interpolator::new(0.0, duration);

            interpolator.set_value(set_value);

            for _ in 0..num_samples {
                interpolator.advance_one_sample(sample_rate, &mut |_| {})
            }

            let resulting_value_internal = interpolator.value / FACTOR;
            let resulting_value = interpolator.get_value();

            let accepted_error = set_value.abs() / 1_000_000_000_000.0;

            let success = ((set_value - resulting_value).abs() <= accepted_error)
                && (resulting_value - resulting_value_internal).abs() <= accepted_error;

            if !success {
                println!();
                println!("duration: {:?}", duration);
                println!("set value: {}", set_value);
                println!("resulting value: {}", resulting_value);
                println!("resulting value (interal): {}", resulting_value);
            }

            TestResult::from_bool(success)
        }

        quickcheck(
            (|value: f64| prop(InterpolationDuration::approx_1ms(), value))
                as fn(f64) -> TestResult,
        );
        quickcheck(
            (|value: f64| prop(InterpolationDuration::approx_3ms(), value))
                as fn(f64) -> TestResult,
        );
        quickcheck(
            (|value: f64| prop(InterpolationDuration::exactly_50ms(), value))
                as fn(f64) -> TestResult,
        );
    }
}
