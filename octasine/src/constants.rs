use std::f32::consts::PI;

use crate::common::WaveType;


pub const TAU: f32 = PI * 2.0;

/// Lower values can be considered to be zero for optimization purposes
pub const ZERO_VALUE_LIMIT: f32 = 0.001;

pub const PLUGIN_NAME: &str = "OctaSine";

pub const NUM_OPERATORS: usize = 4;

/// Multiply the volume of each voice with this factor
pub const VOICE_VOLUME_FACTOR: f32 = 0.1;

pub const MASTER_FREQUENCY_STEPS: [f32; 12] = [20.0, 110.0, 220.0, 400.0, 435.0, 438.0, 440.0, 442.0, 445.0, 480.0, 880.0, 20_000.0];

pub const OPERATOR_RATIO_STEPS: [f32; 28] = [0.0625, 0.125, 0.2, 0.25, 0.33, 0.5, 0.55, 0.66, 0.6896, 0.8, 0.8333, 1.0, 1.2, 1.25, 1.33, 1.45, 1.5, 1.8, 1.875, 2.0, 2.5, 3.0, 3.5, 4.0, 8.0, 16.0, 32.0, 64.0];
pub const OPERATOR_FREE_STEPS: [f32; 14] = [0.001, 0.0625, 0.125, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 16.0, 64.0, 256.0];
pub const OPERATOR_FINE_STEPS: [f32; 17] = [0.8, 0.85, 0.9, 0.95, 0.97, 0.98, 0.99, 0.995, 1.0, 1.005, 1.01, 1.02, 1.03, 1.05, 1.1, 1.15, 1.2];
pub const OPERATOR_BETA_STEPS: [f32; 16] = [0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0];

pub const INTERPOLATION_SAMPLES_PER_STEP: u8 = 4;
pub const INTERPOLATION_STEPS: u8 = 8;
pub const INTERPOLATION_STEPS_FLOAT: f32 = INTERPOLATION_STEPS as f32;

pub const ENVELOPE_MAX_DURATION: f32 = 4.0;
pub const ENVELOPE_MIN_DURATION: f32 = 0.004;

/// After this duration, the envelope slope does not get mixed with linear
/// slope at all
pub const ENVELOPE_CURVE_TAKEOVER: f32 = 0.01;
pub const ENVELOPE_CURVE_TAKEOVER_RECIP: f32 = 1.0 / ENVELOPE_CURVE_TAKEOVER;

// Default values

pub const DEFAULT_MASTER_VOLUME: f32 = 1.0;
pub const DEFAULT_MASTER_FREQUENCY: f32 = 440.0;

pub const DEFAULT_OPERATOR_VOLUME: f32 = 1.0;
pub const DEFAULT_OPERATOR_SKIP_CHAIN_FACTOR: f32 = 0.0;
pub const DEFAULT_OPERATOR_ADDITIVE_FACTOR: f32 = 0.0;
pub const DEFAULT_OPERATOR_PANNING: f32 = 0.5;
pub const DEFAULT_OPERATOR_FREQUENCY_RATIO: f32 = 1.0;
pub const DEFAULT_OPERATOR_FREQUENCY_FREE: f32 = 1.0;
pub const DEFAULT_OPERATOR_FREQUENCY_FINE: f32 = 1.0;
pub const DEFAULT_OPERATOR_FEEDBACK: f32 = 0.0;
pub const DEFAULT_OPERATOR_MODULATION_INDEX: f32 = 1.0;
pub const DEFAULT_OPERATOR_WAVE_TYPE: WaveType = WaveType::Sine;

pub const DEFAULT_OPERATOR_3_MOD_TARGET: usize = 1;
pub const DEFAULT_OPERATOR_4_MOD_TARGET: usize = 2;

pub const DEFAULT_ENVELOPE_ATTACK_DURATION: f32 = ENVELOPE_MIN_DURATION;
pub const DEFAULT_ENVELOPE_ATTACK_VOLUME: f32 = 1.0;
pub const DEFAULT_ENVELOPE_DECAY_DURATION: f32 = ENVELOPE_MIN_DURATION;
pub const DEFAULT_ENVELOPE_DECAY_VOLUME: f32 = 1.0;
pub const DEFAULT_ENVELOPE_RELEASE_DURATION: f32 = 0.25;