use crate::common::SampleRate;

#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(f64);

impl InterpolationDuration {
    pub fn samples(&self, sample_rate: SampleRate) -> usize {
        (self.0 * sample_rate.0).round() as usize
    }
    /// 1.04 ms. 45.9375 samples with 44.1 Hz, 50 with 48 kHz
    pub fn approx_1ms() -> Self {
        Self(1.0 / 960.0)
    }
    /// 3.33 ms. 147 samples with 44.1 Hz, 160 with 48 kHz
    pub fn approx_3ms() -> Self {
        Self(1.0 / 300.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_duration_samples() {
        let sample_rate = SampleRate(44100.0);

        assert_eq!(InterpolationDuration::approx_1ms().samples(sample_rate), 46);
        assert_eq!(
            InterpolationDuration::approx_3ms().samples(sample_rate),
            147
        );
    }
}
