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

#[cfg(test)]
mod tests {
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
}
