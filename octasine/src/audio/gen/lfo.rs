use arrayvec::ArrayVec;

use crate::audio::parameters::{common::AudioParameter, LfoAudioParameters};
use crate::audio::voices::lfos::VoiceLfo;
use crate::common::*;
use crate::parameters::{LfoParameter, PARAMETERS};

pub struct LfoTargetValues {
    values: [Option<f32>; PARAMETERS.len()],
    set_indices: ArrayVec<u8, NUM_LFOS>,
}

impl Default for LfoTargetValues {
    fn default() -> Self {
        Self {
            values: [None; PARAMETERS.len()],
            set_indices: Default::default(),
        }
    }
}

impl LfoTargetValues {
    pub fn get(&self, target: u8) -> Option<f32> {
        self.values[target as usize]
    }

    fn set_or_add(&mut self, target: u8, value: f32) {
        match &mut self.values[target as usize] {
            Some(v) => *v += value,
            v @ None => {
                *v = Some(value);

                self.set_indices.push(target);
            }
        }
    }

    fn clear_set(&mut self) {
        for i in self.set_indices.iter().copied() {
            self.values[i as usize] = None;
        }

        self.set_indices.clear();
    }
}

pub fn update_lfo_target_values(
    lfo_values: &mut LfoTargetValues,
    lfo_parameters: &mut [LfoAudioParameters; NUM_LFOS],
    voice_lfos: &mut [VoiceLfo; NUM_LFOS],
    sample_rate: SampleRate,
    time_per_sample: TimePerSample,
    bpm: BeatsPerMinute,
) {
    const AMOUNT_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::Amount.init_index_array();
    const SHAPE_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::Shape.init_index_array();
    const RATIO_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::FrequencyRatio.init_index_array();
    const FREE_PARAMETER_INDICES: [u8; NUM_LFOS] = LfoParameter::FrequencyFree.init_index_array();

    lfo_values.clear_set();

    for (lfo_index, (voice_lfo, lfo_parameter)) in voice_lfos
        .iter_mut()
        .zip(lfo_parameters.iter_mut())
        .enumerate()
        .rev()
    {
        assert!(lfo_index < NUM_LFOS);

        let target_index = lfo_parameter.target.get_value().index();

        let target_index = match (target_index, voice_lfo.is_stopped()) {
            (None, _) | (_, true) => continue,
            (Some(index), false) => index,
        };

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

        let addition = voice_lfo.get_value(amount as f32);

        lfo_values.set_or_add(target_index, addition);
    }
}
