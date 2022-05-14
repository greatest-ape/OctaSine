use crate::parameter_values::*;

use super::atomic_double::AtomicPositiveDouble;

pub struct PatchParameter {
    value: AtomicPositiveDouble,
    pub name: String,
    value_from_text: fn(String) -> Option<f64>,
    pub format: fn(f64) -> String,
}

impl PatchParameter {
    pub fn all() -> Vec<Self> {
        PARAMETERS
            .iter()
            .map(PatchParameter::new_from_parameter)
            .collect()
    }

    fn new_from_parameter(parameter: &Parameter) -> Self {
        let name = &parameter.name();

        match parameter {
            Parameter::None => panic!("Attempted to create PatchParameter from Parameter::None"),
            Parameter::Master(p) => match p {
                MasterParameter::Frequency => Self::new::<MasterFrequencyValue>(name),
                MasterParameter::Volume => Self::new::<MasterVolumeValue>(name),
            },
            Parameter::Operator(index, p) => {
                use OperatorParameter::*;

                match p {
                    Volume => Self::new::<OperatorVolumeValue>(name),
                    Active => Self::new::<OperatorActiveValue>(name),
                    MixOut => Self::new::<OperatorMixOutValue>(name),
                    Panning => Self::new::<OperatorPanningValue>(name),
                    WaveType => Self::new::<OperatorWaveTypeValue>(name),
                    Feedback => Self::new::<OperatorFeedbackValue>(name),
                    FrequencyRatio => Self::new::<OperatorFrequencyRatioValue>(name),
                    FrequencyFree => Self::new::<OperatorFrequencyFreeValue>(name),
                    FrequencyFine => Self::new::<OperatorFrequencyFineValue>(name),
                    AttackDuration => Self::new::<OperatorAttackDurationValue>(name),
                    AttackValue => Self::new::<OperatorAttackVolumeValue>(name),
                    DecayDuration => Self::new::<OperatorDecayDurationValue>(name),
                    DecayValue => Self::new::<OperatorDecayVolumeValue>(name),
                    ReleaseDuration => Self::new::<OperatorReleaseDurationValue>(name),
                    ModTargets => match index {
                        1 => Self::new::<Operator2ModulationTargetValue>(name),
                        2 => Self::new::<Operator3ModulationTargetValue>(name),
                        3 => Self::new::<Operator4ModulationTargetValue>(name),
                        _ => panic!("Unsupported parameter"),
                    },
                    ModOut => match index {
                        1 | 2 | 3 => Self::new::<OperatorModOutValue>(name),
                        _ => panic!("Unsupported parameter"),
                    },
                }
            }
            Parameter::Lfo(index, p) => {
                use LfoParameter::*;

                match p {
                    BpmSync => Self::new::<LfoBpmSyncValue>(name),
                    FrequencyRatio => Self::new::<LfoFrequencyRatioValue>(name),
                    FrequencyFree => Self::new::<LfoFrequencyFreeValue>(name),
                    Mode => Self::new::<LfoModeValue>(name),
                    Shape => Self::new::<LfoShapeValue>(name),
                    Amount => Self::new::<LfoAmountValue>(name),
                    Active => Self::new::<LfoActiveValue>(name),
                    Target => match index {
                        0 => Self::new::<Lfo1TargetParameterValue>(name),
                        1 => Self::new::<Lfo2TargetParameterValue>(name),
                        2 => Self::new::<Lfo3TargetParameterValue>(name),
                        3 => Self::new::<Lfo4TargetParameterValue>(name),
                        _ => panic!("Unsupported parameter"),
                    },
                }
            }
        }
    }

    fn new<V: ParameterValue>(name: &str) -> Self {
        Self {
            name: name.to_string(),
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
    fn test_sync_parameters_len() {
        assert!(PatchParameter::all().len() <= MAX_NUM_PARAMETERS);
    }
}
