use crate::atomics::atomic_float::AtomicFloat;

use super::super::processing::*;
use super::super::common::*;

use super::common::*;


// Macros


macro_rules! impl_parameter_value_access {
    ($name:ident) => {
        impl SyncParameterValueAccess for $name {
            fn set_value(&self, value: f32) {
                self.value.set(value);
            }
            fn get_value(&self) -> f32 {
                self.value.get()
            }
            fn get_value_if_changed(&self) -> Option<f32> {
                self.value.get_if_changed()
            }
        }
    };
}


macro_rules! impl_value_conversion_from_processing {
    ($name:ident, $other:ident) => {
        impl ParameterValueConversion for $name {
            type ProcessingValue = <$other as ProcessingParameter>::Value;

            fn to_processing(value: f32) -> Self::ProcessingValue {
                $other::to_processing(value)
            }
            fn to_sync(value: Self::ProcessingValue) -> f32 {
                $other::to_sync(value)
            }

            /// Parse a string value coming from the host
            fn parse_string_value(value: String) -> Option<Self::ProcessingValue> {
                $other::parse_string_value(value)
            }

            fn format_processing(internal_value: Self::ProcessingValue) -> String {
                $other::format_processing(internal_value)
            }
        }
    };
}


macro_rules! create_operator_parameter {
    ($name:ident, $parameter_name:expr, $processing_parameter:ident) => {

        #[derive(Debug)]
        pub struct $name {
            value: AtomicFloat,
            operator_index: usize,
        }

        impl $name {
            pub fn new(operator_index: usize) -> Self {
                let value = $processing_parameter::default()
                    .get_sync_target_value();

                Self {
                    value: AtomicFloat::new(value),
                    operator_index: operator_index,
                }
            }
        }

        impl SyncParameterGetName for $name {
            fn get_parameter_name(&self) -> String {
                format!("Op. {} {}", self.operator_index + 1, $parameter_name)
            }
        }

        impl SyncParameterGetUnit for $name {}

        impl_parameter_value_access!($name);

        impl_value_conversion_from_processing!($name, $processing_parameter);
    };  
}


// Master volume

#[derive(Debug)]
pub struct SyncMasterVolume {
    value: AtomicFloat,
}

impl Default for SyncMasterVolume {
    fn default() -> Self {
        let value = ProcessingMasterVolume::default().get_sync_target_value();

        Self {
            value: AtomicFloat::new(value),
        }
    }
}


impl SyncParameterGetName for SyncMasterVolume {
    fn get_parameter_name(&self) -> String {
        "Master volume".to_string()
    }
}

impl SyncParameterGetUnit for SyncMasterVolume {}

impl_parameter_value_access!(SyncMasterVolume);
impl_value_conversion_from_processing!(SyncMasterVolume, ProcessingMasterVolume);


// Master frequency

#[derive(Debug)]
pub struct SyncMasterFrequency {
    pub value: AtomicFloat,
}

impl Default for SyncMasterFrequency {
    fn default() -> Self {
        let value = ProcessingMasterFrequency::default().get_sync_target_value();


        Self {
            value: AtomicFloat::new(value),
        }
    }
}


impl SyncParameterGetName for SyncMasterFrequency {
    fn get_parameter_name(&self) -> String {
        "Master frequency".to_string()
    }
}

impl SyncParameterGetUnit for SyncMasterFrequency {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "Hz".to_string()
    }
}

impl_parameter_value_access!(SyncMasterFrequency);

impl_value_conversion_from_processing!(SyncMasterFrequency, ProcessingMasterFrequency);


// Operator volume

create_operator_parameter!(
    SyncOperatorVolume,
    "volume",
    ProcessingOperatorVolume
);


// Operator modulation target

#[derive(Debug)]
pub enum SyncOperatorModulationTarget {
    OperatorIndex2(SyncOperatorModulationTarget2),
    OperatorIndex3(SyncOperatorModulationTarget3),
}


impl SyncOperatorModulationTarget {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(SyncOperatorModulationTarget::OperatorIndex2(
                SyncOperatorModulationTarget2::new(operator_index)
            )),
            3 => Some(SyncOperatorModulationTarget::OperatorIndex3(
                SyncOperatorModulationTarget3::new(operator_index)
            )),
            _ => None
        }
    }
}


create_operator_parameter!(
    SyncOperatorModulationTarget2,
    "mod out",
    ProcessingOperatorModulationTarget2
);



create_operator_parameter!(
    SyncOperatorModulationTarget3,
    "mod out",
    ProcessingOperatorModulationTarget3
);


// Operator additive factor

create_operator_parameter!(
    SyncOperatorAdditiveFactor,
    "additive",
    ProcessingOperatorAdditiveFactor
);


// Operator panning

create_operator_parameter!(
    SyncOperatorPanning,
    "pan",
    ProcessingOperatorPanning
);


// Operator frequency ratio

create_operator_parameter!(
    SyncOperatorFrequencyRatio,
    "freq ratio",
    ProcessingOperatorFrequencyRatio
);


// Operator free frequency

create_operator_parameter!(
    SyncOperatorFrequencyFree,
    "freq free",
    ProcessingOperatorFrequencyFree
);



// Operator fine frequency

create_operator_parameter!(
    SyncOperatorFrequencyFine,
    "freq fine",
    ProcessingOperatorFrequencyFine
);


// Operator feedback

create_operator_parameter!(
    SyncOperatorFeedback,
    "feedback",
    ProcessingOperatorFeedback
);


// Operator modulation index

create_operator_parameter!(
    SyncOperatorModulationIndex,
    "mod index",
    ProcessingOperatorModulationIndex
);


// Operator wave type

create_operator_parameter!(
    SyncOperatorWaveType,
    "wave type",
    ProcessingOperatorWaveType
);



// Volume envelope attack duration

create_operator_parameter!(
    SyncOperatorAttackDuration,
    "attack time",
    ProcessingOperatorAttackDuration
);


// Volume envelope attack value

create_operator_parameter!(
    SyncOperatorAttackVolume,
    "attack vol",
    ProcessingOperatorAttackVolume
);



// Volume envelope decay duration

create_operator_parameter!(
    SyncOperatorDecayDuration,
    "decay time",
    ProcessingOperatorDecayDuration
);


// Volume envelope decay value

create_operator_parameter!(
    SyncOperatorDecayVolume,
    "decay vol",
    ProcessingOperatorDecayVolume
);


// Volume envelope release duration

create_operator_parameter!(
    SyncOperatorReleaseDuration,
    "release time",
    ProcessingOperatorReleaseDuration
);



#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::common::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_set_volume_text(){
        let p = SyncOperatorVolume::new(3);

        assert!(p.set_parameter_value_text("-1.0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1.2".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 1.2);

        assert!(p.set_parameter_value_text("2.0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 2.0);

        assert!(p.set_parameter_value_text("3.0".to_string()));
        assert_eq!(SyncOperatorVolume::to_processing(p.get_value()), 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let p = SyncOperatorModulationTarget3::new(3);

        assert!(!p.set_parameter_value_text("abc".to_string()));
        assert!(!p.set_parameter_value_text("0".to_string()));
        assert!(!p.set_parameter_value_text("0.5".to_string()));
        assert!(!p.set_parameter_value_text("4".to_string()));

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(SyncOperatorModulationTarget3::to_processing(p.get_value()), 0);

        assert!(p.set_parameter_value_text("2".to_string()));
        assert_eq!(SyncOperatorModulationTarget3::to_processing(p.get_value()), 1);

        assert!(p.set_parameter_value_text("3".to_string()));
        assert_eq!(SyncOperatorModulationTarget3::to_processing(p.get_value()), 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let p = SyncOperatorFrequencyRatio::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), OPERATOR_RATIO_STEPS[0]);

        assert!(p.set_parameter_value_text("10000000.0".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.99".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.5".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        assert!(p.set_parameter_value_text("0.51".to_string()));
        assert_eq!(SyncOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        for step in OPERATOR_RATIO_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_frequency_free_text(){
        let p = SyncOperatorFrequencyFree::new(3);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(SyncOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(SyncOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_approx_eq!(SyncOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS[0]);

        assert!(p.set_parameter_value_text("4.0".to_string()));
        assert_approx_eq!(SyncOperatorFrequencyFree::to_processing(p.get_value()), 4.0);

        assert!(p.set_parameter_value_text("256.0".to_string()));
        assert_approx_eq!(SyncOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS.last().unwrap());

        for step in OPERATOR_FREE_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_wave_type_text(){
        let p = SyncOperatorWaveType::new(3);

        assert!(p.set_parameter_value_text("sine".to_string()));
        assert_eq!(SyncOperatorWaveType::to_processing(p.get_value()), WaveType::Sine);

        assert!(p.set_parameter_value_text("noise".to_string()));
        assert_eq!(SyncOperatorWaveType::to_processing(p.get_value()), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = SyncOperatorAttackDuration::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(SyncOperatorAttackDuration::to_processing(p.get_value()), ENVELOPE_MIN_DURATION);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(SyncOperatorAttackDuration::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("10".to_string()));
        assert_eq!(SyncOperatorAttackDuration::to_processing(p.get_value()),
            ENVELOPE_MAX_DURATION);
    }
}