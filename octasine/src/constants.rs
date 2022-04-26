pub const PLUGIN_NAME: &str = "OctaSine";
pub const PLUGIN_UNIQUE_ID: i32 = 1_438_048_624;

pub const NUM_OPERATORS: usize = 4;
pub const NUM_LFOS: usize = 4;

/// Multiply the volume of each voice with this factor
pub const VOICE_VOLUME_FACTOR: f64 = 0.1;

pub const OPERATOR_MOD_INDEX_STEPS: [f64; 16] = [
    0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0,
];
