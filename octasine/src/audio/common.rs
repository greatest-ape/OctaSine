use crate::common::SampleRate;

#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(f64);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_duration_samples() {
        use InterpolationDuration as D;

        assert_eq!(D::approx_1ms().samples(SampleRate(44100.0)), 46);
        assert_eq!(D::approx_3ms().samples(SampleRate(44100.0)), 147);

        assert_eq!(D::approx_1ms().samples(SampleRate(48000.0)), 50);
        assert_eq!(D::approx_3ms().samples(SampleRate(48000.0)), 160);
    }
}
