use std::f64::consts::FRAC_PI_2;

use crate::common::*;
use crate::constants::*;
use crate::parameters::values::*;

use super::interpolatable_value::*;


pub trait ProcessingParameter {
    type Value;
    type ExtraData;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value;
    fn set_value(&mut self, value: Self::Value);
    fn set_from_sync(&mut self, value: f64);
}


macro_rules! create_interpolatable_processing_parameter {
    ($name:ident, $value_struct:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            value: InterpolatableProcessingValue,
        }

        impl Default for $name {
            fn default() -> Self {
                let default = $value_struct::default().get();

                Self {
                    value: InterpolatableProcessingValue::new(default)
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = f64;
            type ExtraData = TimeCounter;

            fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value {
                self.value.get_value(extra_data, &mut |_| ())
            }
            fn set_value(&mut self, value: Self::Value) {
                self.value.set_value(value)
            }
            fn set_from_sync(&mut self, value: f64){
                self.set_value($value_struct::from_sync(value).get())
            }
        }
    }
}


macro_rules! create_simple_processing_parameter {
    ($name:ident, $value_struct:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub value: <$value_struct as ParameterValue>::Value,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: $value_struct::default().get()
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = <$value_struct as ParameterValue>::Value;
            type ExtraData = ();

            fn get_value(&mut self, _: Self::ExtraData) -> Self::Value {
                self.value
            }
            fn set_value(&mut self, value: Self::Value){
                self.value = value;
            }
            fn set_from_sync(&mut self, value: f64){
                self.set_value($value_struct::from_sync(value).get())
            }
        }
    };
}


// Master volume

create_interpolatable_processing_parameter!(
    ProcessingParameterMasterVolume,
    MasterVolume
);


// Master frequency

create_simple_processing_parameter!(
    ProcessingParameterMasterFrequency,
    MasterFrequency
);


// Operator volume

#[derive(Debug, Clone)]
pub struct ProcessingParameterOperatorVolume {
    value: InterpolatableProcessingValue,
}

impl ProcessingParameterOperatorVolume {
    pub fn new(operator_index: usize) -> Self {
        let value = OperatorVolume::new(operator_index).get();

        Self {
            value: InterpolatableProcessingValue::new(value)
        }
    }
}

impl ProcessingParameter for ProcessingParameterOperatorVolume {
    type Value = f64;
    type ExtraData = TimeCounter;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value {
        self.value.get_value(extra_data, &mut |_| ())
    }
    fn set_value(&mut self, value: Self::Value) {
        self.value.set_value(value)
    }
    fn set_from_sync(&mut self, value: f64){
        self.set_value(OperatorVolume::from_sync(value).get())
    }
}


// Additive factor

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorAdditiveFactor,
    OperatorAdditive
);


// Frequency - ratio

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyRatio,
    OperatorFrequencyRatio
);


// Frequency - free

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyFree,
    OperatorFrequencyFree
);


// Frequency - fine

create_simple_processing_parameter!(
    ProcessingParameterOperatorFrequencyFine,
    OperatorFrequencyFine
);


// Feedback

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorFeedback,
    OperatorFeedback
);


// Modulation index

create_interpolatable_processing_parameter!(
    ProcessingParameterOperatorModulationIndex,
    OperatorModulationIndex
);


// Wave type

create_simple_processing_parameter!(
    ProcessingParameterOperatorWaveType,
    OperatorWaveType
);


// Attack duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorAttackDuration,
    OperatorAttackDuration
);


// Attack volume

create_simple_processing_parameter!(
    ProcessingParameterOperatorAttackVolume,
    OperatorAttackVolume
);


// Decay duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorDecayDuration,
    OperatorDecayDuration
);


// Decay volume

create_simple_processing_parameter!(
    ProcessingParameterOperatorDecayVolume,
    OperatorDecayVolume
);


// Release duration

create_simple_processing_parameter!(
    ProcessingParameterOperatorReleaseDuration,
    OperatorReleaseDuration
);


// Modulation target

create_simple_processing_parameter!(
    ProcessingParameterOperatorModulationTarget2,
    OperatorModulationTarget2
);


create_simple_processing_parameter!(
    ProcessingParameterOperatorModulationTarget3,
    OperatorModulationTarget3
);


#[derive(Debug)]
pub enum ProcessingParameterOperatorModulationTarget {
    OperatorIndex2(ProcessingParameterOperatorModulationTarget2),
    OperatorIndex3(ProcessingParameterOperatorModulationTarget3),
}


impl ProcessingParameterOperatorModulationTarget {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        match operator_index {
            2 => Some(ProcessingParameterOperatorModulationTarget::OperatorIndex2(
                ProcessingParameterOperatorModulationTarget2::default()
            )),
            3 => Some(ProcessingParameterOperatorModulationTarget::OperatorIndex3(
                ProcessingParameterOperatorModulationTarget3::default()
            )),
            _ => None
        }
    }
}


// Panning

#[derive(Debug, Clone)]
pub struct ProcessingParameterOperatorPanning {
    value: InterpolatableProcessingValue,
    pub left_and_right: [f64; 2],
}


impl ProcessingParameterOperatorPanning {
    pub fn calculate_left_and_right(panning: f64) -> [f64; 2] {
        let pan_phase = panning * FRAC_PI_2;

        [pan_phase.cos(), pan_phase.sin()]
    }
}


impl ProcessingParameter for ProcessingParameterOperatorPanning {
    type Value = f64;
    type ExtraData = TimeCounter;

    fn get_value(&mut self, time: Self::ExtraData) -> Self::Value {
        let mut opt_new_left_and_right = None;

        let value = self.value.get_value(time, &mut |new_panning| {
            opt_new_left_and_right =
                Some(Self::calculate_left_and_right(new_panning));
        });

        if let Some(new_left_and_right) = opt_new_left_and_right {
            self.left_and_right = new_left_and_right;
        }

        value
    }
    fn set_value(&mut self, value: Self::Value) {
        self.value.set_value(value)
    }
    fn set_from_sync(&mut self, value: f64) {
        self.set_value(OperatorPanning::from_sync(value).get())
    }
}


impl Default for ProcessingParameterOperatorPanning {
    fn default() -> Self {
        let default = DEFAULT_OPERATOR_PANNING;

        Self {
            value: InterpolatableProcessingValue::new(default),
            left_and_right: Self::calculate_left_and_right(default),
        }
    }
}
