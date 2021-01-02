use std::f64::consts::PI;

use crate::common::*;

pub const TAU: f64 = PI * 2.0;

/// Lower values can be considered to be zero for optimization purposes
pub const ZERO_VALUE_LIMIT: f64 = 0.001;

pub const PLUGIN_NAME: &str = "OctaSine";
pub const PLUGIN_UNIQUE_ID: i32 = 43789;

pub const NUM_OPERATORS: usize = 4;
pub const NUM_LFOS: usize = 1;

/// Multiply the volume of each voice with this factor
pub const VOICE_VOLUME_FACTOR: f64 = 0.1;

pub const MASTER_FREQUENCY_STEPS: [f64; 12] = [20.0, 110.0, 220.0, 400.0, 435.0, 438.0, 440.0, 442.0, 445.0, 480.0, 880.0, 20_000.0];

pub const OPERATOR_RATIO_STEPS: [f64; 28] = [0.0625, 0.125, 0.2, 0.25, 0.33, 0.5, 0.55, 0.66, 0.6896, 0.8, 0.8333, 1.0, 1.2, 1.25, 1.33, 1.45, 1.5, 1.8, 1.875, 2.0, 2.5, 3.0, 3.5, 4.0, 8.0, 16.0, 32.0, 64.0];
pub const OPERATOR_FREE_STEPS: [f64; 14] = [0.001, 0.0625, 0.125, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 16.0, 64.0, 256.0];
pub const OPERATOR_FINE_STEPS: [f64; 17] = [0.8, 0.85, 0.9, 0.95, 0.97, 0.98, 0.99, 0.995, 1.0, 1.005, 1.01, 1.02, 1.03, 1.05, 1.1, 1.15, 1.2];
pub const OPERATOR_BETA_STEPS: [f64; 16] = [0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0];

pub const ENVELOPE_MAX_DURATION: f64 = 4.0;
pub const ENVELOPE_MIN_DURATION: f64 = 0.004;

/// After this duration, the envelope slope does not get mixed with linear
/// slope at all
pub const ENVELOPE_CURVE_TAKEOVER: f64 = 0.01;
pub const ENVELOPE_CURVE_TAKEOVER_RECIP: f64 = 1.0 / ENVELOPE_CURVE_TAKEOVER;

pub const LFO_TARGET_CONTEXT_STEPS: [LfoTargetParameter; 2] = [ 
    LfoTargetParameter::Master(LfoTargetMasterParameter::Frequency),
    LfoTargetParameter::Master(LfoTargetMasterParameter::Volume),
    // LfoTargetParameter::Operator(0, LfoTargetOperatorParameter::Volume),
    // LfoTargetParameter::Operator(1, LfoTargetOperatorParameter::Volume),
    // LfoTargetParameter::Operator(2, LfoTargetOperatorParameter::Volume),
    // LfoTargetParameter::Operator(3, LfoTargetOperatorParameter::Volume),
    // LfoTargetParameter::Lfo(0, LfoTargetLfoParameter::Magnitude),
];
pub const LFO_SHAPE_STEPS: [LfoShape; 2] = [ 
    LfoShape::LinearUp,
    LfoShape::LinearDown,
];
pub const LFO_MODE_STEPS: [LfoMode; 3] = [ 
    LfoMode::Half,
    LfoMode::Once,
    LfoMode::Forever,
];
pub const LFO_SPEED_STEPS: [f64; 7] = [0.125, 0.5, 0.9, 1.0, 1.1, 2.0, 16.0];
pub const LFO_MAGNITUDE_STEPS: [f64; 3] = [0.0, 1.0, 5.0];

// Default values

pub const DEFAULT_MASTER_VOLUME: f64 = 1.0;
pub const DEFAULT_MASTER_FREQUENCY: f64 = 440.0;

pub const DEFAULT_OPERATOR_VOLUME: f64 = 1.0;
pub const DEFAULT_OPERATOR_SKIP_CHAIN_FACTOR: f64 = 0.0;
pub const DEFAULT_OPERATOR_ADDITIVE_FACTOR: f64 = 0.0;
pub const DEFAULT_OPERATOR_PANNING: f64 = 0.5;
pub const DEFAULT_OPERATOR_FREQUENCY_RATIO: f64 = 1.0;
pub const DEFAULT_OPERATOR_FREQUENCY_FREE: f64 = 1.0;
pub const DEFAULT_OPERATOR_FREQUENCY_FINE: f64 = 1.0;
pub const DEFAULT_OPERATOR_FEEDBACK: f64 = 0.0;
pub const DEFAULT_OPERATOR_MODULATION_INDEX: f64 = 1.0;
pub const DEFAULT_OPERATOR_WAVE_TYPE: WaveType = WaveType::Sine;

pub const DEFAULT_OPERATOR_3_MOD_TARGET: usize = 1;
pub const DEFAULT_OPERATOR_4_MOD_TARGET: usize = 2;

pub const DEFAULT_ENVELOPE_ATTACK_DURATION: f64 = ENVELOPE_MIN_DURATION;
pub const DEFAULT_ENVELOPE_ATTACK_VOLUME: f64 = 1.0;
pub const DEFAULT_ENVELOPE_DECAY_DURATION: f64 = ENVELOPE_MIN_DURATION;
pub const DEFAULT_ENVELOPE_DECAY_VOLUME: f64 = 1.0;
pub const DEFAULT_ENVELOPE_RELEASE_DURATION: f64 = 0.25;