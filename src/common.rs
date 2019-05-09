

/// Number that gets incremented with 1.0 every second
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimeCounter(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct MasterFrequency(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct TimePerSample(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct BeatsPerMinute(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Ended
}

#[derive(Debug, Copy, Clone)]
pub enum WaveType {
    Sine,
    WhiteNoise
}