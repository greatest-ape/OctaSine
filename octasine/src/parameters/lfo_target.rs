use super::utils::*;
use super::{LfoParameter, MasterParameter, OperatorParameter, Parameter, ParameterValue};

// When adjusting this, remember to also modify get_lfo_target_parameters
pub const LFO_TARGETS: &[LfoTargetParameter] = &[
    LfoTargetParameter::new(Parameter::None),
    LfoTargetParameter::new(Parameter::Master(MasterParameter::Volume)),
    LfoTargetParameter::new(Parameter::Master(MasterParameter::Frequency)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::Volume)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::Panning)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::MixOut)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::Feedback)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Operator(0, OperatorParameter::FrequencyFine)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::Volume)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::Panning)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::MixOut)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::ModOut)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::Feedback)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Operator(1, OperatorParameter::FrequencyFine)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::Volume)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::Panning)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::MixOut)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::ModOut)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::Feedback)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Operator(2, OperatorParameter::FrequencyFine)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::Volume)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::Panning)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::MixOut)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::ModOut)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::Feedback)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Operator(3, OperatorParameter::FrequencyFine)),
    LfoTargetParameter::new(Parameter::Lfo(0, LfoParameter::Shape)),
    LfoTargetParameter::new(Parameter::Lfo(0, LfoParameter::Amount)),
    LfoTargetParameter::new(Parameter::Lfo(0, LfoParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Lfo(0, LfoParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Lfo(1, LfoParameter::Shape)),
    LfoTargetParameter::new(Parameter::Lfo(1, LfoParameter::Amount)),
    LfoTargetParameter::new(Parameter::Lfo(1, LfoParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Lfo(1, LfoParameter::FrequencyFree)),
    LfoTargetParameter::new(Parameter::Lfo(2, LfoParameter::Shape)),
    LfoTargetParameter::new(Parameter::Lfo(2, LfoParameter::Amount)),
    LfoTargetParameter::new(Parameter::Lfo(2, LfoParameter::FrequencyRatio)),
    LfoTargetParameter::new(Parameter::Lfo(2, LfoParameter::FrequencyFree)),
];

pub fn get_lfo_target_parameters(lfo_index: usize) -> &'static [LfoTargetParameter] {
    let end = match lfo_index {
        0 => 34,
        1 => 38,
        2 => 42,
        3 => LFO_TARGETS.len(),
        _ => unreachable!(),
    };

    &LFO_TARGETS[..end]
}

/// Parameter with index stored for performance reasons
#[derive(Debug, Clone, Copy)]
pub struct LfoTargetParameter {
    parameter: Parameter,
    index: Option<u8>,
}

impl PartialEq for LfoTargetParameter {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for LfoTargetParameter {}

impl LfoTargetParameter {
    pub const fn new(parameter: Parameter) -> Self {
        let index = if let Parameter::None = parameter {
            None
        } else {
            Some(parameter.to_index())
        };

        Self { parameter, index }
    }
    pub fn parameter(&self) -> Parameter {
        self.parameter
    }
    pub fn index(&self) -> Option<u8> {
        self.index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo1TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo1TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::new(Parameter::None))
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
        self.0.parameter().name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo2TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo2TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::new(Parameter::None))
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
        self.0.parameter().name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo3TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo3TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::new(Parameter::None))
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
        self.0.parameter().name()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lfo4TargetParameterValue(pub LfoTargetParameter);

impl Default for Lfo4TargetParameterValue {
    fn default() -> Self {
        Self(LfoTargetParameter::new(Parameter::None))
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
        self.0.parameter().name()
    }
}

#[cfg(test)]
mod tests {
    use super::{get_lfo_target_parameters, Parameter};

    #[test]
    fn test_get_lfo_target_parameters() {
        assert!(!get_lfo_target_parameters(0)
            .iter()
            .any(|t| matches!(t.parameter(), Parameter::Lfo(_, _))));
        assert!(!get_lfo_target_parameters(1)
            .iter()
            .any(|t| matches!(t.parameter(), Parameter::Lfo(1.., _))));
        assert!(!get_lfo_target_parameters(2)
            .iter()
            .any(|t| matches!(t.parameter(), Parameter::Lfo(2.., _))));
    }
}
