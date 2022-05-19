use crate::audio::parameters::{common::AudioParameter, AudioParameterLfo};
use crate::audio::voices::lfos::VoiceLfo;
use crate::common::*;
use crate::parameters::{LfoParameter, Parameter, PARAMETERS};

pub struct LfoTargetValues([Option<f64>; PARAMETERS.len()]);

impl LfoTargetValues {
    fn set_or_add(&mut self, target: u8, value: f64) {
        match &mut self.0[target as usize] {
            Some(v) => *v += value,
            v @ None => *v = Some(value),
        }
    }

    pub fn get(&self, target: u8) -> Option<f64> {
        self.0[target as usize]
    }
}

impl Default for LfoTargetValues {
    fn default() -> Self {
        Self([None; PARAMETERS.len()])
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

    const AMOUNT_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::Amount.init_index_array();
    const SHAPE_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::Shape.init_index_array();
    const RATIO_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::FrequencyRatio.init_index_array();
    const FREE_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::FrequencyFree.init_index_array();

    for (lfo_index, (voice_lfo, lfo_parameter)) in voice_lfos
        .iter_mut()
        .zip(lfo_parameters.iter_mut())
        .enumerate()
        .rev()
    {
        assert!(lfo_index < NUM_LFOS);

        let target = lfo_parameter.target.get_value();

        if voice_lfo.is_stopped() | matches!(target, Parameter::None) {
            continue;
        }

        let amount = lfo_parameter.active.get_value()
            * lfo_parameter
                .amount
                .get_value_with_lfo_addition(lfo_values.get(AMOUNT_PARAMETER_INDICES[lfo_index]));

        let mode = lfo_parameter.mode.get_value();
        let bpm_sync = lfo_parameter.bpm_sync.get_value();

        let shape = lfo_parameter
            .shape
            .get_value_with_lfo_addition(lfo_values.get(SHAPE_PARAMETER_INDICES[lfo_index]));
        let frequency_ratio = lfo_parameter
            .frequency_ratio
            .get_value_with_lfo_addition(lfo_values.get(RATIO_PARAMETER_INDICES[lfo_index]));
        let frequency_free = lfo_parameter
            .frequency_free
            .get_value_with_lfo_addition(lfo_values.get(FREE_PARAMETER_INDICES[lfo_index]));

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

        lfo_values.set_or_add(target.to_index(), addition);
    }

    lfo_values
}
