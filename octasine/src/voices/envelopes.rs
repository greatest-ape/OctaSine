use crate::approximations::Log10Table;
use crate::common::*;
use crate::constants::*;
use crate::parameters::processing::ProcessingParameterOperatorEnvelope;

use super::VoiceDuration;


#[derive(Debug, Copy, Clone)]
pub struct VoiceOperatorVolumeEnvelope {
    stage: EnvelopeStage,
    duration_at_stage_change: VoiceDuration,
    volume_at_stage_change: f64,
    last_volume: f64,
}


impl VoiceOperatorVolumeEnvelope {
    #[inline]
    fn advance_if_key_not_pressed(
        &mut self,
        key_pressed: bool,
        voice_duration: VoiceDuration
    ){
        use EnvelopeStage::*;

        if !key_pressed {
            match self.stage {
                Attack | Decay | Sustain => {
                    self.stage = Release;
                    self.duration_at_stage_change = voice_duration;
                    self.volume_at_stage_change = self.last_volume;
                },
                _ => ()
            }
        }
    }

    #[inline]
    fn advance_if_stage_time_up(
        &mut self,
        operator_envelope: &ProcessingParameterOperatorEnvelope,
        voice_duration: VoiceDuration,
    ) {
        use EnvelopeStage::*;

        let opt_stage_duration = match self.stage {
            Attack => Some(operator_envelope.attack_duration.value),
            Decay => Some(operator_envelope.decay_duration.value),
            Release => Some(operator_envelope.release_duration.value),
            _ => None
        };

        if let Some(stage_duration) = opt_stage_duration {
            let duration_since_stage_change = voice_duration.0 -
                self.duration_at_stage_change.0;

            if duration_since_stage_change >= stage_duration {
                if self.stage == Attack {
                    self.stage = Decay;
                    self.duration_at_stage_change = voice_duration;
                    self.volume_at_stage_change = self.last_volume;
                } else if self.stage == Decay {
                    self.stage = Sustain;
                    self.duration_at_stage_change = voice_duration;
                    self.volume_at_stage_change = self.last_volume;
                } else if self.stage == Release {
                    self.stage = Ended;
                    self.duration_at_stage_change = VoiceDuration(0.0);
                    self.volume_at_stage_change = 0.0;
                }
            }
        }
    }

    #[inline]
    fn calculate_volume(
        &self,
        log10_table: &Log10Table,
        operator_envelope: &ProcessingParameterOperatorEnvelope,
        voice_duration: VoiceDuration,
    ) -> f64 {
        use EnvelopeStage::*;

        let duration_since_stage_change = voice_duration.0 -
            self.duration_at_stage_change.0;

        match self.stage {
            Attack => {
                Self::calculate_curve(
                    log10_table,
                    self.volume_at_stage_change,
                    operator_envelope.attack_end_value.value,
                    duration_since_stage_change,
                    operator_envelope.attack_duration.value,
                )
            },
            Decay => {
                Self::calculate_curve(
                    log10_table,
                    self.volume_at_stage_change,
                    operator_envelope.decay_end_value.value,
                    duration_since_stage_change,
                    operator_envelope.decay_duration.value,
                )
            },
            Sustain => {
                operator_envelope.decay_end_value.value
            },
            Release => {
                Self::calculate_curve(
                    log10_table,
                    self.volume_at_stage_change,
                    0.0,
                    duration_since_stage_change,
                    operator_envelope.release_duration.value,
                )
            },
            Ended => {
                0.0
            }
        }
    }

    #[inline]
    pub fn calculate_curve(
        log10_table: &Log10Table,
        start_volume: f64,
        end_volume: f64,
        time_so_far_this_stage: f64,
        stage_length: f64,
    ) -> f64 {
        let time_progress = time_so_far_this_stage / stage_length;

        let curve_factor = (stage_length * ENVELOPE_CURVE_TAKEOVER_RECIP).min(1.0);
        let linear_factor = 1.0 - curve_factor;

        let curve = curve_factor * log10_table.calculate(time_progress);
        let linear = linear_factor * time_progress;

        start_volume + (end_volume - start_volume) * (curve + linear)
    }

    #[inline]
    /// Calculate volume and possibly advance envelope stage
    pub fn get_volume(
        &mut self,
        log10_table: &Log10Table,
        operator_envelope: &ProcessingParameterOperatorEnvelope,
        key_pressed: bool,
        voice_duration: VoiceDuration,
    ) -> f64 {
        if self.stage == EnvelopeStage::Ended {
            0.0
        } else {
            self.advance_if_key_not_pressed(key_pressed, voice_duration);
            self.advance_if_stage_time_up(operator_envelope, voice_duration);

            self.last_volume = self.calculate_volume(
                log10_table, operator_envelope, voice_duration);
            
            self.last_volume
        }
    }

    #[inline]
    pub fn restart(&mut self){
        self.stage = EnvelopeStage::Attack;
        self.volume_at_stage_change = self.last_volume;
        self.duration_at_stage_change = VoiceDuration(0.0);
    }

    #[inline]
    pub fn is_ended(&self) -> bool {
        self.stage == EnvelopeStage::Ended
    }
}


impl Default for VoiceOperatorVolumeEnvelope {
    fn default() -> Self {
        Self {
            stage: EnvelopeStage::Attack,
            duration_at_stage_change: VoiceDuration(0.0),
            volume_at_stage_change: 0.0,
            last_volume: 0.0
        }
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    fn valid_volume(volume: f64) -> bool {
        volume >= 0.0 && volume <= 1.0
    }

    #[test]
    fn calculate_curve_volume_output_in_range(){
        fn prop(values: (f64, f64, f64, f64)) -> TestResult {
            let start_volume = values.0;
            let end_volume = values.1;
            let time_so_far_this_stage = values.2;
            let stage_length = values.3;

            if values.0.is_nan() || values.1.is_nan() || values.2.is_nan() || values.3.is_nan(){
                return TestResult::discard();
            }

            if !valid_volume(start_volume) || !valid_volume(end_volume) {
                return TestResult::discard();
            }

            if stage_length < 0.0 || time_so_far_this_stage < 0.0 {
                return TestResult::discard();
            }

            if stage_length > ENVELOPE_MAX_DURATION {
                return TestResult::discard();
            }

            if time_so_far_this_stage > stage_length {
                return TestResult::discard();
            }

            let volume = VoiceOperatorVolumeEnvelope::calculate_curve(
                &Log10Table::default(),
                start_volume,
                end_volume,
                time_so_far_this_stage,
                stage_length
            );

            let success = valid_volume(volume);

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn((f64, f64, f64, f64)) -> TestResult);
    }

    #[test]
    fn calculate_curve_volume_start_end(){
        let table = Log10Table::default();

        assert_approx_eq!(
            VoiceOperatorVolumeEnvelope::calculate_curve(
                &table, 0.0, 1.0, 0.0, 4.0),
            0.0
        );
        assert_approx_eq!(
            VoiceOperatorVolumeEnvelope::calculate_curve(
                &table, 0.0, 1.0, 4.0, 4.0),
            1.0
        );
    }

    #[test]
    fn calculate_curve_volume_stage_change_continuity(){
        fn prop(stage_change_volume: f64) -> TestResult {
            if !valid_volume(stage_change_volume) {
                return TestResult::discard();
            }

            let stage_1_end = VoiceOperatorVolumeEnvelope::calculate_curve(
                &Log10Table::default(),
                0.0, stage_change_volume, 4.0, 4.0);

            let stage_2_start = VoiceOperatorVolumeEnvelope::calculate_curve(
                &Log10Table::default(),
                stage_change_volume, 1.0, 0.0, 4.0);
            
            let diff = (stage_1_end - stage_2_start).abs();

            let success = diff < 0.000001;

            if !success {
                println!("diff: {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}