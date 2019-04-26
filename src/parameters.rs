use crate::synth::AutomatableState;
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

    fn set_value_float(&mut self, state: &mut AutomatableState, value: f64);
    fn set_value_text(&mut self, _: &mut AutomatableState, _: String) -> bool {
        false
    }
}


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

            fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
                state.operators[self.operator_index].$field.set_host_value_float(value);

                state.operators[self.operator_index].duration.0 = 0.0;
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

            fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
                state.operators[self.operator_index].$envelope_field.$field.set_host_value_float(value);

                state.operators[self.operator_index].duration.0 = 0.0;
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
    OperatorSkipChainFactorParameter,
    skip_chain_factor,
    "skip chain"
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