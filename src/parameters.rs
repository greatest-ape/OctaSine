use crate::AutomatableState;
use crate::operators::*;


pub trait Parameter {
    fn get_name(&self, state: &AutomatableState) -> String;
    fn get_unit_of_measurement(&self, _: &AutomatableState) -> String {
        "".to_string()
    }

    fn get_value_float(&self, state: &AutomatableState) -> f64;
    fn get_value_text(&self, state: &AutomatableState) -> String {
        format!("{:.2}", self.get_value_float(state))
    }

    fn set_value_float(&self, state: &mut AutomatableState, value: f64);
    fn set_value_text(&self, _: &mut AutomatableState, _: String) -> bool {
        false
    }
}


/// Create parameter for optional parameter field
#[macro_export]
macro_rules! create_operator_opt_field_parameter {
    ($parameter_struct:ident, $field:ident, $field_name:expr) => {
        pub struct $parameter_struct {
            operator_index: usize,
        }

        impl $parameter_struct {
            pub fn new(operator_index: usize) -> Self {
                Self {
                    operator_index: operator_index,
                }
            }
        }

        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Op. {} {}", self.operator_index + 1, $field_name)
            }

            fn get_value_float(&self, state: &AutomatableState) -> f64 {
                if let Some(ref o) = state.operators[self.operator_index].$field {
                    o.get_host_value_float()
                }
                else {
                    error!(
                        "tried to access field {} on operator with index {}",
                        $field_name,
                        self.operator_index
                    );

                    0.0
                }
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                if let Some(ref o ) = state.operators[self.operator_index].$field {
                    o.get_host_value_text().to_owned()
                }
                else {
                    error!(
                        "tried to access field {} on operator with index {}",
                        $field_name,
                        self.operator_index
                    );

                    "error".to_string()
                }
            }

            fn set_value_float(&self, state: &mut AutomatableState, value: f64) {
                if let Some(o) = &mut state.operators[self.operator_index].$field {
                    o.set_host_value_float(value);
                }
            }
        }

    };  
}

/// Create parameter for operator field
#[macro_export]
macro_rules! create_operator_field_parameter {
    ($parameter_struct:ident, $field:ident, $field_name:expr) => {
        pub struct $parameter_struct {
            operator_index: usize,
        }

        impl $parameter_struct {
            pub fn new(operator_index: usize) -> Self {
                Self {
                    operator_index: operator_index,
                }
            }
        }

        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Op. {} {}", self.operator_index + 1, $field_name)
            }

            fn get_value_float(&self, state: &AutomatableState) -> f64 {
                state.operators[self.operator_index].$field.get_host_value_float()
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                state.operators[self.operator_index].$field.get_host_value_text()
            }

            fn set_value_float(&self, state: &mut AutomatableState, value: f64) {
                state.operators[self.operator_index].$field.set_host_value_float(value);
            }
        }
    };  
}


/// Specific macro for volume envelope parameters
/// 
/// I would have preferred to use the normal field macro, but that was
/// difficult with the envelope being inside of its own variable. It might
/// prove useful with envelope-specific features anyway.
#[macro_export]
macro_rules! create_operator_envelope_field_parameter {
    ($parameter_struct:ident, $envelope_field:ident, $field:ident, $field_name:expr) => {
        pub struct $parameter_struct {
            operator_index: usize,
        }

        impl $parameter_struct {
            pub fn new(operator_index: usize) -> Self {
                Self {
                    operator_index: operator_index,
                }
            }
        }

        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Op. {} {}", self.operator_index + 1, $field_name)
            }

            fn get_value_float(&self, state: &AutomatableState) -> f64 {
                state.operators[self.operator_index].$envelope_field.$field.get_host_value_float()
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                state.operators[self.operator_index].$envelope_field.$field.get_host_value_text()
            }

            fn set_value_float(&self, state: &mut AutomatableState, value: f64) {
                state.operators[self.operator_index].$envelope_field.$field.set_host_value_float(value);
            }
        }
    };  
}


create_operator_field_parameter!(
    OperatorVolumeParameter,
    volume,
    "volume"
);

create_operator_field_parameter!(
    OperatorWaveTypeParameter,
    wave_type,
    "wave type"
);

create_operator_opt_field_parameter!(
    OperatorAdditiveFactorParameter,
    additive_factor,
    "additive"
);

create_operator_opt_field_parameter!(
    OperatorOutputOperatorParameter,
    output_operator,
    "mod out"
);

create_operator_field_parameter!(
    OperatorPanningParameter,
    panning,
    "pan"
);

create_operator_field_parameter!(
    OperatorFrequencyRatioParameter,
    frequency_ratio,
    "freq ratio"
);

create_operator_field_parameter!(
    OperatorFrequencyFreeParameter,
    frequency_free,
    "freq free"
);

create_operator_field_parameter!(
    OperatorFrequencyFineParameter,
    frequency_fine,
    "freq fine"
);

create_operator_field_parameter!(
    OperatorFeedbackParameter,
    feedback,
    "feedback"
);

create_operator_field_parameter!(
    OperatorModulationIndexParameter,
    modulation_index,
    "mod index"
);


create_operator_envelope_field_parameter!(
    OperatorVolumeEnvelopeAttackDurationParameter,
    volume_envelope,
    attack_duration,
    "attack time"
);

create_operator_envelope_field_parameter!(
    OperatorVolumeEnvelopeAttackValueParameter,
    volume_envelope,
    attack_end_value,
    "attack vol"
);

create_operator_envelope_field_parameter!(
    OperatorVolumeEnvelopeDecayDurationParameter,
    volume_envelope,
    decay_duration,
    "decay time"
);

create_operator_envelope_field_parameter!(
    OperatorVolumeEnvelopeDecayValueParameter,
    volume_envelope,
    decay_end_value,
    "decay vol"
);

create_operator_envelope_field_parameter!(
    OperatorVolumeEnvelopeReleaseDurationParameter,
    volume_envelope,
    release_duration,
    "release time"
);


pub struct OperatorParameters {
    volume: OperatorVolumeParameter,
    panning: OperatorPanningParameter,
    wave_type: OperatorWaveTypeParameter,
    additive_factor: OperatorAdditiveFactorParameter,
    output_operator: OperatorOutputOperatorParameter,
    modulation_index: OperatorModulationIndexParameter,
    feedback: OperatorFeedbackParameter,
    frequency_ratio: OperatorFrequencyRatioParameter,
    frequency_free: OperatorFrequencyFreeParameter,
    frequency_fine: OperatorFrequencyFineParameter,
    volume_envelope_attack_duration: OperatorVolumeEnvelopeAttackDurationParameter,
    volume_envelope_attack_value: OperatorVolumeEnvelopeAttackValueParameter,
    volume_envelope_decay_duration: OperatorVolumeEnvelopeDecayDurationParameter,
    volume_envelope_decay_value: OperatorVolumeEnvelopeDecayValueParameter,
    volume_envelope_release_duration: OperatorVolumeEnvelopeReleaseDurationParameter,
}

impl OperatorParameters {
    fn new(operator_index: usize) -> Self {
        Self {
            volume: OperatorVolumeParameter::new(operator_index),
            wave_type: OperatorWaveTypeParameter::new(operator_index),
            additive_factor: OperatorAdditiveFactorParameter::new(operator_index),
            output_operator: OperatorOutputOperatorParameter::new(operator_index),
            panning: OperatorPanningParameter::new(operator_index),
            modulation_index: OperatorModulationIndexParameter::new(operator_index),
            feedback: OperatorFeedbackParameter::new(operator_index),
            frequency_ratio: OperatorFrequencyRatioParameter::new(operator_index),
            frequency_free: OperatorFrequencyFreeParameter::new(operator_index),
            frequency_fine: OperatorFrequencyFineParameter::new(operator_index),
            volume_envelope_attack_duration: OperatorVolumeEnvelopeAttackDurationParameter::new(operator_index),
            volume_envelope_attack_value: OperatorVolumeEnvelopeAttackValueParameter::new(operator_index),
            volume_envelope_decay_duration: OperatorVolumeEnvelopeDecayDurationParameter::new(operator_index),
            volume_envelope_decay_value: OperatorVolumeEnvelopeDecayValueParameter::new(operator_index),
            volume_envelope_release_duration: OperatorVolumeEnvelopeReleaseDurationParameter::new(operator_index),
        }
    }
}


pub struct Parameters {
    operator_1: OperatorParameters,
    operator_2: OperatorParameters,
    operator_3: OperatorParameters,
    operator_4: OperatorParameters,
}


impl Parameters {
    pub fn new() -> Self {
        Self {
            operator_1: OperatorParameters::new(0),
            operator_2: OperatorParameters::new(1),
            operator_3: OperatorParameters::new(2),
            operator_4: OperatorParameters::new(3),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Parameter> {
        // This should maybe be generated by a macro
        match index {
            0  => Some(&self.operator_1.volume),
            1  => Some(&self.operator_1.wave_type),
            2  => Some(&self.operator_1.additive_factor), // TODO remove
            3  => Some(&self.operator_1.modulation_index),
            4  => Some(&self.operator_1.feedback),
            5  => Some(&self.operator_1.frequency_ratio),
            6  => Some(&self.operator_1.frequency_free),
            7  => Some(&self.operator_1.frequency_fine),
            8  => Some(&self.operator_1.volume_envelope_attack_duration),
            9  => Some(&self.operator_1.volume_envelope_attack_value),
            10 => Some(&self.operator_1.volume_envelope_decay_duration),
            11 => Some(&self.operator_1.volume_envelope_decay_value),
            12 => Some(&self.operator_1.volume_envelope_release_duration),
            13 => Some(&self.operator_2.volume),
            14 => Some(&self.operator_2.wave_type),
            15 => Some(&self.operator_2.additive_factor),
            16 => Some(&self.operator_2.modulation_index),
            17 => Some(&self.operator_2.feedback),
            18 => Some(&self.operator_2.frequency_ratio),
            19 => Some(&self.operator_2.frequency_free),
            20 => Some(&self.operator_2.frequency_fine),
            21 => Some(&self.operator_2.volume_envelope_attack_duration),
            22 => Some(&self.operator_2.volume_envelope_attack_value),
            23 => Some(&self.operator_2.volume_envelope_decay_duration),
            24 => Some(&self.operator_2.volume_envelope_decay_value),
            25 => Some(&self.operator_2.volume_envelope_release_duration),
            26 => Some(&self.operator_3.volume),
            27 => Some(&self.operator_3.wave_type),
            28 => Some(&self.operator_3.additive_factor),
            29 => Some(&self.operator_3.modulation_index),
            30 => Some(&self.operator_3.feedback),
            31 => Some(&self.operator_3.frequency_ratio),
            32 => Some(&self.operator_3.frequency_free),
            33 => Some(&self.operator_3.frequency_fine),
            34 => Some(&self.operator_3.volume_envelope_attack_duration),
            35 => Some(&self.operator_3.volume_envelope_attack_value),
            36 => Some(&self.operator_3.volume_envelope_decay_duration),
            37 => Some(&self.operator_3.volume_envelope_decay_value),
            38 => Some(&self.operator_3.volume_envelope_release_duration),
            39 => Some(&self.operator_4.volume),
            40 => Some(&self.operator_4.wave_type),
            41 => Some(&self.operator_4.additive_factor),
            42 => Some(&self.operator_4.modulation_index),
            43 => Some(&self.operator_4.feedback),
            44 => Some(&self.operator_4.frequency_ratio),
            45 => Some(&self.operator_4.frequency_free),
            46 => Some(&self.operator_4.frequency_fine),
            47 => Some(&self.operator_4.volume_envelope_attack_duration),
            48 => Some(&self.operator_4.volume_envelope_attack_value),
            49 => Some(&self.operator_4.volume_envelope_decay_duration),
            50 => Some(&self.operator_4.volume_envelope_decay_value),
            51 => Some(&self.operator_4.volume_envelope_release_duration),

            52  => Some(&self.operator_1.panning),
            53  => Some(&self.operator_2.panning),
            54  => Some(&self.operator_3.panning),
            55  => Some(&self.operator_4.panning),

            56  => Some(&self.operator_3.output_operator),
            57  => Some(&self.operator_4.output_operator),
            _  => None
        }
    }

    pub fn len(&self) -> usize {
        52 + 4 + 2
    }
}