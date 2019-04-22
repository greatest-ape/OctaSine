use std::f64::consts::PI;


pub const TAU: f64 = 2.0 * PI;

pub const NUM_WAVES: usize = 4;

pub const WAVE_DEFAULT_MIX: f64 = 1.0;
pub const WAVE_DEFAULT_RATIO: f64 = 1.0;
pub const WAVE_DEFAULT_FREQUENCY_FREE: f64 = 1.0;
pub const WAVE_DEFAULT_FEEDBACK: f64 = 0.0;
pub const WAVE_DEFAULT_MODULATION_INDEX: f64 = 1.0;

pub const WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_DURATION: f64 = 1.0;
pub const WAVE_DEFAULT_VOLUME_ENVELOPE_ATTACK_VALUE: f64 = 1.0;
pub const WAVE_DEFAULT_VOLUME_ENVELOPE_RELEASE_DURATION: f64 = 1.0;

pub const VOLUME_ENVELOPE_MAX_DURATION: f64 = 1.0;

pub const WAVE_RATIO_STEPS: [f64; 18] = [0.125, 0.2, 0.25, 0.33, 0.5, 0.66, 0.75, 1.0, 1.25, 1.33, 1.5, 1.66, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];
pub const WAVE_BETA_STEPS: [f64; 16] = [0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0];