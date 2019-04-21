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
macro_rules! derive_wave_field_parameter {
    ($parameter_struct:ident, $field:ident, $field_name:expr) => {
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
macro_rules! derive_wave_envelope_field_parameter {
    ($parameter_struct:ident, $envelope_field:ident, $field:ident, $field_name:expr) => {
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


pub struct WaveRatioParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveRatioParameter, ratio, "ratio");


pub struct WaveFrequencyFreeParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveFrequencyFreeParameter, frequency_free, "free");


pub struct WaveFeedbackParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveFeedbackParameter, feedback, "feedback");


/// Frequency modulation index
pub struct WaveBetaParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveBetaParameter, beta, "beta");


pub struct WaveMixParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveMixParameter, mix, "mix");


pub struct WaveVolumeEnvelopeAttackDurationParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_envelope_field_parameter!(
    WaveVolumeEnvelopeAttackDurationParameter,
    volume_envelope,
    attack_duration,
    "vol env atk"
);