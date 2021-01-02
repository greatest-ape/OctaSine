//! Fallback audio generation not requiring simd

use arrayvec::ArrayVec;
use fastrand::Rng;
use vst::buffer::AudioBuffer;

use crate::approximations::*;
use crate::common::*;
use crate::constants::*;
use crate::voices::*;
use crate::parameters::processing::*;
use crate::OctaSine;


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


#[derive(Default)]
pub struct LfoTargetValues(ArrayVec<[(LfoTargetParameter, f64); NUM_LFOS]>);


impl LfoTargetValues {
    fn set_or_add(
        &mut self,
        target: LfoTargetParameter,
        value: f64
    ){
        for (t, v) in self.0.iter_mut(){
            if *t == target {
                *v += value;

                return;
            }
        }

        self.0.push((target, value));
    }

    fn get(&mut self, target: LfoTargetParameter) -> Option<f64> {
        for (t, v) in self.0.iter() {
            if *t == target {
                return Some(*v)
            }
        }

        None
    }
}


#[inline]
fn hard_limit(value: f64) -> f64 {
    value.min(1.0).max(-1.0)
}


#[inline]
pub fn process_f32(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>){
    let bpm = octasine.get_bpm();

    let mut outputs = audio_buffer.split().1;
    let lefts = outputs.get_mut(0).iter_mut();
    let rights = outputs.get_mut(1).iter_mut();

    for (buffer_sample_left, buffer_sample_right) in lefts.zip(rights){
        let (left, right) = gen_samples_for_voices(octasine, bpm);

        *buffer_sample_left = left as f32;
        *buffer_sample_right = right as f32;
    }
}


#[inline]
pub fn gen_samples_for_voices(
    octasine: &mut OctaSine,
    bpm: BeatsPerMinute
) -> (f64, f64) {
    let changed_preset_parameters = octasine.sync.presets
        .get_changed_parameters_from_processing();

    if let Some(indeces) = changed_preset_parameters {
        for (index, opt_new_value) in indeces.iter().enumerate(){
            if let Some(new_value) = opt_new_value {
                octasine.processing.parameters.set_from_sync(index, *new_value);
            }
        }
    }

    let time_per_sample = octasine.processing.time_per_sample;

    let mut voice_sum_left: f64 = 0.0;
    let mut voice_sum_right: f64 = 0.0;

    for voice in octasine.processing.voices.iter_mut(){
        if voice.active {
            let (out_left, out_right) = generate_voice_samples(
                &octasine.processing.log10_table,
                &mut octasine.processing.rng,
                octasine.processing.global_time,
                time_per_sample,
                &mut octasine.processing.parameters,
                bpm,
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
    rng: &mut Rng,
    time: TimeCounter,
    time_per_sample: TimePerSample,
    parameters: &mut ProcessingParameters,
    bpm: BeatsPerMinute,
    voice: &mut Voice,
) -> (f64, f64) {
    let mut lfo_values = LfoTargetValues::default();

    for (voice_lfo, lfo_parameter) in voice.lfos.iter_mut()
        .zip(parameters.lfos.iter_mut())
    {
        let shape = lfo_parameter.shape.value;
        let mode = lfo_parameter.mode.value;
        let bpm_sync = lfo_parameter.bpm_sync.value;
        let speed = lfo_parameter.speed.value;
        let magnitude = lfo_parameter.magnitude.get_value(time);

        let bpm = if bpm_sync {
            bpm
        } else {
            BeatsPerMinute::default()
        };

        let addition = voice_lfo.get_value(
            time_per_sample,
            bpm,
            shape,
            mode,
            speed,
            magnitude,
        );

        let target = lfo_parameter.target_parameter.value;

        lfo_values.set_or_add(target, addition);
    }

    let base_frequency = voice.midi_pitch.get_frequency(
        parameters.master_frequency.get_value_with_lfo_addition(
            (),
            lfo_values.get(LfoTargetParameter::Master(LfoTargetMasterParameter::Frequency))
        )
    );

    let mut output_channels = [
        OutputChannel::default(),
        OutputChannel::default()
    ];

    let operators = &mut parameters.operators;

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
        // original stereo signals and mix in mono. Require exact f64 identity
        // for this optimization for now.
        #[allow(clippy::float_cmp)] 
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

                // Require exact identity for now, to prevent possible artifacts
                #[allow(clippy::float_cmp)]
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
                    (rng.f64() - 0.5) * 2.0;

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

    let master_volume = parameters.master_volume.get_value_with_lfo_addition(
        time,
        lfo_values.get(LfoTargetParameter::Master(LfoTargetMasterParameter::Volume))
    );

    let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 * master_volume;

    (signal_left * volume_factor, signal_right * volume_factor)
}