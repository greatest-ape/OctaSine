

/// Number that gets incremented with 1.0 every second
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimeCounter(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct TimePerSample(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct BeatsPerMinute(pub f32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Ended
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    WhiteNoise
}