use crate::parameters::*;

use super::atomic_double::AtomicPositiveDouble;

/// Thread-safe storage of parameter values in patch format (f64 in range 0.0
/// to 1.0)
pub struct PatchParameter {
    parameter: Parameter,
    value: AtomicPositiveDouble,
    pub name: String,
    value_from_text: fn(String) -> Option<f64>,
    pub format: fn(f64) -> String,
}

impl PatchParameter {
    pub fn all() -> Vec<Self> {
        PARAMETERS
            .iter()
            .copied()
            .map(PatchParameter::new_from_parameter)
            .collect()
    }

    fn new_from_parameter(parameter: Parameter) -> Self {
        match parameter {
            Parameter::None => panic!("Attempted to create PatchParameter from Parameter::None"),
            Parameter::Master(master_parameter) => match master_parameter {
                MasterParameter::Frequency => Self::new::<MasterFrequencyValue>(parameter),
                MasterParameter::Volume => Self::new::<MasterVolumeValue>(parameter),
            },
            Parameter::Operator(index, operator_parameter) => {
                use OperatorParameter::*;

                match operator_parameter {
                    Volume => Self::new::<OperatorVolumeValue>(parameter),
                    Active => Self::new::<OperatorActiveValue>(parameter),
                    MixOut => Self::new::<OperatorMixOutValue>(parameter),
                    Panning => Self::new::<OperatorPanningValue>(parameter),
                    WaveType => Self::new::<OperatorWaveTypeValue>(parameter),
                    Feedback => Self::new::<OperatorFeedbackValue>(parameter),
                    FrequencyRatio => Self::new::<OperatorFrequencyRatioValue>(parameter),
                    FrequencyFree => Self::new::<OperatorFrequencyFreeValue>(parameter),
                    FrequencyFine => Self::new::<OperatorFrequencyFineValue>(parameter),
                    AttackDuration => Self::new::<OperatorAttackDurationValue>(parameter),
                    AttackValue => Self::new::<OperatorAttackVolumeValue>(parameter),
                    DecayDuration => Self::new::<OperatorDecayDurationValue>(parameter),
                    DecayValue => Self::new::<OperatorDecayVolumeValue>(parameter),
                    ReleaseDuration => Self::new::<OperatorReleaseDurationValue>(parameter),
                    ModTargets => match index {
                        1 => Self::new::<Operator2ModulationTargetValue>(parameter),
                        2 => Self::new::<Operator3ModulationTargetValue>(parameter),
                        3 => Self::new::<Operator4ModulationTargetValue>(parameter),
                        _ => panic!("Unsupported parameter"),
                    },
                    ModOut => match index {
                        1 | 2 | 3 => Self::new::<OperatorModOutValue>(parameter),
                        _ => panic!("Unsupported parameter"),
                    },
                }
            }
            Parameter::Lfo(index, lfo_parameter) => {
                use LfoParameter::*;

                match lfo_parameter {
                    BpmSync => Self::new::<LfoBpmSyncValue>(parameter),
                    FrequencyRatio => Self::new::<LfoFrequencyRatioValue>(parameter),
                    FrequencyFree => Self::new::<LfoFrequencyFreeValue>(parameter),
                    Mode => Self::new::<LfoModeValue>(parameter),
                    Shape => Self::new::<LfoShapeValue>(parameter),
                    Amount => Self::new::<LfoAmountValue>(parameter),
                    Active => Self::new::<LfoActiveValue>(parameter),
                    Target => match index {
                        0 => Self::new::<Lfo1TargetParameterValue>(parameter),
                        1 => Self::new::<Lfo2TargetParameterValue>(parameter),
                        2 => Self::new::<Lfo3TargetParameterValue>(parameter),
                        3 => Self::new::<Lfo4TargetParameterValue>(parameter),
                        _ => panic!("Unsupported parameter"),
                    },
                }
            }
        }
    }

    fn new<V: ParameterValue>(parameter: Parameter) -> Self {
        Self {
            parameter,
            name: parameter.name(),
            value: AtomicPositiveDouble::new(V::default().to_patch()),
            value_from_text: |v| V::new_from_text(v).map(|v| v.to_patch()),
            format: |v| V::new_from_patch(v).get_formatted(),
        }
    }

    pub fn set_value(&self, value: f64) {
        self.value.set(value);
    }

    pub fn get_value(&self) -> f64 {
        self.value.get()
    }

    pub fn get_value_text(&self) -> String {
        (self.format)(self.value.get())
    }

    pub fn set_from_text(&self, text: String) -> bool {
        if let Some(value) = (self.value_from_text)(text) {
            self.value.set(value);

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::change_info::MAX_NUM_PARAMETERS;

    use super::PatchParameter;

    #[test]
    fn test_patch_parameters_len() {
        assert!(PatchParameter::all().len() <= MAX_NUM_PARAMETERS);
    }
}
