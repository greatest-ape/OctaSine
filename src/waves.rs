use crate::constants::*;
use crate::common::*;
use crate::utils::*;


#[derive(Debug, Copy, Clone)]
pub struct WaveDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct WaveStepData {
    pub step_size: f64,
    pub steps_remaining: usize,
    pub last_time: NoteTime,
    pub num_steps: usize
}

impl Default for WaveStepData {
    fn default() -> Self {
        Self {
            step_size: 0.0,
            steps_remaining: 0,
            last_time: NoteTime(0.0),
            num_steps: 32,
        }
    }
}

pub trait InterpolatableValue {
    fn get_value(&mut self, time: NoteTime) -> f64;
    fn set_value(&mut self, value: f64);
}


pub trait AutomatableValue {
    fn set_host_value_float(&mut self, value: f64);
    fn get_host_value_float(&self) -> f64;
    fn get_host_value_text(&self) -> String;
}


#[macro_export]
macro_rules! create_wave_value {
    ($struct_name:ident, $default_value:ident) => {

        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            current_value: f64,
            target_value: f64,
            step_data: WaveStepData,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    current_value: $default_value,
                    target_value: $default_value,
                    step_data: WaveStepData::default(),
                }
            }
        }

        impl InterpolatableValue for $struct_name {
            fn get_value(&mut self, time: NoteTime) -> f64 {
                if self.step_data.num_steps == 0 {
                    return self.current_value;
                }

                if time != self.step_data.last_time && self.step_data.steps_remaining > 0 {
                    self.current_value += self.step_data.step_size;
                    self.step_data.steps_remaining -= 1;
                    self.step_data.last_time = time;
                }

                self.current_value
            }

            fn set_value(&mut self, value: f64){
                self.target_value = value;

                if self.step_data.num_steps == 0 {
                    self.current_value = value;

                    return;
                }

                if value == self.current_value {
                    self.step_data.steps_remaining = 0;
                }
                else {
                    // Restart stepping process
                    let diff = value - self.current_value;
                    self.step_data.step_size = diff / self.step_data.num_steps as f64;
                    self.step_data.steps_remaining = self.step_data.num_steps;
                }
            }
        }

        impl AutomatableValue for $struct_name {
            fn set_host_value_float(&mut self, value: f64){
                self.set_value(self.from_host_value(value));
            }
            fn get_host_value_float(&self) -> f64 {
                self.to_host_value(self.target_value)
            }
            fn get_host_value_text(&self) -> String {
                format!("{:.2}", self.target_value)
            }
        }
    };  
}



create_wave_value!(WaveVolume, WAVE_DEFAULT_VOLUME);

impl WaveVolume {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        value / 2.0
    }
}


create_wave_value!(WaveSkipChainFactor, WAVE_DEFAULT_SKIP_CHAIN_FACTOR);

impl WaveSkipChainFactor {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        value
    }
}


create_wave_value!(WaveFrequencyRatio, WAVE_DEFAULT_FREQUENCY_RATIO);

impl WaveFrequencyRatio {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step(&WAVE_RATIO_STEPS[..], value)
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        get_host_value_for_step(&WAVE_RATIO_STEPS[..], value)
    }
}


create_wave_value!(WaveFrequencyFree, WAVE_DEFAULT_FREQUENCY_FREE);

impl WaveFrequencyFree {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).powf(3.0)
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        value.powf(1.0/3.0) - 0.5
    }
}


create_wave_value!(WaveFrequencyFine, WAVE_DEFAULT_FREQUENCY_FINE);

impl WaveFrequencyFine {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).sqrt()
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        value.powf(2.0) - 0.5
    }
}


create_wave_value!(WaveFeedback, WAVE_DEFAULT_FEEDBACK);

impl WaveFeedback {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        value
    }
}


create_wave_value!(WaveModulationIndex, WAVE_DEFAULT_MODULATION_INDEX);

impl WaveModulationIndex {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step_smooth(&WAVE_BETA_STEPS[..], value)
    }
    pub fn to_host_value(&self, value: f64) -> f64 {
        get_host_value_for_step(&WAVE_BETA_STEPS[..], value) // TODO: add util for smooth reverse step finding
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VolumeEnvelopeAttackDuration(pub f64);

impl VolumeEnvelopeAttackDuration {
    pub fn new() -> Self {
        Self(WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_DURATION)
    }

    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_DURATION
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VolumeEnvelopeAttackValue(pub f64);

impl VolumeEnvelopeAttackValue {
    pub fn new() -> Self {
        Self(WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_VALUE)
    }

    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_VALUE
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VolumeEnvelopeDecayDuration(pub f64);

impl VolumeEnvelopeDecayDuration {
    pub fn new() -> Self {
        Self(WAVE_DEFAULT_VOLUME_ENVELOPE_DECAY_DURATION)
    }

    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME_ENVELOPE_DECAY_DURATION
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VolumeEnvelopeDecayValue(pub f64);

impl VolumeEnvelopeDecayValue {
    pub fn new() -> Self {
        Self(WAVE_DEFAULT_VOLUME_ENVELOPE_DECAY_VALUE)
    }

    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME_ENVELOPE_DECAY_VALUE
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VolumeEnvelopeReleaseDuration(pub f64);

impl VolumeEnvelopeReleaseDuration {
    pub fn new() -> Self {
        Self(WAVE_DEFAULT_VOLUME_ENVELOPE_RELEASE_DURATION)
    }

    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME_ENVELOPE_RELEASE_DURATION
    }
}


#[derive(Debug, Copy, Clone)]
pub struct WaveVolumeEnvelope {
    pub attack_duration: VolumeEnvelopeAttackDuration,
    pub attack_end_value: VolumeEnvelopeAttackValue,
    pub decay_duration: VolumeEnvelopeDecayDuration,
    pub decay_end_value: VolumeEnvelopeDecayValue,
    pub release_duration: VolumeEnvelopeReleaseDuration,
}

impl Default for WaveVolumeEnvelope {
    fn default() -> Self {
        Self {
            attack_duration: VolumeEnvelopeAttackDuration::new(),
            attack_end_value: VolumeEnvelopeAttackValue::new(),
            decay_duration: VolumeEnvelopeDecayDuration::new(),
            decay_end_value: VolumeEnvelopeDecayValue::new(),
            release_duration: VolumeEnvelopeReleaseDuration::new(),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Wave {
    pub duration: WaveDuration,
    pub volume: WaveVolume,
    pub skip_chain_factor: WaveSkipChainFactor,
    pub frequency_ratio: WaveFrequencyRatio,
    pub frequency_free: WaveFrequencyFree,
    pub frequency_fine: WaveFrequencyFine,
    pub feedback: WaveFeedback,
    pub modulation_index: WaveModulationIndex,
    pub volume_envelope: WaveVolumeEnvelope,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            duration: WaveDuration(0.0),
            skip_chain_factor: WaveSkipChainFactor::default(),
            volume: WaveVolume::default(),
            frequency_ratio: WaveFrequencyRatio::default(),
            frequency_free: WaveFrequencyFree::default(),
            frequency_fine: WaveFrequencyFine::default(),
            feedback: WaveFeedback::default(),
            modulation_index: WaveModulationIndex::default(),
            volume_envelope: WaveVolumeEnvelope::default(),
        }
    }
}