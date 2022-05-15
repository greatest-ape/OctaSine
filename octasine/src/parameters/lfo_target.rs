use super::utils::*;
use super::{LfoParameter, MasterParameter, OperatorParameter, Parameter, ParameterValue};

// When adjusting this, remember to also modify get_lfo_target_parameters
pub const LFO_TARGETS: &[Parameter] = &[
    Parameter::None,
    Parameter::Master(MasterParameter::Volume),
    Parameter::Master(MasterParameter::Frequency),
    Parameter::Operator(0, OperatorParameter::Volume),
    Parameter::Operator(0, OperatorParameter::Panning),
    Parameter::Operator(0, OperatorParameter::MixOut),
    Parameter::Operator(0, OperatorParameter::Feedback),
    Parameter::Operator(0, OperatorParameter::FrequencyRatio),
    Parameter::Operator(0, OperatorParameter::FrequencyFree),
    Parameter::Operator(0, OperatorParameter::FrequencyFine),
    Parameter::Operator(1, OperatorParameter::Volume),
    Parameter::Operator(1, OperatorParameter::Panning),
    Parameter::Operator(1, OperatorParameter::MixOut),
    Parameter::Operator(1, OperatorParameter::ModOut),
    Parameter::Operator(1, OperatorParameter::Feedback),
    Parameter::Operator(1, OperatorParameter::FrequencyRatio),
    Parameter::Operator(1, OperatorParameter::FrequencyFree),
    Parameter::Operator(1, OperatorParameter::FrequencyFine),
    Parameter::Operator(2, OperatorParameter::Volume),
    Parameter::Operator(2, OperatorParameter::Panning),
    Parameter::Operator(2, OperatorParameter::MixOut),
    Parameter::Operator(2, OperatorParameter::ModOut),
    Parameter::Operator(2, OperatorParameter::Feedback),
    Parameter::Operator(2, OperatorParameter::FrequencyRatio),
    Parameter::Operator(2, OperatorParameter::FrequencyFree),
    Parameter::Operator(2, OperatorParameter::FrequencyFine),
    Parameter::Operator(3, OperatorParameter::Volume),
    Parameter::Operator(3, OperatorParameter::Panning),
    Parameter::Operator(3, OperatorParameter::MixOut),
    Parameter::Operator(3, OperatorParameter::ModOut),
    Parameter::Operator(3, OperatorParameter::Feedback),
    Parameter::Operator(3, OperatorParameter::FrequencyRatio),
    Parameter::Operator(3, OperatorParameter::FrequencyFree),
    Parameter::Operator(3, OperatorParameter::FrequencyFine),
    Parameter::Lfo(0, LfoParameter::Shape),
    Parameter::Lfo(0, LfoParameter::Amount),
    Parameter::Lfo(0, LfoParameter::FrequencyRatio),
    Parameter::Lfo(0, LfoParameter::FrequencyFree),
    Parameter::Lfo(1, LfoParameter::Shape),
    Parameter::Lfo(1, LfoParameter::Amount),
    Parameter::Lfo(1, LfoParameter::FrequencyRatio),
    Parameter::Lfo(1, LfoParameter::FrequencyFree),
    Parameter::Lfo(2, LfoParameter::Shape),
    Parameter::Lfo(2, LfoParameter::Amount),
    Parameter::Lfo(2, LfoParameter::FrequencyRatio),
    Parameter::Lfo(2, LfoParameter::FrequencyFree),
];

pub fn get_lfo_target_parameters(lfo_index: usize) -> &'static [Parameter] {
    let end = match lfo_index {
        0 => 34,
        1 => 38,
        2 => 42,
        3 => LFO_TARGETS.len(),
        _ => unreachable!(),
    };

    &LFO_TARGETS[..end]
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo1TargetParameterValue(pub Parameter);

impl Default for Lfo1TargetParameterValue {
    fn default() -> Self {
        Self(Parameter::None)
    }
}

impl ParameterValue for Lfo1TargetParameterValue {
    type Value = Parameter;

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
        self.0.name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo2TargetParameterValue(pub Parameter);

impl Default for Lfo2TargetParameterValue {
    fn default() -> Self {
        Self(Parameter::None)
    }
}

impl ParameterValue for Lfo2TargetParameterValue {
    type Value = Parameter;

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
        self.0.name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo3TargetParameterValue(pub Parameter);

impl Default for Lfo3TargetParameterValue {
    fn default() -> Self {
        Self(Parameter::None)
    }
}

impl ParameterValue for Lfo3TargetParameterValue {
    type Value = Parameter;

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
        self.0.name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo4TargetParameterValue(pub Parameter);

impl Default for Lfo4TargetParameterValue {
    fn default() -> Self {
        Self(Parameter::None)
    }
}

impl ParameterValue for Lfo4TargetParameterValue {
    type Value = Parameter;

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
        self.0.name()
    }
}

#[cfg(test)]
mod tests {
    use super::{get_lfo_target_parameters, Parameter};

    #[test]
    fn test_get_lfo_target_parameters() {
        assert!(!get_lfo_target_parameters(0)
            .iter()
            .any(|t| matches!(t, Parameter::Lfo(_, _))));
        assert!(!get_lfo_target_parameters(1)
            .iter()
            .any(|t| matches!(t, Parameter::Lfo(1.., _))));
        assert!(!get_lfo_target_parameters(2)
            .iter()
            .any(|t| matches!(t, Parameter::Lfo(2.., _))));
    }
}
