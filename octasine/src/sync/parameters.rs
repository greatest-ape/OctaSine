use compact_str::CompactString;

use crate::{
    common::IndexMap,
    parameters::{velocity_sensitivity::VelocitySensitivityValue, *},
};

use super::atomic_float::AtomicFloat;

/// Thread-safe storage of parameter values in patch format (f64 in range 0.0
/// to 1.0)
pub struct PatchParameter {
    value: AtomicFloat,
    pub name: CompactString,
    pub value_from_text: fn(&str) -> Option<f32>,
    pub format: fn(f32) -> CompactString,
    pub get_serializable: fn(f32) -> SerializableRepresentation,
    pub text_choices: Option<Vec<CompactString>>,
    pub default_value: f32,
    pub clap_path: CompactString,
    pub parameter: WrappedParameter,
}

impl PatchParameter {
    pub fn all() -> IndexMap<ParameterKey, Self> {
        PARAMETERS
            .iter()
            .copied()
            .map(|p| {
                let p: WrappedParameter = p.into();

                (p.key(), PatchParameter::new_from_parameter(p))
            })
            .collect()
    }

    fn new_from_parameter(parameter: WrappedParameter) -> Self {
        match parameter.parameter() {
            Parameter::None => panic!("Attempted to create PatchParameter from Parameter::None"),
            Parameter::Master(master_parameter) => match master_parameter {
                MasterParameter::Frequency => Self::new::<MasterFrequencyValue>(parameter),
                MasterParameter::Volume => Self::new::<MasterVolumeValue>(parameter),
                MasterParameter::PitchBendRangeUp => {
                    Self::new::<MasterPitchBendRangeUpValue>(parameter)
                }
                MasterParameter::PitchBendRangeDown => {
                    Self::new::<MasterPitchBendRangeDownValue>(parameter)
                }
                MasterParameter::VelocitySensitivityVolume => {
                    Self::new::<VelocitySensitivityValue>(parameter)
                }
            },
            Parameter::Operator(index, operator_parameter) => {
                use OperatorParameter::*;

                match operator_parameter {
                    Volume => Self::new::<OperatorVolumeValue>(parameter),
                    Active => Self::new::<OperatorActiveValue>(parameter),
                    MixOut => {
                        Self::new_with_value(parameter, OperatorMixOutValue::new(index as usize))
                    }
                    Panning => Self::new::<OperatorPanningValue>(parameter),
                    WaveType => Self::new::<OperatorWaveTypeValue>(parameter),
                    Feedback => Self::new::<OperatorFeedbackValue>(parameter),
                    FrequencyRatio => Self::new::<OperatorFrequencyRatioValue>(parameter),
                    FrequencyFree => Self::new::<OperatorFrequencyFreeValue>(parameter),
                    FrequencyFine => Self::new::<OperatorFrequencyFineValue>(parameter),
                    AttackDuration => Self::new::<OperatorAttackDurationValue>(parameter),
                    DecayDuration => Self::new::<OperatorDecayDurationValue>(parameter),
                    SustainVolume => Self::new::<OperatorSustainVolumeValue>(parameter),
                    ReleaseDuration => Self::new::<OperatorReleaseDurationValue>(parameter),
                    EnvelopeLockGroup => Self::new::<OperatorEnvelopeGroupValue>(parameter),
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
                    VelocitySensitivityFeedback | VelocitySensitivityModOut => {
                        Self::new::<VelocitySensitivityValue>(parameter)
                    }
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
                    KeySync => Self::new::<LfoKeySyncValue>(parameter),
                }
            }
        }
    }

    fn new<V: ParameterValue>(parameter: WrappedParameter) -> Self {
        Self {
            name: parameter.parameter().name(),
            value: AtomicFloat::new(V::default().to_patch()),
            value_from_text: |v| V::new_from_text(v).map(|v| v.to_patch()),
            format: |v| V::new_from_patch(v).get_formatted(),
            get_serializable: |v| V::new_from_patch(v).get_serializable(),
            text_choices: V::get_text_choices(),
            default_value: V::default().to_patch(),
            clap_path: parameter.parameter().clap_path(),
            parameter,
        }
    }
    fn new_with_value<V: ParameterValue>(parameter: WrappedParameter, v: V) -> Self {
        let p = Self::new::<V>(parameter);

        p.value.set(v.to_patch());

        p
    }

    pub fn set_value(&self, value: f32) {
        self.value.set(value);
    }

    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    pub fn get_value_text(&self) -> CompactString {
        (self.format)(self.value.get())
    }

    pub fn get_serializable(&self) -> SerializableRepresentation {
        (self.get_serializable)(self.value.get())
    }

    pub fn set_from_text(&self, text: &str) -> bool {
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
