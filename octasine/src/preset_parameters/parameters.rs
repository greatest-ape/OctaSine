use crate::processing_parameters::*;

use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;
use vst2_helpers::presets::parameters::*;
use vst2_helpers::processing_parameters::*;

use vst2_helpers::*;


// Macros


macro_rules! create_operator_parameter {
    ($name:ident, $parameter_name:expr, $processing_parameter:ident) => {

        #[derive(Debug)]
        pub struct $name {
            value: AtomicPositiveDouble,
            operator_index: usize,
        }

        impl $name {
            pub fn new(operator_index: usize) -> Self {
                let value = $processing_parameter::default()
                    .get_preset_target_value();

                Self {
                    value: AtomicPositiveDouble::new(value),
                    operator_index: operator_index,
                }
            }
        }

        impl PresetParameterGetName for $name {
            fn get_parameter_name(&self) -> String {
                format!("Op. {} {}", self.operator_index + 1, $parameter_name)
            }
        }

        impl PresetParameterGetUnit for $name {}

        impl_preset_parameter_value_access!($name);

        impl_value_conversion_from_processing!($name, $processing_parameter);
    };  
}


// Master volume

#[derive(Debug)]
pub struct PresetParameterMasterVolume {
    value: AtomicPositiveDouble,
}

impl Default for PresetParameterMasterVolume {
    fn default() -> Self {
        let value = ProcessingParameterMasterVolume::default().get_preset_target_value();

        Self {
            value: AtomicPositiveDouble::new(value),
        }
    }
}


impl PresetParameterGetName for PresetParameterMasterVolume {
    fn get_parameter_name(&self) -> String {
        "Master volume".to_string()
    }
}

impl PresetParameterGetUnit for PresetParameterMasterVolume {}

impl_preset_parameter_value_access!(PresetParameterMasterVolume);
impl_value_conversion_from_processing!(PresetParameterMasterVolume, ProcessingParameterMasterVolume);


// Master frequency

#[derive(Debug)]
pub struct PresetParameterMasterFrequency {
    pub value: AtomicPositiveDouble,
}

impl Default for PresetParameterMasterFrequency {
    fn default() -> Self {
        let value = ProcessingParameterMasterFrequency::default().get_preset_target_value();


        Self {
            value: AtomicPositiveDouble::new(value),
        }
    }
}


impl PresetParameterGetName for PresetParameterMasterFrequency {
    fn get_parameter_name(&self) -> String {
        "Master frequency".to_string()
    }
}

impl PresetParameterGetUnit for PresetParameterMasterFrequency {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "Hz".to_string()
    }
}

impl_preset_parameter_value_access!(PresetParameterMasterFrequency);

impl_value_conversion_from_processing!(PresetParameterMasterFrequency, ProcessingParameterMasterFrequency);


// Operator volume

create_operator_parameter!(
    PresetParameterOperatorVolume,
    "volume",
    ProcessingParameterOperatorVolume
);


// Operator modulation target

#[derive(Debug)]
pub enum PresetParameterOperatorModulationTarget {
    OperatorIndex2(PresetParameterOperatorModulationTarget2),
    OperatorIndex3(PresetParameterOperatorModulationTarget3),
}


impl PresetParameterOperatorModulationTarget {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(PresetParameterOperatorModulationTarget::OperatorIndex2(
                PresetParameterOperatorModulationTarget2::new(operator_index)
            )),
            3 => Some(PresetParameterOperatorModulationTarget::OperatorIndex3(
                PresetParameterOperatorModulationTarget3::new(operator_index)
            )),
            _ => None
        }
    }
}


create_operator_parameter!(
    PresetParameterOperatorModulationTarget2,
    "mod out",
    ProcessingParameterOperatorModulationTarget2
);



create_operator_parameter!(
    PresetParameterOperatorModulationTarget3,
    "mod out",
    ProcessingParameterOperatorModulationTarget3
);


// Operator additive factor

create_operator_parameter!(
    PresetParameterOperatorAdditiveFactor,
    "additive",
    ProcessingParameterOperatorAdditiveFactor
);


// Operator panning

create_operator_parameter!(
    PresetParameterOperatorPanning,
    "pan",
    ProcessingParameterOperatorPanning
);


// Operator frequency ratio

create_operator_parameter!(
    PresetParameterOperatorFrequencyRatio,
    "freq ratio",
    ProcessingParameterOperatorFrequencyRatio
);


// Operator free frequency

create_operator_parameter!(
    PresetParameterOperatorFrequencyFree,
    "freq free",
    ProcessingParameterOperatorFrequencyFree
);



// Operator fine frequency

create_operator_parameter!(
    PresetParameterOperatorFrequencyFine,
    "freq fine",
    ProcessingParameterOperatorFrequencyFine
);


// Operator feedback

create_operator_parameter!(
    PresetParameterOperatorFeedback,
    "feedback",
    ProcessingParameterOperatorFeedback
);


// Operator modulation index

create_operator_parameter!(
    PresetParameterOperatorModulationIndex,
    "mod index",
    ProcessingParameterOperatorModulationIndex
);


// Operator wave type

create_operator_parameter!(
    PresetParameterOperatorWaveType,
    "wave type",
    ProcessingParameterOperatorWaveType
);



// Volume envelope attack duration

create_operator_parameter!(
    PresetParameterOperatorAttackDuration,
    "attack time",
    ProcessingParameterOperatorAttackDuration
);


// Volume envelope attack value

create_operator_parameter!(
    PresetParameterOperatorAttackVolume,
    "attack vol",
    ProcessingParameterOperatorAttackVolume
);



// Volume envelope decay duration

create_operator_parameter!(
    PresetParameterOperatorDecayDuration,
    "decay time",
    ProcessingParameterOperatorDecayDuration
);


// Volume envelope decay value

create_operator_parameter!(
    PresetParameterOperatorDecayVolume,
    "decay vol",
    ProcessingParameterOperatorDecayVolume
);


// Volume envelope release duration

create_operator_parameter!(
    PresetParameterOperatorReleaseDuration,
    "release time",
    ProcessingParameterOperatorReleaseDuration
);



#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::common::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_set_volume_text(){
        let p = PresetParameterOperatorVolume::new(3);

        assert!(p.set_parameter_value_text("-1.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1.2".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 1.2);

        assert!(p.set_parameter_value_text("2.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 2.0);

        assert!(p.set_parameter_value_text("3.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let p = PresetParameterOperatorModulationTarget3::new(3);

        assert!(!p.set_parameter_value_text("abc".to_string()));
        assert!(!p.set_parameter_value_text("0".to_string()));
        assert!(!p.set_parameter_value_text("0.5".to_string()));
        assert!(!p.set_parameter_value_text("4".to_string()));

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 0);

        assert!(p.set_parameter_value_text("2".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 1);

        assert!(p.set_parameter_value_text("3".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let p = PresetParameterOperatorFrequencyRatio::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), OPERATOR_RATIO_STEPS[0]);

        assert!(p.set_parameter_value_text("10000000.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.99".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.5".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        assert!(p.set_parameter_value_text("0.51".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        for step in OPERATOR_RATIO_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_frequency_free_text(){
        let p = PresetParameterOperatorFrequencyFree::new(3);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS[0]);

        assert!(p.set_parameter_value_text("4.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 4.0);

        assert!(p.set_parameter_value_text("256.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS.last().unwrap());

        for step in OPERATOR_FREE_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_wave_type_text(){
        let p = PresetParameterOperatorWaveType::new(3);

        assert!(p.set_parameter_value_text("sine".to_string()));
        assert_eq!(PresetParameterOperatorWaveType::to_processing(p.get_value()), WaveType::Sine);

        assert!(p.set_parameter_value_text("noise".to_string()));
        assert_eq!(PresetParameterOperatorWaveType::to_processing(p.get_value()), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = PresetParameterOperatorAttackDuration::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()), ENVELOPE_MIN_DURATION);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("10".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()),
            ENVELOPE_MAX_DURATION);
    }
}