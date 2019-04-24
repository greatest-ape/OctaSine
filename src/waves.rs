use crate::constants::*;
use crate::utils::*;


#[derive(Debug, Copy, Clone)]
pub struct WaveDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct WaveVolume(pub f64);

impl WaveVolume {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME / 2.0
    }
}


#[derive(Debug, Copy, Clone)]
pub struct WaveSkipModulation(pub f64);

impl WaveSkipModulation {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_SKIP_MODULATION
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFrequencyRatio(pub f64);

impl WaveFrequencyRatio {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step(&WAVE_RATIO_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_RATIO_STEPS[..], 1.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFrequencyFree(pub f64);

impl WaveFrequencyFree {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).powf(3.0)
    }
    pub fn get_default_host_value(&self) -> f64 {
        0.5
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFrequencyFine(pub f64);

impl WaveFrequencyFine {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).sqrt()
    }
    pub fn get_default_host_value(&self) -> f64 {
        0.5
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFeedback(pub f64);

impl WaveFeedback {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_FEEDBACK
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveModulationIndex(pub f64);

impl WaveModulationIndex {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step_smooth(&WAVE_BETA_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_BETA_STEPS[..], WAVE_DEFAULT_MODULATION_INDEX)
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
    pub skip_modulation: WaveSkipModulation,
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
            skip_modulation: WaveSkipModulation(WAVE_DEFAULT_SKIP_MODULATION),
            volume: WaveVolume(WAVE_DEFAULT_VOLUME),
            frequency_ratio: WaveFrequencyRatio(WAVE_DEFAULT_FREQUENCY_RATIO),
            frequency_free: WaveFrequencyFree(WAVE_DEFAULT_FREQUENCY_FREE),
            frequency_fine: WaveFrequencyFine(WAVE_DEFAULT_FREQUENCY_FINE),
            feedback: WaveFeedback(WAVE_DEFAULT_FEEDBACK),
            modulation_index: WaveModulationIndex(WAVE_DEFAULT_MODULATION_INDEX),
            volume_envelope: WaveVolumeEnvelope::default(),
        }
    }
}