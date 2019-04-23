use crate::synth::{AutomatableState, Waves};


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
macro_rules! create_wave_field_parameter {
    ($parameter_struct:ident, $field:ident, $field_name:expr) => {
        pub struct $parameter_struct {
            wave_index: usize,
            host_value: f64,
        }

        impl $parameter_struct {
            pub fn get_wave_index(&self) -> usize {
                self.wave_index
            }

            pub fn new(waves: &Waves, wave_index: usize) -> Self {
                Self {
                    wave_index: wave_index,
                    host_value: waves[wave_index].$field.get_default_host_value(),
                }
            }
        }

        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Wave {} {}", self.wave_index + 1, $field_name)
            }

            fn get_value_float(&self, _: &AutomatableState) -> f64 {
                self.host_value
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                format!("{:.2}", state.waves[self.get_wave_index()].$field.0)
            }

            fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
                let transformed = state.waves[
                    self.get_wave_index()
                ].$field.from_host_value(value);

                state.waves[self.get_wave_index()].$field.0 = transformed;

                state.waves[self.get_wave_index()].duration.0 = 0.0;

                self.host_value = value;
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
macro_rules! create_wave_envelope_field_parameter {
    ($parameter_struct:ident, $envelope_field:ident, $field:ident, $field_name:expr) => {
        pub struct $parameter_struct {
            wave_index: usize,
            host_value: f64,
        }

        impl $parameter_struct {
            pub fn get_wave_index(&self) -> usize {
                self.wave_index
            }

            pub fn new(waves: &Waves, wave_index: usize) -> Self {
                Self {
                    wave_index: wave_index,
                    host_value: waves[wave_index].$envelope_field.$field.get_default_host_value(),
                }
            }
        }

        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Wave {} {}", self.wave_index + 1, $field_name)
            }

            fn get_value_float(&self, _: &AutomatableState) -> f64 {
                self.host_value
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                format!("{:.2}", state.waves[self.get_wave_index()].$envelope_field.$field.0)
            }

            fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
                let transformed = state.waves[
                    self.get_wave_index()
                ].$envelope_field.$field.from_host_value(value);

                state.waves[self.get_wave_index()].$envelope_field.$field.0 = transformed;

                state.waves[self.get_wave_index()].duration.0 = 0.0;

                self.host_value = value;
            }
        }
    };  
}


create_wave_field_parameter!(
    WaveVolumeParameter,
    volume,
    "volume"
);

create_wave_field_parameter!(
    WaveMixParameter,
    mix,
    "mix"
);

create_wave_field_parameter!(
    WaveRatioParameter,
    frequency_ratio,
    "freq ratio"
);

create_wave_field_parameter!(
    WaveFrequencyFreeParameter,
    frequency_free,
    "freq free"
);

create_wave_field_parameter!(
    WaveFrequencyFineParameter,
    frequency_fine,
    "freq fine"
);

create_wave_field_parameter!(
    WaveFeedbackParameter,
    feedback,
    "feedback"
);

create_wave_field_parameter!(
    WaveModulationIndexParameter,
    modulation_index,
    "mod index"
);


create_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeAttackDurationParameter,
    volume_envelope,
    attack_duration,
    "attack time"
);

create_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeAttackValueParameter,
    volume_envelope,
    attack_end_value,
    "attack vol"
);

create_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeDecayDurationParameter,
    volume_envelope,
    decay_duration,
    "decay time"
);

create_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeDecayValueParameter,
    volume_envelope,
    decay_end_value,
    "decay vol"
);

create_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeReleaseDurationParameter,
    volume_envelope,
    release_duration,
    "release time"
);