/// Fallback audio generation not requiring simd
pub mod fallback {
    use rand::Rng;
    use vst::buffer::AudioBuffer;

    use vst2_helpers::approximations::*;
    use vst2_helpers::processing_parameters::ProcessingParameter;

    use crate::OctaSine;
    use crate::common::*;
    use crate::constants::*;
    use crate::voices::*;
    use crate::processing_parameters::*;

    /// One for left channel, one for right
    pub struct OutputChannel {
        pub additive: f64,
        pub operator_inputs: [f64; NUM_OPERATORS],
    }

    impl Default for OutputChannel {
        fn default() -> Self {
            Self {
                additive: 0.0,
                operator_inputs: [0.0; NUM_OPERATORS],
            }
        }
    }

    #[inline]
    fn hard_limit(value: f64) -> f64 {
        value.min(1.0).max(-1.0)
    }

    #[inline]
    pub fn process_f32(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>){
        let mut outputs = audio_buffer.split().1;
        let lefts = outputs.get_mut(0).iter_mut();
        let rights = outputs.get_mut(1).iter_mut();

        for (buffer_sample_left, buffer_sample_right) in lefts.zip(rights){
            let (left, right) = gen_samples_for_voices(octasine);

            *buffer_sample_left = left as f32;
            *buffer_sample_right = right as f32;
        }
    }

    #[inline]
    pub fn gen_samples_for_voices(octasine: &mut OctaSine) -> (f64, f64) {
        let changed_preset_parameters = octasine.sync_only.presets
            .get_changed_parameters();

        if let Some(indeces) = changed_preset_parameters {
            for (index, opt_new_value) in indeces.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    if let Some(p) = octasine.processing.parameters.get(index){
                        p.set_from_preset_value(*new_value);
                    }
                }
            }
        }

        let mut voice_sum_left: f64 = 0.0;
        let mut voice_sum_right: f64 = 0.0;

        let time_per_sample = octasine.processing.time_per_sample;

        for voice in octasine.processing.voices.iter_mut(){
            if voice.active {
                let (out_left, out_right) = generate_voice_samples(
                    &octasine.processing.log10_table,
                    &mut octasine.processing.rng,
                    octasine.processing.global_time,
                    time_per_sample,
                    &mut octasine.processing.parameters,
                    voice,
                );

                voice_sum_left += hard_limit(out_left);
                voice_sum_right += hard_limit(out_right);

                voice.duration.0 += time_per_sample.0;

                voice.deactivate_if_envelopes_ended();
            }
        }

        octasine.processing.global_time.0 += time_per_sample.0;

        (voice_sum_left, voice_sum_right)
    }

    /// Generate stereo samples for a voice
    #[inline]
    pub fn generate_voice_samples(
        log10_table: &Log10Table,
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut ProcessingParameters,
        voice: &mut Voice,
    ) -> (f64, f64) {
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
                    log10_table,
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
                        (rng.gen::<f64>() - 0.5) * 2.0;

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
}


/// Audio generation using explicit simd
/// 
/// At least SSE2 is required. Simdeez scalar fallback will fail due to the
/// method used for stereo modulation input calculation.
/// 
/// TODO:
///   - Interpolation for processing parameters every sample? Build long arrays here too?
///   - Pan tendency broken? Probably not
///   - White noise
///   - Maybe skip audio gen on very low operator volume * envelope volume,
///     but it would introduce branching
#[cfg(feature = "simd2")]
pub mod simdeez {
    use simdeez::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;
    use simdeez::avx2::*;
    use simdeez::avx::*;

    use arrayvec::ArrayVec;

    // use rand::{Rng, rngs::SmallRng};
    use vst::buffer::AudioBuffer;

    use vst2_helpers::processing_parameters::ProcessingParameter;

    use crate::OctaSine;
    use crate::common::*;
    use crate::constants::*;
    use crate::processing_parameters::*;

    /// Each SAMPLE_PASS_SIZE samples, load parameter changes
    const SAMPLE_PASS_SIZE: usize = 16;

    simd_runtime_generate!(
        pub fn process_f32(
            octasine: &mut OctaSine,
            audio_buffer: &mut AudioBuffer<f32>
        ){
            let num_samples = audio_buffer.samples();

            let mut audio_buffer_outputs = audio_buffer.split().1;
            let audio_buffer_lefts = audio_buffer_outputs.get_mut(0);
            let audio_buffer_rights = audio_buffer_outputs.get_mut(1);

            let num_passes = num_samples / SAMPLE_PASS_SIZE;

            for pass_index in 0..num_passes {
                // --- Update processing parameters from preset parameters

                let changed_preset_parameters = octasine.sync_only.presets
                    .get_changed_parameters();

                if let Some(indeces) = changed_preset_parameters {
                    for (index, opt_new_value) in indeces.iter().enumerate(){
                        if let Some(new_value) = opt_new_value {
                            if let Some(p) = octasine.processing.parameters.get(index){
                                p.set_from_preset_value(*new_value);
                            }
                        }
                    }
                }

                // --- Set some variables

                let operators = &mut octasine.processing.parameters.operators;

                let time_per_sample = octasine.processing.time_per_sample;
                let time = octasine.processing.global_time;
                let master_volume_factor = VOICE_VOLUME_FACTOR * octasine.processing.parameters.master_volume.get_value(time);

                // --- Get operator-only data which will be valid for whole pass and all voices.

                // Interpolated
                let mut operator_volume: [f64; 4] = [0.0; 4];
                let mut operator_modulation_index = [0.0f64; 4];
                let mut operator_feedback: [f64; 4] = [0.0; 4];
                let mut operator_panning: [f64; 4] = [0.0; 4];
                let mut operator_additive: [f64; 4] = [0.0; 4];
                
                // Not interpolated
                let mut operator_wave_type = [WaveType::Sine; 4];
                let mut operator_frequency_modifiers: [f64; 4] = [0.0; 4]; 
                let mut operator_modulation_targets = [0usize; 4];

                for (index, operator) in operators.iter_mut().enumerate(){
                    operator_volume[index] = operator.volume.get_value(time);
                    operator_modulation_index[index] = operator.modulation_index.get_value(time);
                    operator_feedback[index] = operator.feedback.get_value(time);
                    operator_panning[index] = operator.panning.get_value(time);

                    // Get additive factor; use 1.0 for operator 1
                    operator_additive[index] = if index == 0 {
                        1.0
                    } else {
                        operator.additive_factor.get_value(time)
                    };

                    operator_wave_type[index] = operator.wave_type.value;

                    operator_frequency_modifiers[index] = operator.frequency_ratio.value *
                        operator.frequency_free.value * operator.frequency_fine.value;

                    if let Some(p) = &mut operator.output_operator {
                        use ProcessingParameterOperatorModulationTarget::*;

                        let opt_value = match p {
                            OperatorIndex2(p) => Some(p.get_value(())),
                            OperatorIndex3(p) => Some(p.get_value(())),
                        };

                        if let Some(value) = opt_value {
                            operator_modulation_targets[index] = value;
                        }
                    }
                }

                octasine.processing.global_time.0 += time_per_sample.0 * (SAMPLE_PASS_SIZE as f64);

                // --- Collect voice data (envelope volume, phases) necessary for sound generation

                // FIXME: optimize section, possibly with simd. Maybe envelopes can be calculated less often

                // Maybe operator indexes should be inversed (3 - operator_index)
                // because that is how they will be accessed later.
                let mut voice_envelope_volumes: ArrayVec<[[[f64; SAMPLE_PASS_SIZE * 2]; 4]; 128]> = ArrayVec::new();
                let mut voice_phases: ArrayVec<[[[f64; SAMPLE_PASS_SIZE * 2]; 4]; 128]> = ArrayVec::new();
                let mut key_velocities: ArrayVec<[f64; 128]> = ArrayVec::new();
                
                let mut num_active_voices = 0;

                for voice in octasine.processing.voices.iter_mut(){
                    if voice.active {
                        let mut operator_envelope_volumes = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];
                        let mut operator_phases = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

                        let voice_base_frequency = voice.midi_pitch.get_frequency(
                            octasine.processing.parameters.master_frequency.value
                        );

                        // Envelope
                        for i in 0..SAMPLE_PASS_SIZE {
                            for (operator_index, operator) in operators.iter_mut().enumerate(){
                                let v = voice.operators[operator_index].volume_envelope.get_volume(
                                    &octasine.processing.log10_table,
                                    &operator.volume_envelope,
                                    voice.key_pressed,
                                    voice.duration
                                );

                                let j = i * 2;

                                operator_envelope_volumes[operator_index][j] = v;
                                operator_envelope_volumes[operator_index][j + 1] = v;
                            }

                            voice.duration.0 += time_per_sample.0;
                        }

                        // Phase
                        for operator_index in 0..4 {
                            let last_phase = voice.operators[operator_index].last_phase.0;
                            let frequency = voice_base_frequency * operator_frequency_modifiers[operator_index];
                            let phase_addition = frequency * time_per_sample.0;

                            let mut new_phase = 0.0;

                            for i in 0..SAMPLE_PASS_SIZE {
                                // Do multiplication instead of successive addition for less precision loss (hopefully)
                                new_phase = last_phase + phase_addition * ((i + 1) as f64);

                                let j = i * 2;

                                operator_phases[operator_index][j] = new_phase;
                                operator_phases[operator_index][j + 1] = new_phase;
                            }

                            // Save phase
                            voice.operators[operator_index].last_phase.0 = new_phase;
                        }

                        voice_envelope_volumes.push(operator_envelope_volumes);
                        voice_phases.push(operator_phases);
                        key_velocities.push(voice.key_velocity.0);

                        voice.deactivate_if_envelopes_ended();

                        num_active_voices += 1;
                    }
                }

                // --- Generate samples for all operators and voices

                // Sample pass size * 2 because of two channels. Even index = left channel
                let mut summed_additive_outputs = [0.0f64; SAMPLE_PASS_SIZE * 2];

                // Voice index here is not the same as in processing storage
                for voice_index in 0..num_active_voices {
                    // Voice modulation input storage, indexed by operator
                    // Sample pass size * 2 because of two channels
                    let mut voice_modulation_inputs = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

                    let key_velocity_splat = S::set1_pd(key_velocities[voice_index]);

                    // Go through operators downwards, starting with operator 4
                    for operator_index in 0..4 { // FIXME: better iterator with 3, 2, 1, 0 possible?
                        let operator_index = 3 - operator_index;

                        if operator_volume[operator_index] < ZERO_VALUE_LIMIT {
                            continue;
                        }

                        // --- Setup operator SIMD vars

                        let operator_volume_splat = S::set1_pd(operator_volume[operator_index]);
                        let operator_feedback_splat = S::set1_pd(operator_feedback[operator_index]);
                        let operator_additive_splat = S::set1_pd(operator_additive[operator_index]);
                        let operator_modulation_index_splat = S::set1_pd(operator_modulation_index[operator_index]);

                        let (pan_tendency, one_minus_pan_tendency) = {
                            // Get panning as value between -1 and 1
                            let pan_transformed = 2.0 * (operator_panning[operator_index] - 0.5);

                            let r = pan_transformed.max(0.0);
                            let l = (pan_transformed * -1.0).max(0.0);

                            // Width 8 in case of eventual avx512 support in simdeez
                            let data = [l, r, l, r, l, r, l, r];
                            
                            let tendency = S::loadu_pd(&data[0]);
                            let one_minus_tendency = S::set1_pd(1.0) - tendency;

                            (tendency, one_minus_tendency)
                        };

                        let constant_power_panning = {
                            let mut data = [0.0f64; 8];

                            let left_and_right = operators[operator_index].panning.left_and_right;
                            
                            for (i, v) in data.iter_mut().enumerate() {
                                *v = left_and_right[i % 2];
                            }

                            S::loadu_pd(&data[0])
                        };

                        let modulation_target = operator_modulation_targets[operator_index];

                        // --- Create samples for both channels

                        let tau_splat = S::set1_pd(TAU);

                        for i in (0..SAMPLE_PASS_SIZE * 2).step_by(S::VF64_WIDTH) {
                            let envelope_volume = S::loadu_pd(&voice_envelope_volumes[voice_index][operator_index][i]);
                            let volume_product = operator_volume_splat * envelope_volume;

                            // FIXME: skip generation when envelope volume or operator volume is zero for all loaded samples?
                            // Probably not really necessary since voice won't be set as active for long on zero envelope volume.
                            //let volume_off = S::cmplt_pd(volume_product, zero_value_limit_splat);

                            let modulation_in_for_channel = S::loadu_pd(&voice_modulation_inputs[operator_index][i]);
                            let phase = S::loadu_pd(&voice_phases[voice_index][operator_index][i]) * tau_splat;

                            // Weird modulation input panning
                            // Note: breaks without VF64_WIDTH >= 2 (SSE2 or newer)
                            let modulation_in_channel_sum = {
                                // Replacing with SIMD: suitable instructions in avx:
                                //   _mm256_permute_pd with imm8 = [1, 0, 1, 0] followed by addition
                                //     Indices:
                                //       0 -> 1
                                //       1 -> 0
                                //       2 -> 3
                                //       3 -> 2
                                //   _mm256_hadd_pd (takes two variables which would need to be identical): pretty slow
                                // So the idea is to take modulation_in_for_channel and run any of the above on it.

                                let mut permuted = [0.0f64; 8]; // Width 8 in case of eventual avx512 support in simdeez

                                // Should be equivalent to simd instruction permute_pd with imm8 = [1, 0, 1, 0]
                                for (j, input) in (&voice_modulation_inputs[operator_index][i..i + S::VF64_WIDTH]).iter().enumerate(){
                                    let add = (j + 1) % 2;
                                    let subtract = j % 2;

                                    permuted[j + add - subtract] = *input;
                                }

                                S::loadu_pd(&permuted[0]) + modulation_in_for_channel
                            };

                            let modulation_in = pan_tendency * modulation_in_channel_sum +
                                one_minus_pan_tendency * modulation_in_for_channel;

                            let feedback = operator_feedback_splat * S::fast_sin_pd(phase);

                            let sin_input = operator_modulation_index_splat * (feedback + modulation_in) + phase;

                            let sample = S::fast_sin_pd(sin_input);

                            let sample_adjusted = sample * volume_product * constant_power_panning;
                            let additive_out = sample_adjusted * operator_additive_splat;
                            let modulation_out = sample_adjusted - additive_out;

                            // Add modulation output to target operator's modulation inputs
                            let modulation_sum = S::loadu_pd(&voice_modulation_inputs[modulation_target][i]) + modulation_out;
                            S::storeu_pd(&mut voice_modulation_inputs[modulation_target][i], modulation_sum);

                            // Add additive output to summed_additive_outputs
                            let summed_plus_new = S::loadu_pd(&summed_additive_outputs[i]) + (additive_out * key_velocity_splat);
                            S::storeu_pd(&mut summed_additive_outputs[i], summed_plus_new);

                            // TODO: noise generation
                        } // End of sample pass size *  2 iteration
                    } // End of operator iteration
                } // End of voice iteration

                // --- Summed additive outputs: apply master volume and hard limit.

                let master_volume_factor_splat = S::set1_pd(master_volume_factor);
                let max_volume_splat = S::set1_pd(5.0);

                for i in (0..SAMPLE_PASS_SIZE * 2).step_by(S::VF64_WIDTH) {
                    let additive_outputs = S::loadu_pd(&summed_additive_outputs[i]);
                    let additive_outputs = additive_outputs * master_volume_factor_splat;
                    let limited_outputs = S::min_pd(additive_outputs, max_volume_splat);

                    S::storeu_pd(&mut summed_additive_outputs[i], limited_outputs);
                }

                // --- Write additive outputs to audio buffer

                let sample_offset = pass_index * SAMPLE_PASS_SIZE;

                for i in 0..SAMPLE_PASS_SIZE {
                    let j = i * 2;
                    audio_buffer_lefts[i + sample_offset] = summed_additive_outputs[j] as f32;
                    audio_buffer_rights[i + sample_offset] = summed_additive_outputs[j + 1] as f32;
                }
            } // End of pass iteration
        }
    );
}


/// Generate stereo samples for a voice using packed_simd. Only here for reference.
#[cfg(feature = "simd")]
pub mod packed_simd {
    pub fn generate_voice_samples_simd(
        log10_table: &Log10Table,
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut ProcessingParameters,
        voice: &mut Voice,
    ) -> (f64, f64) {
        use packed_simd::*;
        use simd_sleef_sin35::*;

        let operators = &mut parameters.operators;

        // Extract data

        let mut envelope_volume: [f64; 4] = [0.0; 4];
        let mut operator_volume: [f64; 4] = [0.0; 4];
        let mut operator_modulation_index = [0.0f64; 4];
        let mut operator_feedback: [f64; 4] = [0.0; 4];
        let mut operator_panning: [f64; 4] = [0.0; 4];
        let mut operator_additive: [f64; 4] = [0.0; 4];
        let mut operator_frequency_ratio: [f64; 4] = [0.0; 4];
        let mut operator_frequency_free: [f64; 4] = [0.0; 4];
        let mut operator_frequency_fine: [f64; 4] = [0.0; 4];
        let mut operator_modulation_targets = [0usize; 4];
        let mut operator_wave_type = [WaveType::Sine; 4];

        let mut operator_last_phase: [f64; 4] = [0.0; 4];

        for (index, operator) in operators.iter_mut().enumerate(){
            envelope_volume[index] = {
                voice.operators[index].volume_envelope.get_volume(
                    log10_table,
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
                    OperatorIndex2(p) => Some(p.get_value(())),
                    OperatorIndex3(p) => Some(p.get_value(())),
                };

                if let Some(value) = opt_value {
                    operator_modulation_targets[index] = value;
                }
            }

            operator_last_phase[index] = voice.operators[index].last_phase.0;
        }

        // Put data into SIMD variables

        let envelope_volume_simd = f64x4::from(envelope_volume);
        let operator_volume_simd = f64x4::from(operator_volume);
        let operator_feedback_simd = f64x4::from(operator_feedback);
        let operator_modulation_index_simd = f64x4::from(operator_modulation_index);
        let operator_panning_simd = f64x4::from(operator_panning);
        let operator_additive_simd = f64x4::from(operator_additive);
        let operator_frequency_ratio_simd = f64x4::from(operator_frequency_ratio);
        let operator_frequency_free_simd = f64x4::from(operator_frequency_free);
        let operator_frequency_fine_simd = f64x4::from(operator_frequency_fine);
        let operator_last_phase_simd = f64x4::from(operator_last_phase);

        // Do calculations

        let zero_value_limit_simd = f64x4::splat(ZERO_VALUE_LIMIT);
        
        let operator_volume_product_simd = operator_volume_simd *
            envelope_volume_simd;

        let operator_volume_off_simd: m64x4 = operator_volume_product_simd.lt(
            zero_value_limit_simd
        );

        // Calculate, save and return new phase * TAU
        let operator_new_phase_simd: f64x4 = {
            let base_frequency = voice.midi_pitch.get_frequency(
                parameters.master_frequency.value
            );

            let operator_frequency_simd: f64x4 = base_frequency *
                operator_frequency_ratio_simd *
                operator_frequency_free_simd *
                operator_frequency_fine_simd;

            let operator_new_phase_simd = operator_frequency_simd.mul_adde(
                f64x4::splat(time_per_sample.0),
                operator_last_phase_simd
            );

            // Save new phase
            for (i, voice_operator) in voice.operators.iter_mut().enumerate(){
                voice_operator.last_phase.0 = operator_new_phase_simd.extract(i);
            }

            operator_new_phase_simd * TAU
        };

        // Calculate feedback if it is on on any operator
        let feedback_simd: f64x4 = {
            let all_feedback_off = operator_feedback_simd.lt(
                zero_value_limit_simd
            ).all();

            if all_feedback_off {
                f64x4::splat(0.0)
            } else {
                operator_feedback_simd * SleefSin35::sin(operator_new_phase_simd)
            }
        };

        fn create_pairs(source: f64x4) -> [f64x2; 4] {
            let array: [f64; 4] = source.into();
            
            [
                f64x2::splat(array[0]),
                f64x2::splat(array[1]),
                f64x2::splat(array[2]),
                f64x2::splat(array[3]),
            ]
        }

        fn create_pairs_from_two(a: f64x4, b: f64x4) -> [f64x2; 4] {
            let array_a: [f64; 4] = a.into();
            let array_b: [f64; 4] = b.into();

            [
                f64x2::new(array_a[0], array_b[0]),
                f64x2::new(array_a[1], array_b[1]),
                f64x2::new(array_a[2], array_b[2]),
                f64x2::new(array_a[3], array_b[3]),
            ]
        }

        // Calculate panning tendency for weird modulation input panning
        let tendency_pairs = {
            let pan_transformed_simd = 2.0 * (operator_panning_simd - 0.5);
            
            let zero_splat_simd = f64x4::splat(0.0);
            
            let right_tendency_simd = pan_transformed_simd.max(zero_splat_simd);
            let left_tendency_simd = (pan_transformed_simd * -1.0)
                .max(zero_splat_simd);
            
            create_pairs_from_two(left_tendency_simd, right_tendency_simd)
        };

        let constant_power_panning_pairs = [
            f64x2::from(operators[0].panning.left_and_right),
            f64x2::from(operators[1].panning.left_and_right),
            f64x2::from(operators[2].panning.left_and_right),
            f64x2::from(operators[3].panning.left_and_right),
        ];

        // Extract data into pairs

        let phase_pairs = create_pairs(operator_new_phase_simd);
        let modulation_index_pairs = create_pairs(operator_modulation_index_simd);
        let feedback_pairs = create_pairs(feedback_simd);
        let operator_volume_product_pairs = create_pairs(operator_volume_product_simd);
        let additive_pairs = create_pairs(operator_additive_simd);

        // Generate samples

        let mut modulation_in_pairs = [f64x2::splat(0.0); 4];
        let mut additive_out_simd = f64x2::splat(0.0);

        for (index, target) in operator_modulation_targets.iter().enumerate().rev(){
            let target = *target;

            if operator_volume_off_simd.extract(index) {
                continue;
            }

            let mut out_simd = if operator_wave_type[index] == WaveType::Sine {
                let mono = modulation_in_pairs[index].sum();

                modulation_in_pairs[index] = tendency_pairs[index].mul_adde(
                    f64x2::splat(mono),
                    (1.0 - tendency_pairs[index]) * modulation_in_pairs[index]
                );

                let sin_input_simd: f64x2 = modulation_index_pairs[index].mul_adde(
                    feedback_pairs[index] + modulation_in_pairs[index],
                    phase_pairs[index]
                );

                SleefSin35::sin(sin_input_simd)
            } else {
                f64x2::splat((rng.gen::<f64>() - 0.5) * 2.0)
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
}


/// Attempt to split packed simd gen implementation into separate functions.
/// Only here for reference.
#[cfg(feature = "simd")]
mod packed_simd_split {
    use rand::Rng;

    use vst2_helpers::approximations::*;
    use vst2_helpers::processing_parameters::ProcessingParameter;

    pub use crate::common::*;
    pub use crate::constants::*;
    pub use crate::voices::*;
    pub use crate::processing_parameters::*;
    pub use crate::preset_parameters::*;

    use packed_simd::*;
    use simd_sleef_sin35::*;

    // Get envelope volume and last phase for voice
    fn get_voice_data(
        voice: &mut Voice,
        operators: &mut ProcessingParameterOperators,
    ) -> ([f64; 4], [f64; 4]) {
        let mut envelope_volume: [f64; 4] = [0.0; 4];
        let mut operator_last_phase: [f64; 4] = [0.0; 4];

        for (index, operator) in operators.iter_mut().enumerate(){
            envelope_volume[index] = {
                voice.operators[index].volume_envelope.get_volume(
                    log10_table,
                    &operator.volume_envelope,
                    voice.key_pressed,
                    voice.duration
                )
            };

            operator_last_phase[index] = voice.operators[index].last_phase.0;
        }

        let envelope_volume_simd = f64x4::from(envelope_volume);
        let operator_last_phase_simd = f64x4::from(operator_last_phase);

        (envelope_volume_simd, operator_last_phase_simd)
    }

    fn calculate_new_phase(
        master_frequency: ProcessingParameterMasterFrequency,
        operator_frequency_modifiers_simd: f64x4,
        time_per_sample: TimePerSample,
        operator_last_phase_simd: f64x4,
    ) -> f64x4 {
        let base_frequency = voice.midi_pitch.get_frequency(
            master_frequency.value
        );

        let operator_frequency_simd: f64x4 = base_frequency *
            operator_frequency_modifiers_simd;

        let operator_new_phase_simd = operator_frequency_simd.mul_adde(
            f64x4::splat(time_per_sample.0),
            operator_last_phase_simd
        );

        operator_new_phase_simd * TAU
    }

    // FIXME: previously didn't get multiplied with TAU before saving
    fn save_phase_to_voice_operators(
        voice: &mut Voice,
        operator_new_phase_simd: f64x4
    ){
        // Save new phase
        for (i, voice_operator) in voice.operators.iter_mut().enumerate(){
            voice_operator.last_phase.0 = operator_new_phase_simd.extract(i);
        }
    }

    // Calculate feedback if it is on on any operator
    fn calculate_feedback(
        operator_feedback_simd: f64x4,
        operator_new_phase_simd: f64x4,
    ){
        let all_feedback_off = operator_feedback_simd.lt(
            f64x4::splat(ZERO_VALUE_LIMIT)
        ).all();

        if all_feedback_off {
            f64x4::splat(0.0)
        } else {
            operator_feedback_simd * SleefSin35::sin(operator_new_phase_simd)
        }
    }

    fn create_pairs(source: f64x4) -> [f64x2; 4] {
        let array: [f64; 4] = source.into();
        
        [
            f64x2::splat(array[0]),
            f64x2::splat(array[1]),
            f64x2::splat(array[2]),
            f64x2::splat(array[3]),
        ]
    }

    fn create_pairs_from_two(a: f64x4, b: f64x4) -> [f64x2; 4] {
        let array_a: [f64; 4] = a.into();
        let array_b: [f64; 4] = b.into();

        [
            f64x2::new(array_a[0], array_b[0]),
            f64x2::new(array_a[1], array_b[1]),
            f64x2::new(array_a[2], array_b[2]),
            f64x2::new(array_a[3], array_b[3]),
        ]
    }

    // Calculate panning tendency for weird modulation input panning
    fn tendency_pairs(
        operator_panning_simd: f64x4
    ){
        let pan_transformed_simd = 2.0 * (operator_panning_simd - 0.5);
        
        let zero_splat_simd = f64x4::splat(0.0);
        
        let right_tendency_simd = pan_transformed_simd.max(zero_splat_simd);
        let left_tendency_simd = (pan_transformed_simd * -1.0)
            .max(zero_splat_simd);
        
        create_pairs_from_two(left_tendency_simd, right_tendency_simd)
    }

    /// Generate stereo samples for a voice
    pub fn generate_voice_samples(
        log10_table: &Log10Table,
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut ProcessingParameters,
        voice: &mut Voice,
    ) -> (f64, f64) {
        use packed_simd::*;
        use simd_sleef_sin35::*;

        let operators = &mut parameters.operators;

        // --- Get operator-only data

        let mut operator_volume: [f64; 4] = [0.0; 4];
        let mut operator_modulation_index = [0.0f64; 4];
        let mut operator_feedback: [f64; 4] = [0.0; 4];
        let mut operator_panning: [f64; 4] = [0.0; 4];
        let mut operator_additive: [f64; 4] = [0.0; 4];
        let mut operator_frequency_ratio: [f64; 4] = [0.0; 4];
        let mut operator_frequency_free: [f64; 4] = [0.0; 4];
        let mut operator_frequency_fine: [f64; 4] = [0.0; 4];
        let mut operator_modulation_targets = [0usize; 4];
        let mut operator_wave_type = [WaveType::Sine; 4];

        for (index, operator) in operators.iter_mut().enumerate(){
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
                    OperatorIndex2(p) => Some(p.get_value(())),
                    OperatorIndex3(p) => Some(p.get_value(())),
                };

                if let Some(value) = opt_value {
                    operator_modulation_targets[index] = value;
                }
            }
        }

        let operator_volume_simd = f64x4::from(operator_volume);
        let operator_feedback_simd = f64x4::from(operator_feedback);
        let operator_modulation_index_simd = f64x4::from(operator_modulation_index);
        let operator_panning_simd = f64x4::from(operator_panning);
        let operator_additive_simd = f64x4::from(operator_additive);
        let operator_frequency_ratio_simd = f64x4::from(operator_frequency_ratio);
        let operator_frequency_free_simd = f64x4::from(operator_frequency_free);
        let operator_frequency_fine_simd = f64x4::from(operator_frequency_fine);

        let operator_frequency_modifiers_simd: f64x4 = operator_frequency_ratio_simd *
            operator_frequency_free_simd *
            operator_frequency_fine_simd;

        // --- Get per-voice data
            
        let (envelope_volume_simd, operator_last_phase_simd) = get_voice_data(
            &mut voice,
            &mut parameters.operators
        );

        // --- Voice and operator mixed calculations
        
        let operator_volume_product_simd = operator_volume_simd *
            envelope_volume_simd;

        let operator_volume_off_simd: m64x4 = operator_volume_product_simd.lt(
            f64x4::splat(ZERO_VALUE_LIMIT)
        );

        let operator_new_phase_simd = calculate_new_phase(
            parameters.master_frequency,
            operator_frequency_modifiers_simd,
            time_per_sample,
            operator_last_phase_simd,
        );

        save_phase_to_voice_operators(
            &mut voice,
            operator_new_phase_simd
        );

        let feedback_simd = calculate_feedback(
            operator_feedback_simd,
            operator_new_phase_simd
        );

        let constant_power_panning_pairs = [
            f64x2::from(operators[0].panning.left_and_right),
            f64x2::from(operators[1].panning.left_and_right),
            f64x2::from(operators[2].panning.left_and_right),
            f64x2::from(operators[3].panning.left_and_right),
        ];

        // Extract data into pairs

        let phase_pairs = create_pairs(operator_new_phase_simd);
        let modulation_index_pairs = create_pairs(operator_modulation_index_simd);
        let feedback_pairs = create_pairs(feedback_simd);
        let operator_volume_product_pairs = create_pairs(operator_volume_product_simd);
        let additive_pairs = create_pairs(operator_additive_simd);

        // Generate samples

        let mut modulation_in_pairs = [f64x2::splat(0.0); 4];
        let mut additive_out_simd = f64x2::splat(0.0);

        for (index, target) in operator_modulation_targets.iter().enumerate().rev(){
            let target = *target;

            if operator_volume_off_simd.extract(index) {
                continue;
            }

            let mut out_simd = if operator_wave_type[index] == WaveType::Sine {
                let mono = modulation_in_pairs[index].sum();

                modulation_in_pairs[index] = tendency_pairs[index].mul_adde(
                    f64x2::splat(mono),
                    (1.0 - tendency_pairs[index]) * modulation_in_pairs[index]
                );

                let sin_input_simd: f64x2 = modulation_index_pairs[index].mul_adde(
                    feedback_pairs[index] + modulation_in_pairs[index],
                    phase_pairs[index]
                );

                SleefSin35::sin(sin_input_simd)
            } else {
                f64x2::splat((rng.gen::<f64>() - 0.5) * 2.0)
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
}