use crate::common::SampleRate;

#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(f64);

impl InterpolationDuration {
    pub fn samples(&self, sample_rate: SampleRate) -> usize {
        (self.0 * sample_rate.0).round() as usize
    }
    /// 0.52 ms. 22.96875 samples with 44.1 kHz, 25 with 48 kHz
    pub fn fast() -> Self {
        Self(1.0 / 1920.0)
    }
    /// 1.04 ms. 45.9375 samples with 44.1 Hz, 50 with 48 kHz
    pub fn medium() -> Self {
        Self(1.0 / 960.0)
    }
    /// 3.33 ms. 147 samples with 44.1 Hz, 160 with 48 kHz
    pub fn slow() -> Self {
        Self(1.0 / 300.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_duration_samples() {
        let sample_rate = SampleRate(44100.0);

        assert_eq!(InterpolationDuration::slow().samples(sample_rate), 147);
        assert_eq!(InterpolationDuration::medium().samples(sample_rate), 46);
        assert_eq!(InterpolationDuration::fast().samples(sample_rate), 23);
    }
}
