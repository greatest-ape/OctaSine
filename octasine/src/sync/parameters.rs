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
            Parameter::Master(MasterParameter::Frequency) => {
                Self::new::<MasterFrequencyValue>(name)
            }
            Parameter::Master(MasterParameter::Volume) => Self::new::<MasterVolumeValue>(name),
            Parameter::Operator(_, OperatorParameter::Volume) => {
                Self::new::<OperatorVolumeValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::Active) => {
                Self::new::<OperatorActiveValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::MixOut) => {
                Self::new::<OperatorMixOutValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::Panning) => {
                Self::new::<OperatorPanningValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::WaveType) => {
                Self::new::<OperatorWaveTypeValue>(name)
            }
            Parameter::Operator(1, OperatorParameter::ModTargets) => {
                Self::new::<Operator2ModulationTargetValue>(name)
            }
            Parameter::Operator(2, OperatorParameter::ModTargets) => {
                Self::new::<Operator3ModulationTargetValue>(name)
            }
            Parameter::Operator(3, OperatorParameter::ModTargets) => {
                Self::new::<Operator4ModulationTargetValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::ModTargets) => {
                panic!("Unsupported parameter")
            }
            Parameter::Operator(1..=3, OperatorParameter::ModOut) => {
                Self::new::<OperatorModOutValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::ModOut) => panic!("Unsupported parameter"),
            Parameter::Operator(_, OperatorParameter::Feedback) => {
                Self::new::<OperatorFeedbackValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::FrequencyRatio) => {
                Self::new::<OperatorFrequencyRatioValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::FrequencyFree) => {
                Self::new::<OperatorFrequencyFreeValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::FrequencyFine) => {
                Self::new::<OperatorFrequencyFineValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::AttackDuration) => {
                Self::new::<OperatorAttackDurationValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::AttackValue) => {
                Self::new::<OperatorAttackVolumeValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::DecayDuration) => {
                Self::new::<OperatorDecayDurationValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::DecayValue) => {
                Self::new::<OperatorDecayVolumeValue>(name)
            }
            Parameter::Operator(_, OperatorParameter::ReleaseDuration) => {
                Self::new::<OperatorReleaseDurationValue>(name)
            }
            Parameter::Lfo(0, LfoParameter::Target) => Self::new::<Lfo1TargetParameterValue>(name),
            Parameter::Lfo(1, LfoParameter::Target) => Self::new::<Lfo2TargetParameterValue>(name),
            Parameter::Lfo(2, LfoParameter::Target) => Self::new::<Lfo3TargetParameterValue>(name),
            Parameter::Lfo(3, LfoParameter::Target) => Self::new::<Lfo4TargetParameterValue>(name),
            Parameter::Lfo(_, LfoParameter::Target) => panic!("Unsupported parameter"),
            Parameter::Lfo(_, LfoParameter::BpmSync) => Self::new::<LfoBpmSyncValue>(name),
            Parameter::Lfo(_, LfoParameter::FrequencyRatio) => {
                Self::new::<LfoFrequencyRatioValue>(name)
            }
            Parameter::Lfo(_, LfoParameter::FrequencyFree) => {
                Self::new::<LfoFrequencyFreeValue>(name)
            }
            Parameter::Lfo(_, LfoParameter::Mode) => Self::new::<LfoModeValue>(name),
            Parameter::Lfo(_, LfoParameter::Shape) => Self::new::<LfoShapeValue>(name),
            Parameter::Lfo(_, LfoParameter::Amount) => Self::new::<LfoAmountValue>(name),
            Parameter::Lfo(_, LfoParameter::Active) => Self::new::<LfoActiveValue>(name),
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
