use crate::common::SampleRate;

#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(f64);

#[allow(dead_code)]
impl InterpolationDuration {
    pub fn samples(&self, sample_rate: SampleRate) -> usize {
        (self.0 * sample_rate.0).round() as usize
    }
    /// 1.04 ms. 45.9375 samples with 44.1 Hz, 50 with 48 kHz
    pub const fn approx_1ms() -> Self {
        const DURATION: f64 = 1.0 / 960.0;

        Self(DURATION)
    }
    /// 3.33 ms. 147 samples with 44.1 Hz, 160 with 48 kHz
    pub const fn approx_3ms() -> Self {
        const DURATION: f64 = 1.0 / 300.0;

        Self(DURATION)
    }
    /// 33.33 ms. 1470 samples with 44.1 Hz, 1600 with 48 kHz
    pub const fn approx_30ms() -> Self {
        const DURATION: f64 = 1.0 / 30.0;

        Self(DURATION)
    }
    pub const fn exactly_10ms() -> Self {
        Self(0.01)
    }
    pub const fn exactly_20ms() -> Self {
        Self(0.02)
    }
    pub const fn exactly_50ms() -> Self {
        Self(0.05)
    }
    pub const fn exactly_100ms() -> Self {
        Self(0.1)
    }
}

/// Interpolation value factor for increasing precision and avoiding subnormals
/// with very small numbers.
const FACTOR: f32 = 1_000_000_000.0;

/// AudioParameter value interpolator. Supports values >= 0.0 only.
#[derive(Debug, Copy, Clone)]
pub struct Interpolator {
    /// Value to be externally consumed
    cached_value: f32,
    current_value: f32,
    target_value: f32,
    step_size: f32,
    steps_remaining: usize,
    interpolation_duration: InterpolationDuration,
    sample_rate: SampleRate,
}

impl Interpolator {
    pub fn new(value: f32, interpolation_duration: InterpolationDuration) -> Self {
        Self {
            cached_value: value,
            current_value: value * FACTOR,
            target_value: value * FACTOR,
            step_size: 0.0,
            steps_remaining: 0,
            interpolation_duration,
            sample_rate: SampleRate::default(),
        }
    }

    pub fn advance_one_sample<F: FnMut(f32)>(
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
        self.current_value += self.step_size;

        // Force value to be at least zero to avoid breaking expectations
        // elsewhere, notable in operator volume/mod out/mix out operator
        // dependency analysis
        self.cached_value = (self.current_value / FACTOR).max(0.0);

        callback_on_advance(self.cached_value);
    }

    pub fn get_value(&self) -> f32 {
        self.cached_value
    }

    fn restart_interpolation(&mut self) {
        let num_steps = self.interpolation_duration.samples(self.sample_rate);
        let step_size = (self.target_value - self.current_value) / (num_steps as f32);

        self.steps_remaining = num_steps;
        self.step_size = step_size;
    }

    #[allow(clippy::float_cmp)]
    pub fn set_value(&mut self, target_value: f32) {
        self.target_value = target_value * FACTOR;

        if self.target_value == self.current_value {
            self.steps_remaining = 0;
        } else {
            self.restart_interpolation()
        }
    }

    /// Immediately set value to target value
    pub fn force_set_value(&mut self, target_value: f32) {
        self.target_value = target_value * FACTOR;
        self.current_value = target_value * FACTOR;
        self.cached_value = target_value;
        self.steps_remaining = 0;
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::*;

    use super::*;

    #[test]
    fn test_interpolation_duration_samples() {
        use InterpolationDuration as D;

        assert_eq!(D::approx_1ms().samples(SampleRate(44100.0)), 46);
        assert_eq!(D::approx_1ms().samples(SampleRate(48000.0)), 50);

        assert_eq!(D::approx_3ms().samples(SampleRate(44100.0)), 147);
        assert_eq!(D::approx_3ms().samples(SampleRate(48000.0)), 160);

        assert_eq!(D::approx_30ms().samples(SampleRate(44100.0)), 1470);
        assert_eq!(D::approx_30ms().samples(SampleRate(48000.0)), 1600);

        assert_eq!(D::exactly_10ms().samples(SampleRate(44100.0)), 441);
        assert_eq!(D::exactly_10ms().samples(SampleRate(48000.0)), 480);
    }

    #[test]
    fn test_interpolator() {
        fn prop(duration: InterpolationDuration, set_value: f32) -> TestResult {
            if set_value.is_sign_negative()
                || set_value.is_nan()
                || set_value.is_infinite()
                || set_value > (10.0f32).powf(20.0)
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

            let resulting_value_internal = interpolator.current_value / FACTOR;
            let resulting_value = interpolator.get_value();

            let accepted_error = set_value.abs() / 10_000.0;

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
            (|value: f32| prop(InterpolationDuration::approx_1ms(), value))
                as fn(f32) -> TestResult,
        );
        quickcheck(
            (|value: f32| prop(InterpolationDuration::approx_3ms(), value))
                as fn(f32) -> TestResult,
        );
        quickcheck(
            (|value: f32| prop(InterpolationDuration::exactly_50ms(), value))
                as fn(f32) -> TestResult,
        );
    }
}
