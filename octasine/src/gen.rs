use rand::Rng;

pub use crate::approximations::*;
pub use crate::common::*;
pub use crate::constants::*;
pub use crate::voices::*;
pub use crate::processing_parameters::*;
pub use crate::presets::*;


/// One for left channel, one for right
pub struct OutputChannel {
    pub additive: f32,
    pub operator_inputs: [f32; NUM_OPERATORS],
}

impl Default for OutputChannel {
    fn default() -> Self {
        Self {
            additive: 0.0,
            operator_inputs: [0.0; NUM_OPERATORS],
        }
    }
}


/// Generate stereo samples for a voice using SIMD
#[cfg(feature = "simd")]
pub fn generate_voice_samples_simd(
    envelope_curve_table: &EnvelopeCurveTable,
    rng: &mut impl Rng,
    time: TimeCounter,
    time_per_sample: TimePerSample,
    parameters: &mut ProcessingParameters,
    voice: &mut Voice,
) -> (f32, f32) {
    use packed_simd::*;
    use simd_sleef_sin35::*;

    let operators = &mut parameters.operators;

    // Extract data

    let mut envelope_volume: [f32; 4] = [0.0; 4];
    let mut operator_volume: [f32; 4] = [0.0; 4];
    let mut operator_modulation_index = [0.0f32; 4];
    let mut operator_feedback: [f32; 4] = [0.0; 4];
    let mut operator_panning: [f32; 4] = [0.0; 4];
    let mut operator_additive: [f32; 4] = [0.0; 4];
    let mut operator_frequency_ratio: [f32; 4] = [0.0; 4];
    let mut operator_frequency_free: [f32; 4] = [0.0; 4];
    let mut operator_frequency_fine: [f32; 4] = [0.0; 4];
    let mut operator_modulation_targets = [0usize; 4];
    let mut operator_wave_type = [WaveType::Sine; 4];

    let mut operator_last_phase: [f32; 4] = [0.0; 4];

    for (index, operator) in operators.iter_mut().enumerate(){
        envelope_volume[index] = {
            voice.operators[index].volume_envelope.get_volume(
                envelope_curve_table,
                &operator.volume_envelope,
                voice.key_pressed,
                voice.duration
            )
        };

        operator_volume[index] = operator.volume.get_value(time);
        operator_modulation_index[index] = operator.modulation_index.get_value(time);
        operator_feedback[index] = operator.feedback.get_value(time);
        operator_panning[index] = operator.panning.get_value(time);
        operator_wave_type[index] = operator.wave_type.value;

        // Get additive factor; use 1.0 for operator 1
        operator_additive[index] = if index == 0 {
            1.0
        } else {
            operator.additive_factor.get_value(time)
        };

        operator_frequency_ratio[index] = operator.frequency_ratio.value;
        operator_frequency_free[index] = operator.frequency_free.value;
        operator_frequency_fine[index] = operator.frequency_fine.value;

        if let Some(p) = &mut operator.output_operator {
            use ProcessingParameterOperatorModulationTarget::*;

            let opt_value = match p {
                OperatorIndex2(p) => Some(p.get_value(time)),
                OperatorIndex3(p) => Some(p.get_value(time)),
            };

            if let Some(value) = opt_value {
                operator_modulation_targets[index] = value;
            }
        }

        operator_last_phase[index] = voice.operators[index].last_phase.0;
    }

    // Put data into SIMD variables

    let envelope_volume_simd = f32x4::from(envelope_volume);
    let operator_volume_simd = f32x4::from(operator_volume);
    let operator_feedback_simd = f32x4::from(operator_feedback);
    let operator_modulation_index_simd = f32x4::from(operator_modulation_index);
    let operator_panning_simd = f32x4::from(operator_panning);
    let operator_additive_simd = f32x4::from(operator_additive);
    let operator_frequency_ratio_simd = f32x4::from(operator_frequency_ratio);
    let operator_frequency_free_simd = f32x4::from(operator_frequency_free);
    let operator_frequency_fine_simd = f32x4::from(operator_frequency_fine);
    let operator_last_phase_simd = f32x4::from(operator_last_phase);

    // Do calculations

    let zero_value_limit_simd = f32x4::splat(ZERO_VALUE_LIMIT);
    
    let operator_volume_product_simd = operator_volume_simd *
        envelope_volume_simd;

    let operator_volume_off_simd: m32x4 = operator_volume_product_simd.lt(
        zero_value_limit_simd
    );

    // Calculate, save and return new phase * TAU
    let operator_new_phase_simd: f32x4 = {
        let base_frequency = voice.midi_pitch.get_frequency(
            parameters.master_frequency.value
        );

        let operator_frequency_simd: f32x4 = base_frequency *
            operator_frequency_ratio_simd *
            operator_frequency_free_simd *
            operator_frequency_fine_simd;

        let mut operator_new_phase_simd = operator_last_phase_simd +
            operator_frequency_simd * time_per_sample.0;

        // Get fractional part of floats
        operator_new_phase_simd -= f32x4::from_cast(
            i32x4::from_cast(operator_new_phase_simd)
        );

        // Save new phase
        for index in 0..4 {
            voice.operators[index].last_phase.0 =
                operator_new_phase_simd.extract(index);
        }

        operator_new_phase_simd * TAU
    };

    // Calculate feedback if it is on on any operator
    let feedback_simd: f32x4 = {
        let all_feedback_off = operator_feedback_simd.lt(
            zero_value_limit_simd
        ).all();

        if all_feedback_off {
            f32x4::splat(0.0)
        } else {
            operator_feedback_simd * SleefSin35::sin(operator_new_phase_simd)
        }
    };

    fn create_pairs(source: f32x4) -> [f32x2; 4] {
        let array: [f32; 4] = source.into();
        
        [
            f32x2::splat(array[0]),
            f32x2::splat(array[1]),
            f32x2::splat(array[2]),
            f32x2::splat(array[3]),
        ]
    }

    fn create_pairs_from_two(a: f32x4, b: f32x4) -> [f32x2; 4] {
        let array_a: [f32; 4] = a.into();
        let array_b: [f32; 4] = b.into();

        [
            f32x2::new(array_a[0], array_b[0]),
            f32x2::new(array_a[1], array_b[1]),
            f32x2::new(array_a[2], array_b[2]),
            f32x2::new(array_a[3], array_b[3]),
        ]
    }

    // Calculate panning tendency for weird modulation input panning
    let tendency_pairs = {
        let pan_transformed_simd = 2.0 * (operator_panning_simd - 0.5);
        
        let zero_splat_simd = f32x4::splat(0.0);
        
        let right_tendency_simd = pan_transformed_simd.max(zero_splat_simd);
        let left_tendency_simd = (pan_transformed_simd * -1.0)
            .max(zero_splat_simd);
        
        create_pairs_from_two(left_tendency_simd, right_tendency_simd)
    };

    let constant_power_panning_pairs = [
        f32x2::from(operators[0].panning.left_and_right),
        f32x2::from(operators[1].panning.left_and_right),
        f32x2::from(operators[2].panning.left_and_right),
        f32x2::from(operators[3].panning.left_and_right),
    ];

    // Extract data into pairs

    let phase_pairs = create_pairs(operator_new_phase_simd);
    let modulation_index_pairs = create_pairs(operator_modulation_index_simd);
    let feedback_pairs = create_pairs(feedback_simd);
    let operator_volume_product_pairs = create_pairs(operator_volume_product_simd);
    let additive_pairs = create_pairs(operator_additive_simd);

    // Generate samples

    let mut modulation_in_pairs = [f32x2::splat(0.0); 4];
    let mut additive_out_simd = f32x2::splat(0.0);

    for (index, target) in operator_modulation_targets.iter().enumerate().rev(){
        let target = *target;

        if operator_volume_off_simd.extract(index) {
            continue;
        }

        let mut out_simd = if operator_wave_type[index] == WaveType::Sine {
            let mono = modulation_in_pairs[index].sum();

            modulation_in_pairs[index] = tendency_pairs[index] * mono +
                (1.0 - tendency_pairs[index]) * modulation_in_pairs[index];

            let sin_input_simd: f32x2 = modulation_index_pairs[index] *
                (feedback_pairs[index] + modulation_in_pairs[index]) +
                phase_pairs[index];

            SleefSin35::sin(sin_input_simd)
        } else {
            f32x2::splat((rng.gen::<f32>() - 0.5) * 2.0)
        };

        out_simd *= operator_volume_product_pairs[index] *
            constant_power_panning_pairs[index];
        
        let additive_out_increase_simd = additive_pairs[index] * out_simd;

        additive_out_simd += additive_out_increase_simd;
        modulation_in_pairs[target] += out_simd - additive_out_increase_simd;
    }

    let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 *
        parameters.master_volume.get_value(time);
    
    additive_out_simd *= volume_factor;

    (additive_out_simd.extract(0), additive_out_simd.extract(1))
}

/// Generate stereo samples for a voice
/// 
/// Doesn't take self parameter due to conflicting borrowing of Voices
/// in calling function `process`
#[allow(dead_code)]
pub fn generate_voice_samples(
    envelope_curve_table: &EnvelopeCurveTable,
    rng: &mut impl Rng,
    time: TimeCounter,
    time_per_sample: TimePerSample,
    parameters: &mut ProcessingParameters,
    voice: &mut Voice,
) -> (f32, f32) {
    let operators = &mut parameters.operators;

    let base_frequency = voice.midi_pitch.get_frequency(
        parameters.master_frequency.value
    );

    let mut output_channels = [
        OutputChannel::default(),
        OutputChannel::default()
    ];

    for (operator_index, operator) in (operators.iter_mut().enumerate()).rev() {
        // Fetch all operator values here to make sure all interpolatable
        // ones are advanced even if calculations are skipped below.

        let operator_volume = operator.volume.get_value(time);
        let operator_feedback = operator.feedback.get_value(time);
        let operator_modulation_index = operator.modulation_index.get_value(time);
        let operator_panning = operator.panning.get_value(time);

        // Get additive factor; use 1.0 for operator 1
        let operator_additive = if operator_index == 0 {
            1.0
        } else {
            operator.additive_factor.get_value(time)
        };

        // Get modulation target; use operator 1 for operator 1 and 2.
        // (Since additive factor is 1.0 for operator 1, its target is
        // irrelevant.)
        let operator_mod_output = if let Some(ref p) = operator.output_operator {
            match p {
                ProcessingParameterOperatorModulationTarget::OperatorIndex2(p) => p.value,
                ProcessingParameterOperatorModulationTarget::OperatorIndex3(p) => p.value,
            }
        } else {
            0
        };

        let operator_frequency = base_frequency *
            operator.frequency_ratio.value *
            operator.frequency_free.value *
            operator.frequency_fine.value;

        // Always calculate envelope to make sure it advances
        let envelope_volume = {
            voice.operators[operator_index].volume_envelope.get_volume(
                envelope_curve_table,
                &operator.volume_envelope,
                voice.key_pressed,
                voice.duration
            )
        };

        // If volume is off, skip sound generation and panning
        if operator_volume < ZERO_VALUE_LIMIT ||
            envelope_volume < ZERO_VALUE_LIMIT {
            continue;
        }

        let mut operator_inputs = [
            output_channels[0].operator_inputs[operator_index],
            output_channels[1].operator_inputs[operator_index],
        ];

        // Mix modulator into current operator depending on panning of
        // current operator. If panned to the middle, just pass through
        // the stereo signals: if panned to any side, mix out the
        // original stereo signals and mix in mono.
        if operator_panning != 0.5 {
            let pan_transformed = 2.0 * (operator_panning - 0.5);

            let right_tendency = pan_transformed.max(0.0);
            let left_tendency = (-pan_transformed).max(0.0);

            let mono = operator_inputs[0] + operator_inputs[1];

            operator_inputs[0] = (1.0 - left_tendency) * operator_inputs[0] +
                left_tendency * mono;
            operator_inputs[1] = (1.0 - right_tendency) * operator_inputs[1] +
                right_tendency * mono;
        }

        // Calculate, save and return new phase
        let new_phase_times_tau = {
            // Calculate phase increment, add to last phase, get remainder
            // after division with 1.0 with .fract(), which seems to fix
            // an audio issue
            let new_phase = operator_frequency.mul_add(
                time_per_sample.0,
                voice.operators[operator_index].last_phase.0,
            ).fract();

            voice.operators[operator_index].last_phase.0 = new_phase;

            new_phase * TAU
        };

        let mut new_signals = [0.0, 0.0];

        // Generate FM sine / noise signals for each channel
        match operator.wave_type.value {
            WaveType::Sine => {
                // Do feedback calculation only if feedback is on
                let new_feedback = if operator_feedback > ZERO_VALUE_LIMIT {
                    operator_feedback * new_phase_times_tau.sin()
                } else {
                    0.0
                };

                let inputs_identical = operator_inputs[0] == operator_inputs[1];

                for channel in 0..2 {
                    // Skip generating right channel signal if inputs
                    // are identical - just use the left channel signal
                    if channel == 1 && inputs_identical {
                        new_signals[1] = new_signals[0];
                    } else {
                        let modulation = operator_modulation_index *
                            (operator_inputs[channel] + new_feedback);

                        let signal = (new_phase_times_tau + modulation).sin();

                        new_signals[channel] = envelope_volume * signal;
                    }
                }
            },
            WaveType::WhiteNoise => {
                let signal = envelope_volume *
                    (rng.gen::<f32>() - 0.5) * 2.0;

                new_signals[0] = signal;
                new_signals[1] = signal;
            }
        }

        // Pan signals and write to output_channels
        for channel in 0..2 {
            let pan_volume = operator.panning.left_and_right[channel];

            let out = pan_volume * operator_volume * new_signals[channel];

            let additive_out = operator_additive * out;
            let mod_out = out - additive_out;

            output_channels[channel].additive += additive_out;
            output_channels[channel]
                .operator_inputs[operator_mod_output] += mod_out;
        }
    }

    let signal_left = output_channels[0].additive;
    let signal_right = output_channels[1].additive;

    let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 *
        parameters.master_volume.get_value(time);

    (signal_left * volume_factor, signal_right * volume_factor)
}