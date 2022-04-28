use super::utils::*;
use super::ParameterValue;

// When adjusting this, remember to also modify get_lfo_target_parameters
pub const LFO_TARGET_CONTEXT_STEPS: &[LfoTargetParameter] = &[
    LfoTargetParameter::Master(LfoTargetMasterParameter::Volume),
    LfoTargetParameter::Master(LfoTargetMasterParameter::Frequency),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::Volume),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::Panning),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::MixOut),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::Feedback),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::FrequencyRatio),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::FrequencyFree),
    LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::FrequencyFine),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::Volume),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::Panning),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::MixOut),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::ModOut),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::Feedback),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::FrequencyRatio),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::FrequencyFree),
    LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::FrequencyFine),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::Volume),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::Panning),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::MixOut),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::ModOut),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::Feedback),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::FrequencyRatio),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::FrequencyFree),
    LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::FrequencyFine),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::Volume),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::Panning),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::MixOut),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::ModOut),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::Feedback),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::FrequencyRatio),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::FrequencyFree),
    LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::FrequencyFine),
    LfoTargetParameter::Lfo(0, LfoTargetLfoParameter::FrequencyRatio),
    LfoTargetParameter::Lfo(0, LfoTargetLfoParameter::FrequencyFree),
    LfoTargetParameter::Lfo(0, LfoTargetLfoParameter::Shape),
    LfoTargetParameter::Lfo(0, LfoTargetLfoParameter::Amount),
    LfoTargetParameter::Lfo(1, LfoTargetLfoParameter::FrequencyRatio),
    LfoTargetParameter::Lfo(1, LfoTargetLfoParameter::FrequencyFree),
    LfoTargetParameter::Lfo(1, LfoTargetLfoParameter::Shape),
    LfoTargetParameter::Lfo(1, LfoTargetLfoParameter::Amount),
    LfoTargetParameter::Lfo(2, LfoTargetLfoParameter::FrequencyRatio),
    LfoTargetParameter::Lfo(2, LfoTargetLfoParameter::FrequencyFree),
    LfoTargetParameter::Lfo(2, LfoTargetLfoParameter::Shape),
    LfoTargetParameter::Lfo(2, LfoTargetLfoParameter::Amount),
];

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

#[derive(Debug, Clone, Copy)]
pub struct Lfo1TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo1TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo1TargetParameterValue {
    type Value = LfoTargetParameter;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(0),
            value,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(0), self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo2TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo2TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo2TargetParameterValue {
    type Value = LfoTargetParameter;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(1),
            value,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(1), self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo3TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo3TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo3TargetParameterValue {
    type Value = LfoTargetParameter;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(2),
            value,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(2), self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo4TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo4TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    }
}

impl ParameterValue for Lfo4TargetParameterValue {
    type Value = LfoTargetParameter;

    fn new_from_audio(value: Self::Value) -> Self {
        Self(value)
    }
    fn get(self) -> Self::Value {
        self.0
    }
    fn new_from_patch(value: f64) -> Self {
        Self(map_parameter_value_to_step(
            get_lfo_target_parameters(3),
            value,
        ))
    }
    fn to_patch(self) -> f64 {
        map_step_to_parameter_value(get_lfo_target_parameters(3), self.0)
    }
    fn get_formatted(self) -> String {
        self.0.to_string()
    }
}
