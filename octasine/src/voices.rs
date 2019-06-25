use std::f32::consts::E;

use crate::approximations::EnvelopeCurveTable;
use crate::common::*;
use crate::constants::*;
use crate::processing_parameters::*;


pub enum CurveType {
    Exp,
    Ln,
    Log2,
    Log10,
    Sqrt,
    Cbrt,
    Sqrt4,
    Linear
}


#[derive(Debug, Copy, Clone)]
pub struct VoiceDuration(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct KeyVelocity(pub f32);

impl KeyVelocity {
    pub fn from_midi_velocity(midi_velocity: u8) -> Self {
        if midi_velocity == 0 {
            Self::default()
        }
        else {
            Self(midi_velocity as f32 / 127.0)
        }
    }
}

impl Default for KeyVelocity {
    fn default() -> Self {
        Self(100.0 / 127.0)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch {
    frequency_factor: f32,
}

impl MidiPitch {
    pub fn new(midi_pitch: u8) -> Self {
        Self {
            frequency_factor: Self::calculate_frequency_factor(midi_pitch),
        }
    }

    fn calculate_frequency_factor(midi_pitch: u8) -> f32 {
        let note_diff = (midi_pitch as i8 - 69) as f32;

        (note_diff / 12.0).exp2()
    }

    pub fn get_frequency(&self, master_frequency: f32) -> f32 {
        self.frequency_factor * master_frequency
    }
}


#[derive(Debug, Copy, Clone)]
pub struct VoiceOperatorVolumeEnvelope {
    stage: EnvelopeStage,
    duration_at_stage_change: VoiceDuration,
    volume_at_stage_change: f32,
    last_volume: f32,
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
        envelope_curve_table: &EnvelopeCurveTable,
        operator_envelope: &ProcessingParameterOperatorEnvelope,
        voice_duration: VoiceDuration,
    ) -> f32 {
        use EnvelopeStage::*;

        let duration_since_stage_change = voice_duration.0 -
            self.duration_at_stage_change.0;

        match self.stage {
            Attack => {
                Self::calculate_curve(
                    envelope_curve_table,
                    self.volume_at_stage_change,
                    operator_envelope.attack_end_value.value,
                    duration_since_stage_change,
                    operator_envelope.attack_duration.value,
                )
            },
            Decay => {
                Self::calculate_curve(
                    envelope_curve_table,
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
                    envelope_curve_table,
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
        envelope_curve_table: &EnvelopeCurveTable,
        start_volume: f32,
        end_volume: f32,
        time_so_far_this_stage: f32,
        stage_length: f32,
    ) -> f32 {
        let time_progress = time_so_far_this_stage / stage_length;

        let curve_factor = (stage_length * ENVELOPE_CURVE_TAKEOVER_RECIP).min(1.0);
        let linear_factor = 1.0 - curve_factor;

        let curve = curve_factor * envelope_curve_table.calculate(time_progress);
        let linear = linear_factor * time_progress;

        start_volume + (end_volume - start_volume) * (curve + linear)
    }

    #[inline]
    /// Calculate volume and possibly advance envelope stage
    pub fn get_volume(
        &mut self,
        envelope_curve_table: &EnvelopeCurveTable,
        operator_envelope: &ProcessingParameterOperatorEnvelope,
        key_pressed: bool,
        voice_duration: VoiceDuration,
    ) -> f32 {
        if self.stage == EnvelopeStage::Ended {
            return 0.0
        } else {
            self.advance_if_key_not_pressed(key_pressed, voice_duration);
            self.advance_if_stage_time_up(operator_envelope, voice_duration);

            self.last_volume = self.calculate_volume(
                envelope_curve_table, operator_envelope, voice_duration);
            
            self.last_volume
        }
    }

    #[inline]
    pub fn restart(&mut self){
        self.stage = EnvelopeStage::Attack;
        self.volume_at_stage_change = self.last_volume;
        self.duration_at_stage_change = VoiceDuration(0.0);
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


#[derive(Debug, Copy, Clone)]
pub struct VoiceOperator {
    pub last_phase: Phase,
    pub volume_envelope: VoiceOperatorVolumeEnvelope,
}

impl Default for VoiceOperator {
    fn default() -> Self {
        Self {
            last_phase: Phase(0.0),
            volume_envelope: VoiceOperatorVolumeEnvelope::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Voice {
    pub active: bool,
    pub midi_pitch: MidiPitch,
    pub duration: VoiceDuration,
    pub key_pressed: bool,
    pub key_velocity: KeyVelocity,
    pub operators: [VoiceOperator; NUM_OPERATORS],
}

impl Voice {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let operators = [VoiceOperator::default(); NUM_OPERATORS];

        Self {
            active: false,
            midi_pitch: midi_pitch,
            duration: VoiceDuration(0.0),
            key_pressed: false,
            key_velocity: KeyVelocity::default(),
            operators: operators,
        }
    }

    #[inline]
    pub fn press_key(&mut self, velocity: u8){
        self.key_velocity = KeyVelocity::from_midi_velocity(velocity);
        self.key_pressed = true;
        self.duration = VoiceDuration(0.0);

        for operator in self.operators.iter_mut(){
            operator.volume_envelope.restart();
        }

        self.active = true;
    }

    #[inline]
    pub fn release_key(&mut self){
        self.key_pressed = false;
    }

    #[inline]
    pub fn deactivate_if_envelopes_ended(&mut self) {
        let all_envelopes_ended = self.operators.iter().all(|voice_operator|
            voice_operator.volume_envelope.stage == EnvelopeStage::Ended
        );

        if all_envelopes_ended {
            self.active = false;
        }
    }
}



/// Kept here for reference
#[allow(dead_code)]
fn calculate_curve(curve: CurveType, v: f32) -> f32 {
    match curve {
        CurveType::Exp => (v.exp() - 1.0) / (E - 1.0),
        CurveType::Ln => (1.0 + v * (E - 1.0)).ln(),
        CurveType::Log2 => (1.0 + v * (2.0 - 1.0)).log2(),
        CurveType::Log10 => (1.0 + v * (10.0 - 1.0)).log10(),
        CurveType::Sqrt => v.sqrt(),
        CurveType::Cbrt => v.cbrt(),
        CurveType::Sqrt4 => v.sqrt().sqrt(),
        CurveType::Linear => v,
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    fn valid_volume(volume: f32) -> bool {
        volume >= 0.0 && volume <= 1.0
    }

    #[test]
    fn calculate_curveolume_output_in_range(){
        fn prop(values: (f32, f32, f32, f32)) -> TestResult {
            let start_volume = values.0;
            let end_volume = values.1;
            let time_so_far_this_stage = values.2;
            let stage_length = values.3;

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
                &EnvelopeCurveTable::new(),
                start_volume,
                end_volume,
                time_so_far_this_stage,
                stage_length
            );

            let success = valid_volume(volume);

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn((f32, f32, f32, f32)) -> TestResult);
    }

    #[test]
    fn calculate_curveolume_start_end(){
        let table = EnvelopeCurveTable::new();

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
    fn calculate_curveolume_stage_change_continuity(){
        fn prop(stage_change_volume: f32) -> TestResult {
            if !valid_volume(stage_change_volume) {
                return TestResult::discard();
            }

            let stage_1_end = VoiceOperatorVolumeEnvelope::calculate_curve(
                &EnvelopeCurveTable::new(),
                0.0, stage_change_volume, 4.0, 4.0);

            let stage_2_start = VoiceOperatorVolumeEnvelope::calculate_curve(
                &EnvelopeCurveTable::new(),
                stage_change_volume, 1.0, 0.0, 4.0);
            
            let diff = (stage_1_end - stage_2_start).abs();

            let success = diff < 0.000001;

            if !success {
                println!("diff: {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    /// Generate plots to check how envelopes look.
    /// 
    /// It is obviously not ideal to do this when running tests, but otherwise
    /// the structure of this crate would become a lot more complicated.
    /// 
    /// (Add #[test] before function to run when testing)
    #[allow(dead_code)]
    fn test_gen_plots(){
        fn plot_envelope_stage(
            start_volume: f32,
            end_volume: f32,
            filename: &str
        ){
            use plotlib::function::*;
            use plotlib::view::ContinuousView;
            use plotlib::page::Page;

            let length = 1.0;

            let f = Function::new(|x| {
                VoiceOperatorVolumeEnvelope::calculate_curve(
                    &EnvelopeCurveTable::new(),
                    start_volume,
                    end_volume,
                    x as f32,
                    length as f32,
                ).into()
            }, 0., length);

            let v = ContinuousView::new()
                .add(&f)
                .x_range(0.0, length)
                .y_range(0.0, 1.0);
            
            Page::single(&v).save(&filename).unwrap();
        }

        plot_envelope_stage(0.0, 1.0, "attack.svg");
        plot_envelope_stage(0.5, 1.0, "decay.svg");
        plot_envelope_stage(1.0, 0.0, "release.svg");
    }
}