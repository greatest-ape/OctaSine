use arrayvec::ArrayVec;

use crate::common::*;
use crate::constants::*;
use crate::voices::lfos::VoiceLfo;
use crate::parameters::processing::*;


#[derive(Default)]
pub struct LfoTargetValues(ArrayVec<(LfoTargetParameter, f64), NUM_LFOS>);


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

    pub fn get(&self, target: LfoTargetParameter) -> Option<f64> {
        for (t, v) in self.0.iter() {
            if *t == target {
                return Some(*v)
            }
        }

        None
    }
}


pub fn get_lfo_target_values(
    lfo_parameters: &mut [ProcessingParameterLfo; NUM_LFOS],
    voice_lfos: &mut [VoiceLfo; NUM_LFOS],
    time: TimeCounter,
    time_advancement: f64,
    bpm: BeatsPerMinute,
) -> LfoTargetValues {
    let mut lfo_values = LfoTargetValues::default();

    for (lfo_index, (voice_lfo, lfo_parameter)) in voice_lfos.iter_mut()
        .zip(lfo_parameters.iter_mut()).enumerate().rev()
    {
        let amount = lfo_parameter.amount.get_value_with_lfo_addition(
            time,
            lfo_values.get(LfoTargetParameter::Lfo(lfo_index, LfoTargetLfoParameter::Amount))
        );

        if amount.abs() < ZERO_VALUE_LIMIT {
            continue;
        }

        let mode = lfo_parameter.mode.value;
        let bpm_sync = lfo_parameter.bpm_sync.value;

        let shape = lfo_parameter.shape.get_value_with_lfo_addition(
            (),
            lfo_values.get(LfoTargetParameter::Lfo(lfo_index, LfoTargetLfoParameter::Shape))
        );
        let frequency_ratio = lfo_parameter.frequency_ratio.get_value_with_lfo_addition(
            (),
            lfo_values.get(LfoTargetParameter::Lfo(lfo_index, LfoTargetLfoParameter::FrequencyRatio))
        );
        let frequency_free = lfo_parameter.frequency_free.get_value_with_lfo_addition(
            (),
            lfo_values.get(LfoTargetParameter::Lfo(lfo_index, LfoTargetLfoParameter::FrequencyFree))
        );

        let bpm = if bpm_sync {
            bpm
        } else {
            BeatsPerMinute::default()
        };

        let addition = voice_lfo.get_value(
            time_advancement,
            bpm,
            shape,
            mode,
            frequency_ratio * frequency_free,
            amount,
        );

        let target = lfo_parameter.target_parameter.get_value();

        lfo_values.set_or_add(target, addition);
    }

    lfo_values
}