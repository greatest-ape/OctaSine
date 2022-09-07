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

                    AvxPackedDouble::gen_segment(
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
                            FallbackPackedDoubleSleef::gen_segment(
                                &mut lefts[offset..end_offset],
                                &mut rights[offset..end_offset],
                                operator_index,
                                operators,
                                offset as usize,
                            );
                        } else {
                            FallbackPackedDoubleStd::gen_segment(
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
        Pd [ FallbackPackedDoubleStd ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
    ]
    [
        Pd [ FallbackPackedDoubleSleef ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(all(feature = "simd")) ]
    ]
    [
        Pd [ AvxPackedDouble ]
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
    use crate::simd::SimdPackedDouble;

    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl PathGen for Pd {
        #[target_feature_enable]
        unsafe fn gen_segment(
            lefts: &mut [Point],
            rights: &mut [Point],
            operator_index: usize,
            operator_data: &[OperatorData; 4],
            offset: usize,
        ) {
            assert_eq!(lefts.len(), Pd::SAMPLES);
            assert_eq!(rights.len(), Pd::SAMPLES);

            let mut phases_arr = [0.0; Pd::PD_WIDTH];

            for sample_index in 0..Pd::SAMPLES {
                let phase = ((offset + sample_index) as f64) / (NUM_POINTS - 1) as f64;

                let sample_index_offset = sample_index * 2;

                phases_arr[sample_index_offset] = phase;
                phases_arr[sample_index_offset + 1] = phase;
            }

            let phases = Pd::load_ptr(phases_arr.as_ptr()) * Pd::new(TAU);

            let mut mod_inputs = [
                Pd::new_zeroed(),
                Pd::new_zeroed(),
                Pd::new_zeroed(),
                Pd::new_zeroed(),
            ];

            let mut out_samples = Pd::new_zeroed();

            let operator_frequency = operator_data[operator_index].frequency();

            for i in (operator_index..4).rev() {
                let samples = match operator_data[i].wave_type.get() {
                    WaveType::Sine => {
                        let phases = {
                            let relative_frequency =
                                Pd::new(operator_data[i].frequency() / operator_frequency);

                            phases * relative_frequency
                        };

                        let feedback = Pd::new(operator_data[i].feedback.get() as f64);
                        let modulation_in = mod_inputs[i];

                        ((feedback * phases.fast_sin()) + modulation_in + phases).fast_sin()
                    }
                    WaveType::WhiteNoise => {
                        let mut random_numbers = [0.0f64; Pd::PD_WIDTH];

                        for sample_index in 0..Pd::SAMPLES {
                            let sample_index_offset = sample_index * 2;

                            // Generate random numbers like this to get same
                            // output as in WavePicker
                            let seed = phases_arr[sample_index_offset].to_bits() + 2;
                            let random_value = fastrand::Rng::with_seed(seed).f64();

                            random_numbers[sample_index_offset] = random_value;
                            random_numbers[sample_index_offset + 1] = random_value;
                        }

                        // Convert random numbers to range -1.0 to 1.0
                        Pd::new(2.0) * (Pd::from_arr(random_numbers) - Pd::new(0.5))
                    }
                };

                let samples = samples
                    * Pd::new(operator_data[i].active.get() as f64)
                    * Pd::new(operator_data[i].volume.get() as f64);

                let panning = Pd::new(operator_data[i].pan.get() as f64);

                // Channel mixing (see audio gen code for more info)
                let samples = {
                    let mono_mix_factor = mono_mix_factor(panning);
                    let mono = samples.pairwise_horizontal_sum() * Pd::new(0.5);

                    (mono_mix_factor * mono) + ((Pd::new(1.0) - mono_mix_factor) * samples)
                };

                if i == operator_index {
                    let constant_power_panning = {
                        let [l, r] = operator_data[i].constant_power_panning;

                        Pd::new_from_pair(l as f64, r as f64)
                    };

                    out_samples = samples * constant_power_panning;

                    break;
                }

                // Store modulation outputs
                match (
                    operator_data[i].mod_out.map(|v| v.get() as f64),
                    operator_data[i].mod_targets.as_ref(),
                ) {
                    (Some(mod_out), Some(mod_targets)) if mod_out > 0.0 => {
                        let pan_factor = linear_panning_factor(panning);

                        let mod_out = samples * pan_factor * Pd::new(mod_out);

                        for target_index in mod_targets.active_indices() {
                            mod_inputs[target_index] += mod_out;
                        }
                    }
                    _ => (),
                }
            }

            // Set output point y values

            let out =
                Pd::new(HEIGHT_MIDDLE as f64) - (out_samples * Pd::new(WAVE_HEIGHT_RANGE as f64));

            let out_arr = out.to_arr();

            for sample_index in 0..Pd::SAMPLES {
                let sample_index_offset = sample_index * 2;

                lefts[sample_index].y = out_arr[sample_index_offset] as f32;
                rights[sample_index].y = out_arr[sample_index_offset + 1] as f32;
            }
        }
    }

    /// Linear panning. Get channel volume as number between 0.0 and 1.0
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn linear_panning_factor(panning: Pd) -> Pd {
        ((Pd::new(1.0) - panning).interleave(panning) * Pd::new(2.0)).min(Pd::new(1.0))
    }

    /// Get amount of channel that should be derived from mono for stereo mix
    /// panning
    #[feature_gate]
    #[target_feature_enable]
    unsafe fn mono_mix_factor(panning: Pd) -> Pd {
        // Get panning as value between -1 and 1
        let pan = Pd::new(2.0) * (panning - Pd::new(0.5));

        (pan * Pd::new_from_pair(-1.0, 1.0)).max(Pd::new_zeroed())
    }
}
