use crate::common::SampleRate;
use crate::parameters::{
    ModTargetStorage, Operator2ModulationTargetValue, Operator3ModulationTargetValue,
    Operator4ModulationTargetValue,
};

use super::common::{AudioParameter, SimpleAudioParameter};
use super::AudioParameterPatchInteraction;

pub enum OperatorModulationTargetAudioParameter {
    Two(SimpleAudioParameter<Operator2ModulationTargetValue>),
    Three(SimpleAudioParameter<Operator3ModulationTargetValue>),
    Four(SimpleAudioParameter<Operator4ModulationTargetValue>),
}

impl OperatorModulationTargetAudioParameter {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            1 => Some(OperatorModulationTargetAudioParameter::Two(
                Default::default(),
            )),
            2 => Some(OperatorModulationTargetAudioParameter::Three(
                Default::default(),
            )),
            3 => Some(OperatorModulationTargetAudioParameter::Four(
                Default::default(),
            )),
            _ => None,
        }
    }

    pub fn get_value(&self) -> ModTargetStorage {
        match self {
            Self::Two(p) => p.get_value(),
            Self::Three(p) => p.get_value(),
            Self::Four(p) => p.get_value(),
        }
    }

    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        match self {
            Self::Two(p) => p.advance_one_sample(sample_rate),
            Self::Three(p) => p.advance_one_sample(sample_rate),
            Self::Four(p) => p.advance_one_sample(sample_rate),
        }
    }
}

impl AudioParameterPatchInteraction for OperatorModulationTargetAudioParameter {
    fn set_patch_value(&mut self, value: f64) {
        match self {
            Self::Two(p) => p.set_from_patch(value),
            Self::Three(p) => p.set_from_patch(value),
            Self::Four(p) => p.set_from_patch(value),
        }
    }

    #[cfg(test)]
    fn compare_patch_value(&mut self, value: f64) -> bool {
        use crate::parameters::ParameterValue;

        let a = match self {
            Self::Two(_) => Operator2ModulationTargetValue::new_from_patch(value).to_patch(),
            Self::Three(_) => Operator3ModulationTargetValue::new_from_patch(value).to_patch(),
            Self::Four(_) => Operator4ModulationTargetValue::new_from_patch(value).to_patch(),
        };

        let b = match self {
            Self::Two(p) => p.get_parameter_value().to_patch(),
            Self::Three(p) => p.get_parameter_value().to_patch(),
            Self::Four(p) => p.get_parameter_value().to_patch(),
        };

        a == b
    }
}
