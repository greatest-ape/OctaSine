#[derive(Debug, Copy, Clone)]
pub struct InterpolationDuration(f64);

impl InterpolationDuration {
    pub fn get(&self) -> f64 {
        self.0
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

impl Default for InterpolationDuration {
    fn default() -> Self {
        Self::slow()
    }
}
