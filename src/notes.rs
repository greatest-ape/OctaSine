use std::f64::consts::E;

use crate::common::*;
use crate::constants::*;
use crate::operators::*;
use crate::parameters::MasterFrequency;


pub enum CurveType {
    Exp,
    Ln,
    Log2,
    Log10,
    Sqrt3,
    Sqrt,
    Linear
}


#[derive(Debug, Copy, Clone)]
pub struct NoteDuration(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct NoteVelocity(pub f64);

impl NoteVelocity {
    pub fn from_midi_velocity(midi_velocity: u8) -> Self {
        if midi_velocity == 0 {
            Self::default()
        }
        else {
            Self(midi_velocity as f64 / 127.0)
        }
    }
}

impl Default for NoteVelocity {
    fn default() -> Self {
        Self(100.0 / 127.0)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch {
    frequency_factor: f64,
}

impl MidiPitch {
    pub fn new(midi_pitch: u8) -> Self {
        Self {
            frequency_factor: Self::calculate_frequency_factor(midi_pitch),
        }
    }

    fn calculate_frequency_factor(midi_pitch: u8) -> f64 {
        let note_diff = (midi_pitch as i8 - 69) as f64;

        (note_diff / 12.0).exp2()
    }

    pub fn get_frequency(&self, master_frequency: MasterFrequency) -> f64 {
        self.frequency_factor * master_frequency.value
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NoteOperatorVolumeEnvelope {
    stage: EnvelopeStage,
    duration_at_stage_change: NoteDuration,
    volume_at_stage_change: f64,
    last_volume: f64,
}

impl NoteOperatorVolumeEnvelope {
    fn advance_if_note_not_pressed(
        &mut self,
        note_pressed: bool,
        note_duration: NoteDuration
    ){
        use EnvelopeStage::*;

        if !note_pressed {
            match self.stage {
                Attack | Decay | Sustain => {
                    self.stage = Release;
                    self.duration_at_stage_change = note_duration;
                    self.volume_at_stage_change = self.last_volume;
                },
                _ => ()
            }
        }
    }

    fn advance_if_stage_time_up(
        &mut self,
        operator_envelope: &OperatorVolumeEnvelope,
        note_duration: NoteDuration,
    ) {
        use EnvelopeStage::*;

        let opt_stage_duration = match self.stage {
            Attack => Some(operator_envelope.attack_duration.value),
            Decay => Some(operator_envelope.decay_duration.value),
            Release => Some(operator_envelope.release_duration.value),
            _ => None
        };

        if let Some(stage_duration) = opt_stage_duration {
            let duration_since_stage_change = note_duration.0 -
                self.duration_at_stage_change.0;

            if duration_since_stage_change >= stage_duration {
                if self.stage == Attack {
                    self.stage = Decay;
                    self.duration_at_stage_change = note_duration;
                    self.volume_at_stage_change = self.last_volume;
                } else if self.stage == Decay {
                    self.stage = Sustain;
                    self.duration_at_stage_change = note_duration;
                    self.volume_at_stage_change = self.last_volume;
                } else if self.stage == Release {
                    self.stage = Ended;
                    self.duration_at_stage_change = NoteDuration(0.0);
                    self.volume_at_stage_change = 0.0;
                }
            }
        }
    }

    fn calculate_stage_volume(
        &self,
        operator_envelope: &OperatorVolumeEnvelope,
        note_duration: NoteDuration,
    ) -> f64 {
        use EnvelopeStage::*;

        let duration_since_stage_change = note_duration.0 -
            self.duration_at_stage_change.0;

        match self.stage {
            Attack => {
                calculate_envelope_volume(
                    self.volume_at_stage_change,
                    operator_envelope.attack_end_value.value,
                    duration_since_stage_change,
                    operator_envelope.attack_duration.value,
                )
            },
            Decay => {
                calculate_envelope_volume(
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
                calculate_envelope_volume(
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

    /// Calculate volume and possibly advance envelope stage
    pub fn get_volume(
        &mut self,
        operator_envelope: &OperatorVolumeEnvelope,
        note_pressed: bool,
        note_duration: NoteDuration,
    ) -> f64 {
        self.advance_if_note_not_pressed(note_pressed, note_duration);
        self.advance_if_stage_time_up(operator_envelope, note_duration);

        self.last_volume = self.calculate_stage_volume(
            operator_envelope, note_duration);
        
        self.last_volume
    }

    pub fn restart(&mut self){
        self.stage = EnvelopeStage::Attack;
        self.volume_at_stage_change = self.last_volume;
        self.duration_at_stage_change = NoteDuration(0.0);
    }
}

impl Default for NoteOperatorVolumeEnvelope {
    fn default() -> Self {
        Self {
            stage: EnvelopeStage::Attack,
            duration_at_stage_change: NoteDuration(0.0),
            volume_at_stage_change: 0.0,
            last_volume: 0.0
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NoteOperator {
    pub last_phase: Phase,
    pub volume_envelope: NoteOperatorVolumeEnvelope,
}

impl Default for NoteOperator {
    fn default() -> Self {
        Self {
            last_phase: Phase(0.0),
            volume_envelope: NoteOperatorVolumeEnvelope::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Note {
    pub pressed: bool,
    pub active: bool,
    pub duration: NoteDuration,
    pub duration_at_key_release: Option<NoteDuration>,
    pub velocity: NoteVelocity,
    pub midi_pitch: MidiPitch,
    pub operators: [NoteOperator; NUM_OPERATORS],
}

impl Note {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let operators = [NoteOperator::default(); NUM_OPERATORS];

        Self {
            pressed: false,
            active: false,
            velocity: NoteVelocity::default(),
            midi_pitch: midi_pitch,
            duration: NoteDuration(0.0),
            duration_at_key_release: None,
            operators: operators,
        }
    }

    pub fn press(&mut self, velocity: u8){
        self.velocity = NoteVelocity::from_midi_velocity(velocity);
        self.pressed = true;
        self.duration = NoteDuration(0.0);
        self.duration_at_key_release = None;

        if self.active {
            for operator in self.operators.iter_mut(){
                operator.volume_envelope.restart();
            }
        } else {
            for operator in self.operators.iter_mut(){
                *operator = NoteOperator::default();
            }

            self.active = true;
        }
    }

    pub fn release(&mut self){
        self.pressed = false;
        self.duration_at_key_release = Some(self.duration);
    }

    pub fn deactivate_if_envelopes_ended(&mut self) {
        let all_envelopes_ended = self.operators.iter().all(|note_operator|
            note_operator.volume_envelope.stage == EnvelopeStage::Ended
        );

        if all_envelopes_ended {
            self.active = false;
        }
    }

    // When CPU load gets very high, envelopes seem not to be completed,
    // correctly, causing lots of fadeout noted to be left in the list
    // still set to active although they should be silent. I try to check
    // for that here, even though this is most likely a bug in the
    // envelope implementation
    pub fn deactivate_extra_check(&mut self){
        if !self.active {
            return;
        }
        if let Some(d) = self.duration_at_key_release {
            if self.duration.0 > d.0 + OPERATOR_ENVELOPE_MAX_DURATION + 1.0 {
                self.active = false;
            }
        }
    }
}


fn calculate_envelope_volume(
    start_volume: f64,
    end_volume: f64,
    time_so_far_this_stage: f64,
    stage_length: f64,
) -> f64 {
    let time_progress = time_so_far_this_stage / stage_length;

    start_volume + (end_volume - start_volume) *
        calculate_curve(CurveType::Linear, time_progress)
}


fn calculate_curve(curve: CurveType, v: f64) -> f64 {
    match curve {
        CurveType::Exp => (v.exp() - 1.0) / (E - 1.0),
        CurveType::Ln => (1.0 + v * (E - 1.0)).ln(),
        CurveType::Log2 => (1.0 + v * (2.0 - 1.0)).log2(),
        CurveType::Log10 => (1.0 + v * (10.0 - 1.0)).log10(),
        CurveType::Sqrt3 => v.powf(1.0/3.0),
        CurveType::Sqrt => v.sqrt(),
        CurveType::Linear => v,
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    fn valid_volume(volume: f64) -> bool {
        volume >= 0.0 && volume <= 1.0
    }

    #[test]
    fn test_calculate_envelope_volume_output_in_range(){
        fn prop(values: (f64, f64, f64, f64)) -> TestResult {
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

            if stage_length > OPERATOR_ENVELOPE_MAX_DURATION {
                return TestResult::discard();
            }

            if time_so_far_this_stage > stage_length {
                return TestResult::discard();
            }

            let volume = calculate_envelope_volume(
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
    fn test_calculate_envelope_volume_start_end(){
        assert_eq!(calculate_envelope_volume(0.0, 1.0, 0.0, 4.0), 0.0);
        assert_eq!(calculate_envelope_volume(0.0, 1.0, 4.0, 4.0), 1.0);
    }

    #[test]
    fn test_calculate_envelope_volume_stage_change_continuity(){
        fn prop(stage_change_volume: f64) -> TestResult {
            if !valid_volume(stage_change_volume) {
                return TestResult::discard();
            }

            let stage_1_end = calculate_envelope_volume(
                0.0, stage_change_volume, 4.0, 4.0);

            let stage_2_start = calculate_envelope_volume(
                stage_change_volume, 1.0, 0.0, 4.0);

            TestResult::from_bool(stage_1_end == stage_2_start)
        }

        quickcheck(prop as fn(f64) -> TestResult);
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
            start_volume: f64,
            end_volume: f64,
            filename: &str
        ){
            use plotlib::function::*;
            use plotlib::view::ContinuousView;
            use plotlib::page::Page;

            let length = 1.0;

            let f = Function::new(|x| {
                calculate_envelope_volume(
                    start_volume,
                    end_volume,
                    x,
                    length,
                )
            }, 0., length);

            let v = ContinuousView::new()
                .add(&f)
                .x_range(0.0, length * 4.0)
                .y_range(0.0, 1.0);
            
            Page::single(&v).save(&filename).unwrap();
        }

        plot_envelope_stage(0.0, 1.0, "attack.svg");
        plot_envelope_stage(0.5, 1.0, "decay.svg");
        plot_envelope_stage(1.0, 0.0, "release.svg");
    }
}