use crate::constants::LFO_TARGET_CONTEXT_STEPS;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModTargets<const N: usize>([bool; N]);

impl ModTargets<1> {
    pub fn permutations() -> &'static [Self] {
        &[ModTargets([true]), ModTargets([false])]
    }
}

impl ModTargets<2> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargets([false, false]),
            ModTargets([true, false]),
            ModTargets([false, true]),
            ModTargets([true, true]),
        ]
    }
}

impl ModTargets<3> {
    pub fn permutations() -> &'static [Self] {
        &[
            ModTargets([true, false, false]),
            ModTargets([true, true, false]),
            ModTargets([true, false, true]),
            ModTargets([true, true, true]),
            ModTargets([false, true, false]),
            ModTargets([false, false, true]),
            ModTargets([false, true, true]),
            ModTargets([false, false, true]),
            ModTargets([false, false, false]),
        ]
    }
}

impl Default for ModTargets<1> {
    fn default() -> Self {
        Self([true])
    }
}

impl Default for ModTargets<2> {
    fn default() -> Self {
        Self([false, true])
    }
}

impl Default for ModTargets<3> {
    fn default() -> Self {
        Self([false, false, true])
    }
}

impl<const N: usize> ModTargets<N> {
    pub fn set_index(&mut self, index: usize, value: bool) {
        self.0[index] = value;
    }

    pub fn index_active(&self, index: usize) -> bool {
        self.0[index]
    }

    pub fn as_string(&self) -> String {
        let mut output = String::new();

        for (index, active) in self.0.into_iter().enumerate() {
            if active {
                let operator_number = index + 1;

                if output.is_empty() {
                    output.push_str(&format!("{}", operator_number))
                } else {
                    output.push_str(&format!(", {}", operator_number))
                }
            }
        }

        output
    }

    pub fn as_iter(&self) -> Box<dyn Iterator<Item = bool>> {
        Box::new(self.0.into_iter())
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    WhiteNoise,
}

impl Default for WaveType {
    fn default() -> Self {
        Self::Sine
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetMasterParameter {
    Volume,
    Frequency,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetOperatorParameter {
    Volume,
    Panning,
    MixOut,
    ModOut,
    Feedback,
    FrequencyRatio,
    FrequencyFree,
    FrequencyFine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetLfoParameter {
    Shape,
    FrequencyRatio,
    FrequencyFree,
    Amount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LfoTargetParameter {
    Master(LfoTargetMasterParameter),
    Operator(usize, LfoTargetOperatorParameter),
    Lfo(usize, LfoTargetLfoParameter),
}

impl std::fmt::Display for LfoTargetParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LfoTargetParameter::Master(p) => {
                write!(f, "Master {}", format!("{:?}", p).to_lowercase())
            }
            LfoTargetParameter::Operator(n, p) => {
                use LfoTargetOperatorParameter::*;

                let p = match p {
                    Volume => "volume",
                    Panning => "pan",
                    MixOut => "mix out",
                    ModOut => "mod out",
                    Feedback => "feedback",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    FrequencyFine => "freq fine",
                };

                write!(f, "Op. {} {}", n + 1, p)
            }
            LfoTargetParameter::Lfo(n, p) => {
                use LfoTargetLfoParameter::*;

                let p = match p {
                    Shape => "shape",
                    FrequencyRatio => "freq ratio",
                    FrequencyFree => "freq free",
                    Amount => "amount",
                };

                write!(f, "LFO {} {}", n + 1, p)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoShape {
    Saw,
    ReverseSaw,
    Triangle,
    ReverseTriangle,
    Square,
    ReverseSquare,
    Sine,
    ReverseSine,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LfoMode {
    Once,
    Forever,
}

pub fn get_lfo_target_parameters(lfo_index: usize) -> &'static [LfoTargetParameter] {
    let end = match lfo_index {
        0 => 29,
        1 => 33,
        2 => 37,
        3 => 41,
        _ => unreachable!(),
    };

    &LFO_TARGET_CONTEXT_STEPS[..end]
}
