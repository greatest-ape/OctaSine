use crate::constants::LFO_TARGET_CONTEXT_STEPS;

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
    Shape,
    FrequencyRatio,
    FrequencyFree,
    Magnitude,
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
                use LfoTargetLfoParameter::*;

                let p = match p {
                    Shape => "shape",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    Magnitude => "magnitude",
                };

                format!("LFO {} {}", n + 1, p)
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoShape {
    LinearUp,
    LinearDown,
    Triangle,
    Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoMode {
    Once,
    Forever
}


pub fn get_lfo_target_parameters(lfo_index: usize) -> &'static [LfoTargetParameter] {
    let end = match lfo_index {
        0 => 33,
        1 => 37,
        2 => 41,
        3 => 45,
        _ => unreachable!(),
    };

    &LFO_TARGET_CONTEXT_STEPS[..end]
}
