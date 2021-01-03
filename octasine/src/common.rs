/// Number that gets incremented with 1.0 every second
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimeCounter(pub f64);

/// Phase. value >= 0.0 && value < 1.0
#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f64);

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
    Ended
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    WhiteNoise
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetMasterParameter {
    Volume,
    Frequency
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetOperatorParameter {
    Volume,
    Panning,
    Additive,
    ModulationIndex,
    Feedback,
    FrequencyRatio,
    FrequencyFree,
    FrequencyFine
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetLfoParameter {
    Magnitude,
    Speed,
    Shape
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetParameter {
    Master(LfoTargetMasterParameter),
    Operator(usize, LfoTargetOperatorParameter),
    Lfo(usize, LfoTargetLfoParameter),
}


impl LfoTargetParameter {
    pub fn to_string(&self) -> String {
        match self {
            LfoTargetParameter::Master(p) => {
                format!("Master {}", format!("{:?}", p).to_lowercase())
            },
            LfoTargetParameter::Operator(n, p) => {
                use LfoTargetOperatorParameter::*;

                let p = match p {
                    Volume => "vol",
                    Panning => "pan",
                    Additive => "additive",
                    ModulationIndex => "mod index",
                    Feedback => "feedback",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    FrequencyFine => "freq fine",
                };

                format!("Op. {} {}", n + 1, p)
            },
            LfoTargetParameter::Lfo(n, p) => {
                format!(
                    "LFO {} {}",
                    n + 1,
                    format!("{:?}", p).to_lowercase()
                )
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoShape {
    LinearUp,
    LinearDown,
    Triangle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoMode {
    Once,
    Forever
}