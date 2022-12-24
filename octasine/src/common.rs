use crate::parameters::ParameterKey;

pub const NUM_OPERATORS: usize = 4;
pub const NUM_LFOS: usize = 4;

pub const OPERATOR_MOD_INDEX_STEPS: [f32; 16] = [
    0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0,
];

pub type IndexMap<K, V> = indexmap::IndexMap<K, V, ahash::RandomState>;

pub trait CalculateCurve: PartialEq + Copy {
    fn calculate(self, phase: Phase) -> f32;
    fn steps() -> &'static [Self];
}

/// Phase. value >= 0.0 && value < 1.0
#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl BeatsPerMinute {
    pub fn one_hertz() -> Self {
        Self(60.0)
    }
}

impl Default for BeatsPerMinute {
    fn default() -> Self {
        Self(120.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BpmLfoMultiplier(pub f64);

impl From<BeatsPerMinute> for BpmLfoMultiplier {
    fn from(bpm: BeatsPerMinute) -> Self {
        Self(bpm.0 / 120.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
    Ended,
}

#[derive(Debug, Clone, Copy)]
pub struct NoteEvent {
    pub delta_frames: u32,
    pub event: NoteEventInner,
}

#[derive(Debug, Clone, Copy)]
pub enum NoteEventInner {
    Midi {
        data: [u8; 3],
    },
    ClapNoteOn {
        key: u8,
        velocity: f64,
        clap_note_id: i32,
    },
    ClapNoteOff {
        key: u8,
    },
    ClapBpm {
        bpm: BeatsPerMinute,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum EventToHost {
    StartAutomating(ParameterKey),
    Automate(ParameterKey, f32),
    EndAutomating(ParameterKey),
    RescanValues,
}
