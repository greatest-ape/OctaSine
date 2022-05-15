use arrayvec::ArrayVec;

use crate::audio::parameters::{common::AudioParameter, AudioParameterLfo};
use crate::audio::voices::lfos::VoiceLfo;
use crate::common::*;
use crate::parameter_values::{LfoParameter, Parameter};

#[derive(Default)]
pub struct LfoTargetValues(ArrayVec<(Parameter, f64), NUM_LFOS>);

impl LfoTargetValues {
    fn set_or_add(&mut self, target: Parameter, value: f64) {
        for (t, v) in self.0.iter_mut() {
            if *t == target {
                *v += value;

                return;
            }
        }

        self.0.push((target, value));
    }

    pub fn get(&self, target: Parameter) -> Option<f64> {
        for (t, v) in self.0.iter() {
            if *t == target {
                return Some(*v);
            }
        }

        None
    }
}

pub fn get_lfo_target_values(
    lfo_parameters: &mut [AudioParameterLfo; NUM_LFOS],
    voice_lfos: &mut [VoiceLfo; NUM_LFOS],
    sample_rate: SampleRate,
    time_per_sample: TimePerSample,
    bpm: BeatsPerMinute,
) -> LfoTargetValues {
    let mut lfo_values = LfoTargetValues::default();

    for (lfo_index, (voice_lfo, lfo_parameter)) in voice_lfos
        .iter_mut()
        .zip(lfo_parameters.iter_mut())
        .enumerate()
        .rev()
    {
        let target = lfo_parameter.target.get_value();

        if voice_lfo.is_stopped() | matches!(target, Parameter::None) {
            continue;
        }

        let amount = lfo_parameter.active.get_value()
            * lfo_parameter.amount.get_value_with_lfo_addition(
                lfo_values.get(Parameter::Lfo(lfo_index, LfoParameter::Amount)),
            );

        let mode = lfo_parameter.mode.get_value();
        let bpm_sync = lfo_parameter.bpm_sync.get_value();

        let shape = lfo_parameter.shape.get_value_with_lfo_addition(
            lfo_values.get(Parameter::Lfo(lfo_index, LfoParameter::Shape)),
        );
        let frequency_ratio = lfo_parameter.frequency_ratio.get_value_with_lfo_addition(
            lfo_values.get(Parameter::Lfo(lfo_index, LfoParameter::FrequencyRatio)),
        );
        let frequency_free = lfo_parameter.frequency_free.get_value_with_lfo_addition(
            lfo_values.get(Parameter::Lfo(lfo_index, LfoParameter::FrequencyFree)),
        );

        let bpm = if bpm_sync {
            bpm
        } else {
            BeatsPerMinute::one_hertz()
        };

        voice_lfo.advance_one_sample(
            sample_rate,
            time_per_sample,
            bpm,
            shape,
            mode,
            frequency_ratio * frequency_free,
        );

        let addition = voice_lfo.get_value(amount);

        lfo_values.set_or_add(target, addition);
    }

    lfo_values
}
