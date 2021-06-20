mod lfo;

#[cfg(feature = "simd")]
use core::arch::x86_64::*;

use duplicate::duplicate;
use vst::buffer::AudioBuffer;

use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::ProcessingParameter;
use crate::OctaSine;

use lfo::*;

/// Each SAMPLE_PASS_SIZE samples, load parameter changes and processing
/// parameter values (interpolated values where applicable)
const SAMPLE_PASS_SIZE: usize = 16;

#[inline]
pub fn process_f32_runtime_select(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>) {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(feature = "simd")] {
                cfg_if::cfg_if! {
                    if #[cfg(target_arch = "x86_64")] {
                        if is_x86_feature_detected!("avx") {
                            Avx::process_f32(octasine, audio_buffer);
                        } else {
                            // SSE2 is always supported on x86_64
                            Sse2::process_f32(octasine, audio_buffer);
                        }
                    } else {
                        FallbackSleef::process_f32(octasine, audio_buffer);
                    }
                }
            } else {
                FallbackStd::process_f32(octasine, audio_buffer);
            }
        }
    }
}

pub trait AudioGen {
    unsafe fn process_f32(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>);
}

pub trait FallbackSine {
    fn sin(a: [f64; 2]) -> [f64; 2];
}

pub struct FallbackSineStd;

impl FallbackSine for FallbackSineStd {
    fn sin([a1, a2]: [f64; 2]) -> [f64; 2] {
        [a1.sin(), a2.sin()]
    }
}

#[cfg(feature = "simd")]
pub struct FallbackSineSleef;

#[cfg(feature = "simd")]
impl FallbackSine for FallbackSineSleef {
    fn sin([a1, a2]: [f64; 2]) -> [f64; 2] {
        unsafe {
            [
                sleef_sys::Sleef_cinz_sind1_u35purec(a1),
                sleef_sys::Sleef_cinz_sind1_u35purec(a2),
            ]
        }
    }
}

pub trait Simd {
    type PackedDouble;
    const PD_WIDTH: usize;

    unsafe fn pd_set1(value: f64) -> Self::PackedDouble;
    unsafe fn pd_loadu(source: *const f64) -> Self::PackedDouble;
    unsafe fn pd_storeu(target: *mut f64, a: Self::PackedDouble);
    unsafe fn pd_add(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_sub(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_mul(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_min(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_max(a: Self::PackedDouble, b: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_fast_sin(a: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_pairwise_horizontal_sum(a: Self::PackedDouble) -> Self::PackedDouble;
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> Self::PackedDouble;
    unsafe fn pd_first_over_zero_limit(volume: Self::PackedDouble) -> bool;
}

pub struct Fallback<T> {
    phantom_data: ::std::marker::PhantomData<T>,
}

impl<T: FallbackSine> Simd for Fallback<T> {
    type PackedDouble = [f64; 2];
    const PD_WIDTH: usize = 2;

    unsafe fn pd_set1(value: f64) -> [f64; 2] {
        [value, value]
    }
    unsafe fn pd_loadu(source: *const f64) -> [f64; 2] {
        *(source as *const [f64; 2])
    }
    unsafe fn pd_storeu(target: *mut f64, a: [f64; 2]) {
        ::std::ptr::write(target as *mut [f64; 2], a);
    }
    unsafe fn pd_add([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 + b1, a2 + b2]
    }
    unsafe fn pd_sub([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 - b1, a2 - b2]
    }
    unsafe fn pd_mul([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1 * b1, a2 * b2]
    }
    unsafe fn pd_min([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1.min(b1), a2.min(b2)]
    }
    unsafe fn pd_max([a1, a2]: [f64; 2], [b1, b2]: [f64; 2]) -> [f64; 2] {
        [a1.max(b1), a2.max(b2)]
    }
    unsafe fn pd_fast_sin(a: [f64; 2]) -> [f64; 2] {
        T::sin(a)
    }
    unsafe fn pd_pairwise_horizontal_sum([l, r]: [f64; 2]) -> [f64; 2] {
        [l + r, l + r]
    }
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> [f64; 2] {
        [l, r]
    }
    unsafe fn pd_first_over_zero_limit([first_volume, _]: [f64; 2]) -> bool {
        first_volume > ZERO_VALUE_LIMIT
    }
}

pub struct Sse2;

#[cfg(feature = "simd")]
impl Simd for Sse2 {
    type PackedDouble = __m128d;
    const PD_WIDTH: usize = 2;

    #[target_feature(enable = "sse2")]
    unsafe fn pd_set1(value: f64) -> __m128d {
        _mm_set1_pd(value)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_loadu(source: *const f64) -> __m128d {
        _mm_loadu_pd(source)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_storeu(target: *mut f64, a: __m128d) {
        _mm_storeu_pd(target, a)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_add(a: __m128d, b: __m128d) -> __m128d {
        _mm_add_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_sub(a: __m128d, b: __m128d) -> __m128d {
        _mm_sub_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_mul(a: __m128d, b: __m128d) -> __m128d {
        _mm_mul_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_min(a: __m128d, b: __m128d) -> __m128d {
        _mm_min_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_max(a: __m128d, b: __m128d) -> __m128d {
        _mm_max_pd(a, b)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_fast_sin(a: __m128d) -> __m128d {
        sleef_sys::Sleef_cinz_sind2_u35sse2(a)
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_pairwise_horizontal_sum(a: __m128d) -> __m128d {
        _mm_add_pd(a, _mm_shuffle_pd(a, a, 0b01))
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> __m128d {
        let lr = [l, r];

        _mm_loadu_pd(&lr[0])
    }
    #[target_feature(enable = "sse2")]
    unsafe fn pd_first_over_zero_limit(volume: __m128d) -> bool {
        let mut volume_tmp = [0.0f64; 2];

        _mm_storeu_pd(&mut volume_tmp[0], volume);

        volume_tmp[0] > ZERO_VALUE_LIMIT
    }
}

pub struct Avx;

#[cfg(feature = "simd")]
impl Simd for Avx {
    type PackedDouble = __m256d;
    const PD_WIDTH: usize = 4;

    #[target_feature(enable = "avx")]
    unsafe fn pd_set1(value: f64) -> __m256d {
        _mm256_set1_pd(value)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_loadu(source: *const f64) -> __m256d {
        _mm256_loadu_pd(source)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_storeu(target: *mut f64, a: __m256d) {
        _mm256_storeu_pd(target, a)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_add(a: __m256d, b: __m256d) -> __m256d {
        _mm256_add_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_sub(a: __m256d, b: __m256d) -> __m256d {
        _mm256_sub_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_mul(a: __m256d, b: __m256d) -> __m256d {
        _mm256_mul_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_min(a: __m256d, b: __m256d) -> __m256d {
        _mm256_min_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_max(a: __m256d, b: __m256d) -> __m256d {
        _mm256_max_pd(a, b)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_fast_sin(a: __m256d) -> __m256d {
        sleef_sys::Sleef_cinz_sind4_u35avx(a)
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_pairwise_horizontal_sum(a: __m256d) -> __m256d {
        _mm256_add_pd(a, _mm256_permute_pd(a, 0b0101))
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_distribute_left_right(l: f64, r: f64) -> __m256d {
        let lr = [l, r, l, r];

        _mm256_loadu_pd(&lr[0])
    }
    #[target_feature(enable = "avx")]
    unsafe fn pd_first_over_zero_limit(volume: __m256d) -> bool {
        let mut volume_tmp = [0.0f64; 4];

        _mm256_storeu_pd(&mut volume_tmp[0], volume);

        volume_tmp[0] > ZERO_VALUE_LIMIT
    }
}

pub type FallbackStd = Fallback<FallbackSineStd>;
#[cfg(feature = "simd")]
pub type FallbackSleef = Fallback<FallbackSineSleef>;

#[duplicate(
    [
        S [ FallbackStd ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
    ]
    [
        S [ FallbackSleef ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(feature = "simd") ]
    ]
    [
        S [ Sse2 ]
        target_feature_enable [ target_feature(enable = "sse2") ]
        feature_gate [ cfg(feature = "simd") ]
    ]
    [
        S [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        feature_gate [ cfg(feature = "simd") ]
    ]
)]
mod gen {
    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl AudioGen for S {
        #[target_feature_enable]
        unsafe fn process_f32(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>) {
            let num_samples = audio_buffer.samples();

            let mut audio_buffer_outputs = audio_buffer.split().1;
            let audio_buffer_lefts = audio_buffer_outputs.get_mut(0);
            let audio_buffer_rights = audio_buffer_outputs.get_mut(1);

            let num_passes = num_samples / SAMPLE_PASS_SIZE;

            for i in 0..num_passes {
                let start = i * SAMPLE_PASS_SIZE;
                let end = start + SAMPLE_PASS_SIZE;

                run_pass(
                    octasine,
                    &mut audio_buffer_lefts[start..end],
                    &mut audio_buffer_rights[start..end],
                )
            }
        }
    }

    #[feature_gate]
    #[target_feature_enable]
    unsafe fn run_pass(
        octasine: &mut OctaSine,
        audio_buffer_lefts: &mut [f32],
        audio_buffer_rights: &mut [f32],
    ) {
        // --- Update processing parameters from preset parameters

        let changed_sync_parameters = octasine
            .sync
            .presets
            .get_changed_parameters_from_processing();

        if let Some(indeces) = changed_sync_parameters {
            for (index, opt_new_value) in indeces.iter().enumerate() {
                if let Some(new_value) = opt_new_value {
                    octasine
                        .processing
                        .parameters
                        .set_from_sync(index, *new_value);
                }
            }
        }

        // --- Set some generally useful variables

        let bpm = octasine.get_bpm();

        let operators = &mut octasine.processing.parameters.operators;

        let time_per_sample = octasine.processing.time_per_sample;
        let time = octasine.processing.global_time;
        let time_advancement = time_per_sample.0 * (SAMPLE_PASS_SIZE as f64);

        // Necessary for interpolation
        octasine.processing.global_time.0 += time_advancement;

        // --- Collect parameter data and do audio generation

        // FIXME: optimize section, possibly with simd. Maybe envelopes can be calculated less often

        // Maybe operator indexes should be inversed (3 - operator_index)
        // because that is how they will be accessed later.

        // Sample pass size * 2 because of two channels. Even index = left channel
        let mut summed_additive_outputs = [0.0f64; SAMPLE_PASS_SIZE * 2];

        for voice in octasine
            .processing
            .voices
            .iter_mut()
            .filter(|voice| voice.active)
        {
            // --- Get voice data

            let lfo_values = get_lfo_target_values(
                &mut octasine.processing.parameters.lfos,
                &mut voice.lfos,
                time,
                time_advancement,
                bpm,
            );

            let voice_base_frequency = voice.midi_pitch.get_frequency(
                octasine
                    .processing
                    .parameters
                    .master_frequency
                    .get_value_with_lfo_addition(
                        (),
                        lfo_values.get(LfoTargetParameter::Master(
                            LfoTargetMasterParameter::Frequency,
                        )),
                    ),
            );

            // Interpolated
            let mut operator_volume: [f64; 4] = [0.0; 4];
            let mut operator_modulation_index = [0.0f64; 4];
            let mut operator_feedback: [f64; 4] = [0.0; 4];
            let mut operator_panning: [f64; 4] = [0.0; 4];
            let mut operator_additive: [f64; 4] = [0.0; 4];

            // Not interpolated
            let mut operator_wave_type = [WaveType::Sine; 4];
            let mut operator_frequency: [f64; 4] = [voice_base_frequency; 4];
            let mut operator_modulation_targets = [0usize; 4];

            let mut operator_envelope_volumes = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];
            let mut operator_phases = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

            for (index, operator) in operators.iter_mut().enumerate() {
                operator_volume[index] = operator.volume.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::Volume,
                    )),
                );
                operator_modulation_index[index] =
                    operator.modulation_index.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            index,
                            LfoTargetOperatorParameter::ModulationIndex,
                        )),
                    );
                operator_feedback[index] = operator.feedback.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::Feedback,
                    )),
                );
                operator_panning[index] = operator.panning.get_value_with_lfo_addition(
                    time,
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::Panning,
                    )),
                );

                // Get additive factor; use 1.0 for operator 1
                operator_additive[index] = if index == 0 {
                    1.0
                } else {
                    operator.additive_factor.get_value_with_lfo_addition(
                        time,
                        lfo_values.get(LfoTargetParameter::Operator(
                            index,
                            LfoTargetOperatorParameter::Additive,
                        )),
                    )
                };

                operator_wave_type[index] = operator.wave_type.value;

                let frequency_ratio = operator.frequency_ratio.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::FrequencyRatio,
                    )),
                );
                let frequency_free = operator.frequency_free.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::FrequencyFree,
                    )),
                );
                let frequency_fine = operator.frequency_fine.get_value_with_lfo_addition(
                    (),
                    lfo_values.get(LfoTargetParameter::Operator(
                        index,
                        LfoTargetOperatorParameter::FrequencyFine,
                    )),
                );

                operator_frequency[index] *= frequency_ratio * frequency_free * frequency_fine;

                if let Some(p) = &mut operator.output_operator {
                    operator_modulation_targets[index] = p.get_value();
                }
            }

            // Envelope
            for i in 0..SAMPLE_PASS_SIZE {
                for (operator_index, operator) in operators.iter_mut().enumerate() {
                    let v = voice.operators[operator_index].volume_envelope.get_volume(
                        &octasine.processing.log10_table,
                        &operator.volume_envelope,
                        voice.key_pressed,
                        voice.duration,
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
                let frequency = operator_frequency[operator_index];
                let phase_addition = frequency * time_per_sample.0;

                let mut new_phase = 0.0;

                for i in 0..SAMPLE_PASS_SIZE {
                    // Do multiplication instead of successive addition for
                    // less precision loss (hopefully)
                    new_phase = last_phase + phase_addition * ((i + 1) as f64);

                    let j = i * 2;

                    operator_phases[operator_index][j] = new_phase;
                    operator_phases[operator_index][j + 1] = new_phase;
                }

                // Save phase
                voice.operators[operator_index].last_phase.0 = new_phase;
            }

            let voice_volume_factor_splat = {
                let lfo_parameter = LfoTargetParameter::Master(LfoTargetMasterParameter::Volume);
                let lfo_addition = lfo_values.get(lfo_parameter);

                let master_volume = octasine
                    .processing
                    .parameters
                    .master_volume
                    .get_value_with_lfo_addition(time, lfo_addition);

                let key_velocity = voice.key_velocity.0;

                S::pd_set1(VOICE_VOLUME_FACTOR * master_volume * key_velocity)
            };

            let operator_generate_audio = run_operator_dependency_analysis(
                operator_volume,
                operator_additive,
                operator_modulation_index,
                operator_wave_type,
                operator_modulation_targets,
            );

            voice.deactivate_if_envelopes_ended();

            // --- Generate samples for all operators

            // Voice modulation input storage, indexed by operator
            let mut voice_modulation_inputs = [[0.0f64; SAMPLE_PASS_SIZE * 2]; 4];

            // Go through operators downwards, starting with operator 4
            for operator_index in 0..4 {
                // FIXME: better iterator with 3, 2, 1, 0 possible?
                let operator_index = 3 - operator_index;

                // Possibly skip generation based on previous dependency analysis
                if !operator_generate_audio[operator_index] {
                    continue;
                }

                if operator_wave_type[operator_index] == WaveType::WhiteNoise {
                    let random_numbers = {
                        let mut random_numbers = [0.0f64; SAMPLE_PASS_SIZE * 2];

                        for i in 0..SAMPLE_PASS_SIZE {
                            let random = (octasine.processing.rng.f64() - 0.5) * 2.0;

                            let j = i * 2;

                            random_numbers[j] = random;
                            random_numbers[j + 1] = random;
                        }

                        random_numbers
                    };

                    let modulation_target = operator_modulation_targets[operator_index];

                    let constant_power_panning = {
                        let [l, r] = operators[operator_index].panning.left_and_right;

                        S::pd_distribute_left_right(l, r)
                    };

                    let operator_volume_splat = S::pd_set1(operator_volume[operator_index]);
                    let operator_additive_splat = S::pd_set1(operator_additive[operator_index]);

                    for i in (0..SAMPLE_PASS_SIZE * 2).step_by(S::PD_WIDTH) {
                        let envelope_volume =
                            S::pd_loadu(&operator_envelope_volumes[operator_index][i]);
                        let volume_product = S::pd_mul(operator_volume_splat, envelope_volume);

                        let sample = S::pd_loadu(&random_numbers[i]);

                        let sample_adjusted =
                            S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                        let additive_out = S::pd_mul(sample_adjusted, operator_additive_splat);
                        let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                        // Add modulation output to target operator's modulation inputs
                        let modulation_sum = S::pd_add(
                            S::pd_loadu(&voice_modulation_inputs[modulation_target][i]),
                            modulation_out,
                        );
                        S::pd_storeu(
                            &mut voice_modulation_inputs[modulation_target][i],
                            modulation_sum,
                        );

                        // Add additive output to summed_additive_outputs
                        let summed_plus_new = S::pd_add(
                            S::pd_loadu(&summed_additive_outputs[i]),
                            S::pd_mul(additive_out, voice_volume_factor_splat),
                        );
                        S::pd_storeu(&mut summed_additive_outputs[i], summed_plus_new);
                    }
                } else {
                    // --- Setup operator SIMD vars

                    let operator_volume_splat = S::pd_set1(operator_volume[operator_index]);
                    let operator_feedback_splat = S::pd_set1(operator_feedback[operator_index]);
                    let operator_additive_splat = S::pd_set1(operator_additive[operator_index]);
                    let operator_modulation_index_splat =
                        S::pd_set1(operator_modulation_index[operator_index]);

                    let (pan_tendency, one_minus_pan_tendency) = {
                        // Get panning as value between -1 and 1
                        let pan_transformed = 2.0 * (operator_panning[operator_index] - 0.5);

                        let r = pan_transformed.max(0.0);
                        let l = (pan_transformed * -1.0).max(0.0);

                        let tendency = S::pd_distribute_left_right(l, r);
                        let one_minus_tendency = S::pd_sub(S::pd_set1(1.0), tendency);

                        (tendency, one_minus_tendency)
                    };

                    let constant_power_panning = {
                        let [l, r] = operators[operator_index].panning.left_and_right;

                        S::pd_distribute_left_right(l, r)
                    };

                    let modulation_target = operator_modulation_targets[operator_index];

                    // --- Create samples for both channels

                    let tau_splat = S::pd_set1(TAU);

                    for i in (0..SAMPLE_PASS_SIZE * 2).step_by(S::PD_WIDTH) {
                        let envelope_volume =
                            S::pd_loadu(&operator_envelope_volumes[operator_index][i]);
                        let volume_product = S::pd_mul(operator_volume_splat, envelope_volume);

                        // Skip generation when envelope volume or operator
                        // volume is zero. Helps performance when operator
                        // envelope lengths vary a lot. Otherwise, the
                        // branching probably negatively impacts performance.
                        // Higher indeces don't really matter: if previous
                        // sample has zero envelope volume, next one probably
                        // does too. The worst case scenario is that attacks
                        // are a tiny bit slower.
                        if !S::pd_first_over_zero_limit(volume_product) {
                            continue;
                        }

                        let phase =
                            S::pd_mul(S::pd_loadu(&operator_phases[operator_index][i]), tau_splat);

                        let modulation_in_for_channel =
                            S::pd_loadu(&voice_modulation_inputs[operator_index][i]);
                        let modulation_in_channel_sum =
                            S::pd_pairwise_horizontal_sum(modulation_in_for_channel);
                        // Weird modulation input panning
                        // Mix modulator into current operator depending on
                        // panning of current operator. If panned to the
                        // middle, just pass through the stereo signals. If
                        // panned to any side, mix out the original stereo
                        // signals and mix in mono.
                        // Note: breaks unless S::PD_WIDTH >= 2
                        let modulation_in = S::pd_add(
                            S::pd_mul(pan_tendency, modulation_in_channel_sum),
                            S::pd_mul(one_minus_pan_tendency, modulation_in_for_channel),
                        );

                        let feedback = S::pd_mul(operator_feedback_splat, S::pd_fast_sin(phase));

                        let sin_input = S::pd_add(
                            S::pd_mul(
                                operator_modulation_index_splat,
                                S::pd_add(feedback, modulation_in),
                            ),
                            phase,
                        );

                        let sample = S::pd_fast_sin(sin_input);

                        let sample_adjusted =
                            S::pd_mul(S::pd_mul(sample, volume_product), constant_power_panning);
                        let additive_out = S::pd_mul(sample_adjusted, operator_additive_splat);
                        let modulation_out = S::pd_sub(sample_adjusted, additive_out);

                        // Add modulation output to target operator's modulation inputs
                        let modulation_sum = S::pd_add(
                            S::pd_loadu(&voice_modulation_inputs[modulation_target][i]),
                            modulation_out,
                        );
                        S::pd_storeu(
                            &mut voice_modulation_inputs[modulation_target][i],
                            modulation_sum,
                        );

                        // Add additive output to summed_additive_outputs
                        let summed_plus_new = S::pd_add(
                            S::pd_loadu(&summed_additive_outputs[i]),
                            S::pd_mul(additive_out, voice_volume_factor_splat),
                        );
                        S::pd_storeu(&mut summed_additive_outputs[i], summed_plus_new);
                    } // End of sample pass size *  2 iteration
                }
            } // End of operator iteration
        } // End of voice iteration

        // --- Summed additive outputs: apply hard limit.

        let max_value_splat = S::pd_set1(5.0);
        let min_value_splat = S::pd_set1(-5.0);

        for chunk in summed_additive_outputs.chunks_exact_mut(S::PD_WIDTH) {
            let additive = S::pd_loadu(&chunk[0]);

            let additive = S::pd_min(additive, max_value_splat);
            let additive = S::pd_max(additive, min_value_splat);

            S::pd_storeu(&mut chunk[0], additive);
        }

        // --- Write additive outputs to audio buffer

        for i in 0..SAMPLE_PASS_SIZE {
            let j = i * 2;
            audio_buffer_lefts[i] = summed_additive_outputs[j] as f32;
            audio_buffer_rights[i] = summed_additive_outputs[j + 1] as f32;
        }
    }

    /// Operator dependency analysis to allow skipping audio generation when possible
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn run_operator_dependency_analysis(
        operator_volume: [f64; 4],
        operator_additive: [f64; 4],
        operator_modulation_index: [f64; 4],
        operator_wave_type: [WaveType; 4],
        operator_modulation_targets: [usize; 4],
    ) -> [bool; 4] {
        let mut operator_generate_audio = [true; 4];
        let mut operator_additive_zero = [false; 4];
        let mut operator_modulation_index_zero = [false; 4];

        for operator_index in 0..4 {
            // If volume is off, just set to skippable, don't even bother with lt calculations
            if operator_volume[operator_index].lt(&ZERO_VALUE_LIMIT) {
                operator_generate_audio[operator_index] = false;
            } else {
                operator_additive_zero[operator_index] =
                    operator_additive[operator_index].lt(&ZERO_VALUE_LIMIT);

                operator_modulation_index_zero[operator_index] =
                    operator_modulation_index[operator_index].lt(&ZERO_VALUE_LIMIT);
            }
        }

        for _ in 0..3 {
            for operator_index in 1..4 {
                let modulation_target = operator_modulation_targets[operator_index];

                // Skip generation if operator was previously determined to be skippable OR
                let skip_condition = !operator_generate_audio[operator_index]
                    || (
                        // Additive factor for this operator is off AND
                        operator_additive_zero[operator_index]
                            && (
                                // Modulation target was previously determined to be skippable OR
                                !operator_generate_audio[modulation_target] ||
                        // Modulation target is white noise OR
                        operator_wave_type[modulation_target] == WaveType::WhiteNoise ||
                        // Modulation target doesn't do anything with its input modulation
                        operator_modulation_index_zero[modulation_target]
                            )
                    );

                if skip_condition {
                    operator_generate_audio[operator_index] = false;
                }
            }
        }

        operator_generate_audio
    }
}
