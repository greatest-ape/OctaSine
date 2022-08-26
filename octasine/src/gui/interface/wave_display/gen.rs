use duplicate::duplicate_item;
use iced_baseview::Point;

use crate::parameters::ParameterValue;
use crate::simd::*;

use super::{HEIGHT_MIDDLE, NUM_POINTS, WAVE_HEIGHT_RANGE};

use super::{OperatorData, PointArray};

pub(super) fn recalculate_canvas_points(
    lefts: &mut PointArray,
    rights: &mut PointArray,
    operator_index: usize,
    operators: &[OperatorData; 4],
) {
    let mut offset = 0;

    loop {
        let num_remaining_samples = NUM_POINTS as u64 - offset as u64;

        unsafe {
            match num_remaining_samples {
                #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                (2..) if is_x86_feature_detected!("avx") => {
                    let end_offset = offset + 2;

                    Avx::gen_segment(
                        &mut lefts[offset..end_offset],
                        &mut rights[offset..end_offset],
                        operator_index,
                        operators,
                        offset as usize,
                    );

                    offset = end_offset;
                }
                1.. => {
                    let end_offset = offset + 1;

                    cfg_if::cfg_if!(
                        if #[cfg(feature = "simd")] {
                            cfg_if::cfg_if!(
                                if #[cfg(target_arch = "x86_64")] {
                                    // SSE2 is always supported on x86_64
                                    Sse2::gen_segment(
                                        &mut lefts[offset..end_offset],
                                        &mut rights[offset..end_offset],
                                        operator_index,
                                        operators,
                                        offset as usize,
                                    );
                                } else {
                                    FallbackSleef::gen_segment(
                                        &mut lefts[offset..end_offset],
                                        &mut rights[offset..end_offset],
                                        operator_index,
                                        operators,
                                        offset as usize,
                                    );
                                }
                            )
                        } else {
                            FallbackStd::gen_segment(
                                &mut lefts[offset..end_offset],
                                &mut rights[offset..end_offset],
                                operator_index,
                                operators,
                                offset as usize,
                            );
                        }
                    );

                    offset = end_offset;
                }
                0 => {
                    break;
                }
            };
        }
    }
}

trait PathGen {
    unsafe fn gen_segment(
        lefts: &mut [Point],
        rights: &mut [Point],
        operator_index: usize,
        operator_data: &[OperatorData; 4],
        offset: usize,
    );
}

#[duplicate_item(
    [
        S [ FallbackStd ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
    ]
    [
        S [ FallbackSleef ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(all(feature = "simd")) ]
    ]
    [
        S [ Sse2 ]
        target_feature_enable [ target_feature(enable = "sse2") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
    [
        S [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
)]
mod gen {
    #[feature_gate]
    use std::f64::consts::TAU;

    #[feature_gate]
    use crate::parameters::operator_wave_type::WaveType;

    #[feature_gate]
    use crate::simd::Simd;

    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl PathGen for S {
        #[target_feature_enable]
        unsafe fn gen_segment(
            lefts: &mut [Point],
            rights: &mut [Point],
            operator_index: usize,
            operator_data: &[OperatorData; 4],
            offset: usize,
        ) {
            assert_eq!(lefts.len(), S::SAMPLES);
            assert_eq!(rights.len(), S::SAMPLES);

            let mut phases_arr = [0.0; S::PD_WIDTH];

            for sample_index in 0..S::SAMPLES {
                let phase = ((offset + sample_index) as f64) / (NUM_POINTS - 1) as f64;

                let sample_index_offset = sample_index * 2;

                phases_arr[sample_index_offset] = phase;
                phases_arr[sample_index_offset + 1] = phase;
            }

            let phases = S::pd_mul(S::pd_loadu(phases_arr.as_ptr()), S::pd_set1(TAU));

            let mut mod_inputs = [
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
            ];

            let mut out_samples = S::pd_setzero();

            let operator_frequency = operator_data[operator_index].frequency();

            for i in (operator_index..4).rev() {
                let samples = match operator_data[i].wave_type.get() {
                    WaveType::Sine => {
                        let phases = {
                            let relative_frequency =
                                S::pd_set1(operator_data[i].frequency() / operator_frequency);

                            S::pd_mul(phases, relative_frequency)
                        };

                        let feedback = S::pd_mul(
                            S::pd_fast_sin(phases),
                            S::pd_set1(operator_data[i].feedback.get() as f64),
                        );
                        let modulation_in = mod_inputs[i];

                        S::pd_fast_sin(S::pd_add(S::pd_add(feedback, modulation_in), phases))
                    }
                    WaveType::WhiteNoise => {
                        let mut samples = [0.0f64; S::PD_WIDTH];

                        for sample_index in 0..S::SAMPLES {
                            let sample_index_offset = sample_index * 2;

                            // Generate random numbers like this to get same
                            // output as in WavePicker
                            let seed = phases_arr[sample_index_offset].to_bits() + 2;
                            let random_value = fastrand::Rng::with_seed(seed).f64();

                            samples[sample_index_offset] = random_value;
                            samples[sample_index_offset + 1] = random_value;
                        }

                        S::pd_mul(
                            S::pd_sub(S::pd_loadu(samples.as_ptr()), S::pd_set1(0.5)),
                            S::pd_set1(2.0),
                        )
                    }
                };

                let samples = S::pd_mul(samples, S::pd_set1(operator_data[i].active.get() as f64));
                let samples = S::pd_mul(samples, S::pd_set1(operator_data[i].volume.get() as f64));

                let panning = S::pd_set1(operator_data[i].pan.get() as f64);

                // Channel mixing (see audio gen code for more info)
                let samples = {
                    let mono = S::pd_mul(S::pd_pairwise_horizontal_sum(samples), S::pd_set1(0.5));
                    let mono_mix_factor = mono_mix_factor(panning);

                    S::pd_add(
                        S::pd_mul(mono_mix_factor, mono),
                        S::pd_mul(S::pd_sub(S::pd_set1(1.0), mono_mix_factor), samples),
                    )
                };

                if i == operator_index {
                    let constant_power_panning = {
                        let [l, r] = operator_data[i].constant_power_panning;

                        S::pd_distribute_left_right(l as f64, r as f64)
                    };

                    out_samples = S::pd_mul(samples, constant_power_panning);

                    break;
                }

                // Store modulation outputs
                match (
                    operator_data[i].mod_out.map(|v| v.get() as f64),
                    operator_data[i].mod_targets.as_ref(),
                ) {
                    (Some(mod_out), Some(mod_targets)) if mod_out > 0.0 => {
                        let pan_factor = linear_panning_factor(panning);

                        let mod_out =
                            S::pd_mul(S::pd_mul(samples, pan_factor), S::pd_set1(mod_out));

                        for target_index in mod_targets.active_indices() {
                            mod_inputs[target_index] = S::pd_add(mod_inputs[target_index], mod_out);
                        }
                    }
                    _ => (),
                }
            }

            // Set output point y values

            let out = S::pd_sub(
                S::pd_set1(HEIGHT_MIDDLE as f64),
                S::pd_mul(out_samples, S::pd_set1(WAVE_HEIGHT_RANGE as f64)),
            );

            let mut out_arr = [0.0f64; S::PD_WIDTH];

            S::pd_storeu(out_arr.as_mut_ptr(), out);

            for sample_index in 0..S::SAMPLES {
                let sample_index_offset = sample_index * 2;

                lefts[sample_index].y = out_arr[sample_index_offset] as f32;
                rights[sample_index].y = out_arr[sample_index_offset + 1] as f32;
            }
        }
    }

    /// Linear panning. Get channel volume as number between 0.0 and 1.0
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn linear_panning_factor(
        panning: <S as Simd>::PackedDouble,
    ) -> <S as Simd>::PackedDouble {
        let factor = S::pd_interleave(S::pd_sub(S::pd_set1(1.0), panning), panning);
        let factor = S::pd_mul(factor, S::pd_set1(2.0));

        S::pd_min(factor, S::pd_set1(1.0))
    }

    /// Get amount of channel that should be derived from mono for stereo mix
    /// panning
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn mono_mix_factor(panning: <S as Simd>::PackedDouble) -> <S as Simd>::PackedDouble {
        // Get panning as value between -1 and 1
        let pan = S::pd_mul(S::pd_set1(2.0), S::pd_sub(panning, S::pd_set1(0.5)));

        S::pd_max(
            S::pd_mul(pan, S::pd_distribute_left_right(-1.0, 1.0)),
            S::pd_setzero(),
        )
    }
}
