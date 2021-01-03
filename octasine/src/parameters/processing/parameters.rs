use std::f64::consts::FRAC_PI_2;

use crate::common::*;
use crate::constants::*;
use crate::parameters::values::*;

use super::interpolatable_value::*;


pub trait ProcessingParameter {
    type Value;
    type ExtraData;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value;
    fn set_from_sync(&mut self, value: f64);
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value;
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
            fn set_from_sync(&mut self, value: f64){
                self.value.set_value($value_struct::from_sync(value).get())
            }
            fn get_value_with_lfo_addition(
                &mut self,
                extra_data: Self::ExtraData,
                lfo_addition: Option<f64>
            ) -> Self::Value {
                if let Some(lfo_addition) = lfo_addition {
                    let sync_value = $value_struct::from_processing(
                        self.get_value(extra_data)
                    ).to_sync();
        
                    $value_struct::from_sync(
                        (sync_value + lfo_addition).min(1.0).max(0.0)
                    ).get()
                } else {
                    self.get_value(extra_data)
                }   
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
            fn set_from_sync(&mut self, value: f64){
                self.value = $value_struct::from_sync(value).get();
            }
            fn get_value_with_lfo_addition(
                &mut self,
                extra_data: Self::ExtraData,
                lfo_addition: Option<f64>
            ) -> Self::Value {
                if let Some(lfo_addition) = lfo_addition {
                    let sync_value = $value_struct::from_processing(
                        self.get_value(extra_data)
                    ).to_sync();
        
                    $value_struct::from_sync(
                        (sync_value + lfo_addition).min(1.0).max(0.0)
                    ).get()
                } else {
                    self.get_value(extra_data)
                }   
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
    fn set_from_sync(&mut self, value: f64){
        self.value.set_value(OperatorVolume::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = OperatorVolume::from_processing(
                self.get_value(extra_data)
            ).to_sync();

            OperatorVolume::from_sync(
                (sync_value + lfo_addition).min(1.0).max(0.0)
            ).get()
        } else {
            self.get_value(extra_data)
        }   
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
    fn set_from_sync(&mut self, value: f64) {
        self.value.set_value(OperatorPanning::from_sync(value).get())
    }
    fn get_value_with_lfo_addition(
        &mut self,
        extra_data: Self::ExtraData,
        lfo_addition: Option<f64>
    ) -> Self::Value {
        if let Some(lfo_addition) = lfo_addition {
            let sync_value = OperatorPanning::from_processing(
                self.get_value(extra_data)
            ).to_sync();

            OperatorPanning::from_sync(
                (sync_value + lfo_addition).min(1.0).max(0.0)
            ).get()
        } else {
            self.get_value(extra_data)
        }   
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


// LFO target parameter

create_simple_processing_parameter!(
    ProcessingParameterLfo1TargetParameter,
    Lfo1TargetParameterValue
);
create_simple_processing_parameter!(
    ProcessingParameterLfo2TargetParameter,
    Lfo2TargetParameterValue
);
create_simple_processing_parameter!(
    ProcessingParameterLfo3TargetParameter,
    Lfo3TargetParameterValue
);
create_simple_processing_parameter!(
    ProcessingParameterLfo4TargetParameter,
    Lfo4TargetParameterValue
);


pub enum ProcessingParameterLfoTargetParameter {
    One(ProcessingParameterLfo1TargetParameter),
    Two(ProcessingParameterLfo2TargetParameter),
    Three(ProcessingParameterLfo3TargetParameter),
    Four(ProcessingParameterLfo4TargetParameter),
}

impl ProcessingParameterLfoTargetParameter {
    pub fn new(lfo_index: usize) -> Self {
        match lfo_index {
            0 => Self::One(ProcessingParameterLfo1TargetParameter::default()),
            1 => Self::Two(ProcessingParameterLfo2TargetParameter::default()),
            2 => Self::Three(ProcessingParameterLfo3TargetParameter::default()),
            3 => Self::Four(ProcessingParameterLfo4TargetParameter::default()),
            _ => unreachable!(),
        }
    }

    pub fn set_from_sync(&mut self, value: f64){
        match self {
            Self::One(p) => p.set_from_sync(value),
            Self::Two(p) => p.set_from_sync(value),
            Self::Three(p) => p.set_from_sync(value),
            Self::Four(p) => p.set_from_sync(value),
        }
    }

    pub fn get_value(&self) -> LfoTargetParameter {
        match self {
            Self::One(p) => p.value,
            Self::Two(p) => p.value,
            Self::Three(p) => p.value,
            Self::Four(p) => p.value,
        }
    }
}


// LFO shape

create_simple_processing_parameter!(
    ProcessingParameterLfoShape,
    LfoShapeValue
);



// LFO mode

create_simple_processing_parameter!(
    ProcessingParameterLfoMode,
    LfoModeValue
);



// LFO bpm sync

create_simple_processing_parameter!(
    ProcessingParameterLfoBpmSync,
    LfoBpmSyncValue
);


// LFO speed

create_simple_processing_parameter!(
    ProcessingParameterLfoSpeed,
    LfoSpeedValue
);


// LFO magnitude

create_interpolatable_processing_parameter!(
    ProcessingParameterLfoMagnitude,
    LfoMagnitudeValue
);