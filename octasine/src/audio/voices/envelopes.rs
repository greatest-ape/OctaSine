use crate::audio::parameters::common::AudioParameter;
use crate::audio::parameters::OperatorEnvelopeAudioParameters;
use crate::common::*;
use crate::parameters::ENVELOPE_CURVE_TAKEOVER_RECIP;

use super::log10_table::Log10Table;
use super::VoiceDuration;

const INTERPOLATION_DURATION: f64 = 0.01;

#[derive(Debug, Copy, Clone)]
pub struct VoiceOperatorVolumeEnvelope {
    stage: EnvelopeStage,
    duration: VoiceDuration,
    duration_at_stage_change: VoiceDuration,
    volume_at_stage_change: f32,
    last_volume: f32,
    restarting_from_volume: Option<f32>,
}

impl VoiceOperatorVolumeEnvelope {
    pub fn advance_one_sample(
        &mut self,
        parameters: &OperatorEnvelopeAudioParameters,
        key_pressed: bool,
        time_per_sample: TimePerSample,
    ) {
        use EnvelopeStage::*;

        if let Ended = self.stage {
            return;
        }

        self.duration.0 += time_per_sample.0;

        if self.restarting_from_volume.is_some() && self.duration.0 >= INTERPOLATION_DURATION {
            self.restarting_from_volume = None;
        }

        if !key_pressed {
            match self.stage {
                Attack | Decay | Sustain => {
                    self.stage = Release;
                    self.duration_at_stage_change = self.duration;
                    self.volume_at_stage_change = self.last_volume;

                    return;
                }
                Release | Ended => (),
            }
        }

        let duration_since_stage_change = self.duration_since_stage_change();

        match self.stage {
            Attack if duration_since_stage_change >= parameters.attack_duration.get_value() => {
                self.stage = Decay;
                self.duration_at_stage_change = self.duration;
                self.volume_at_stage_change = self.last_volume;
            }
            Decay if duration_since_stage_change >= parameters.decay_duration.get_value() => {
                self.stage = Sustain;
                self.duration_at_stage_change = self.duration;
                self.volume_at_stage_change = self.last_volume;
            }
            Release if duration_since_stage_change >= parameters.release_duration.get_value() => {
                self.stage = Ended;
                self.duration_at_stage_change = VoiceDuration(0.0);
                self.volume_at_stage_change = 0.0;
            }
            _ => {}
        }
    }

    pub fn get_volume(
        &mut self,
        log10table: &Log10Table,
        parameters: &OperatorEnvelopeAudioParameters,
    ) -> f32 {
        use EnvelopeStage::*;

        if let Ended = self.stage {
            self.last_volume = 0.0;

            return 0.0;
        }

        let volume = match self.stage {
            Attack => Self::calculate_curve(
                log10table,
                0.0,
                1.0,
                self.duration_since_stage_change(),
                parameters.attack_duration.get_value(),
            ),
            Decay => Self::calculate_curve(
                log10table,
                self.volume_at_stage_change,
                parameters.sustain_volume.get_value(),
                self.duration_since_stage_change(),
                parameters.decay_duration.get_value(),
            ),
            Sustain => parameters.sustain_volume.get_value(),
            Release => Self::calculate_curve(
                log10table,
                self.volume_at_stage_change,
                0.0,
                self.duration_since_stage_change(),
                parameters.release_duration.get_value(),
            ),
            Ended => unreachable!(),
        };

        self.last_volume = if let Some(restart_volume) = self.restarting_from_volume {
            let progress = (self.duration.0 / INTERPOLATION_DURATION) as f32;

            progress * volume + (1.0 - progress) * restart_volume
        } else {
            volume
        };

        self.last_volume
    }

    fn duration_since_stage_change(&self) -> f64 {
        self.duration.0 - self.duration_at_stage_change.0
    }

    pub fn calculate_curve(
        log10table: &Log10Table,
        start_volume: f32,
        end_volume: f32,
        time_so_far_this_stage: f64,
        stage_length: f64,
    ) -> f32 {
        let time_progress = (time_so_far_this_stage / stage_length) as f32;

        let curve_factor = (stage_length * ENVELOPE_CURVE_TAKEOVER_RECIP).min(1.0) as f32;
        let linear_factor = 1.0 - curve_factor;
        let curve = curve_factor * log10table.calculate(time_progress);
        let linear = linear_factor * time_progress;

        start_volume + (end_volume - start_volume) * (curve + linear)
    }

    #[inline]
    pub fn restart(&mut self) {
        *self = if let EnvelopeStage::Ended = self.stage {
            Self::default()
        } else {
            Self {
                restarting_from_volume: Some(self.last_volume),
                ..Default::default()
            }
        };
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
            duration: VoiceDuration(0.0),
            volume_at_stage_change: 0.0,
            last_volume: 0.0,
            restarting_from_volume: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use quickcheck::{quickcheck, TestResult};

    use crate::parameters::ENVELOPE_MAX_DURATION;

    use super::*;

    fn valid_volume(volume: f32) -> bool {
        volume >= 0.0 && volume <= 1.0
    }

    #[test]
    fn calculate_curve_volume_output_in_range() {
        fn prop(values: (f32, f32, f64, f64)) -> TestResult {
            let start_volume = values.0;
            let end_volume = values.1;
            let time_so_far_this_stage = values.2;
            let stage_length = values.3;

            if values.0.is_nan() || values.1.is_nan() || values.2.is_nan() || values.3.is_nan() {
                return TestResult::discard();
            }
            if values.0.is_sign_negative()
                || values.1.is_sign_negative()
                || values.2.is_sign_negative()
                || values.3.is_sign_negative()
            {
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
                stage_length,
            );

            let success = valid_volume(volume);

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn((f32, f32, f64, f64)) -> TestResult);
    }

    #[test]
    fn calculate_curve_volume_start_end() {
        let table = Log10Table::default();

        assert_approx_eq!(
            VoiceOperatorVolumeEnvelope::calculate_curve(&table, 0.0, 1.0, 0.0, 4.0),
            0.0
        );
        assert_approx_eq!(
            VoiceOperatorVolumeEnvelope::calculate_curve(&table, 0.0, 1.0, 4.0, 4.0),
            1.0
        );
    }

    #[test]
    fn calculate_curve_volume_stage_change_continuity() {
        fn prop(stage_change_volume: f32) -> TestResult {
            if !valid_volume(stage_change_volume) {
                return TestResult::discard();
            }

            let table = Log10Table::default();

            let stage_1_end = VoiceOperatorVolumeEnvelope::calculate_curve(
                &table,
                0.0,
                stage_change_volume,
                4.0,
                4.0,
            );

            let stage_2_start = VoiceOperatorVolumeEnvelope::calculate_curve(
                &table,
                stage_change_volume,
                1.0,
                0.0,
                4.0,
            );

            let diff = (stage_1_end - stage_2_start).abs();

            let success = diff < 0.000001;

            if !success {
                println!("diff: {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }
}
