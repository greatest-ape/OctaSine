pub trait CalculateCurve: PartialEq + Copy {
    fn calculate(self, phase: Phase) -> f64;
    fn steps() -> &'static [Self];
}

/// Phase. value >= 0.0 && value < 1.0
#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f64);

impl Default for SampleRate {
    fn default() -> Self {
        Self(44100.0)
    }
}

impl Into<TimePerSample> for SampleRate {
    fn into(self) -> TimePerSample {
        TimePerSample(1.0 / self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimePerSample(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BeatsPerMinute(pub f64);

impl Default for BeatsPerMinute {
    fn default() -> Self {
        Self(120.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Ended,
    Restart,
}
