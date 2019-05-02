use smallvec::SmallVec;

use crate::common::*;
use crate::constants::*;
use crate::operators::*;


#[derive(Debug, Copy, Clone)]
pub struct NoteDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch(pub u8);

impl MidiPitch {
    pub fn get_frequency(&self, master_frequency: MasterFrequency) -> f64 {
        let note_diff = (self.0 as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency.0
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NoteOperatorVolumeEnvelope {
    stage: EnvelopeStage,
    duration_at_state_change: f64,
    pre_state_change_volume: f64,
    last_volume: f64,
}

impl NoteOperatorVolumeEnvelope {

    /// Calculate volume and possibly advance envelope stage
    pub fn calculate_volume(
        &mut self,
        operator_envelope: &OperatorVolumeEnvelope,
        note_pressed: bool,
        note_duration: NoteDuration,
    ) -> f64 {
        let effective_duration = note_duration.0 - self.duration_at_state_change;

        let volume = match self.stage {
            EnvelopeStage::Attack => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);

                    self.last_volume
                }
                else if effective_duration < operator_envelope.attack_duration.0 {
                    (effective_duration / operator_envelope.attack_duration.0) * operator_envelope.attack_end_value.0
                }
                else {
                    self.change_stage(EnvelopeStage::Decay, note_duration);

                    operator_envelope.attack_end_value.0
                }
            },
            EnvelopeStage::Decay => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);

                    self.last_volume
                }
                else if effective_duration < operator_envelope.decay_duration.0 {
                    self.pre_state_change_volume + ((effective_duration / operator_envelope.decay_duration.0) *
                        (operator_envelope.decay_end_value.0 - self.pre_state_change_volume))
                }
                else {
                    self.change_stage(EnvelopeStage::Sustain, note_duration);

                    operator_envelope.decay_end_value.0
                }
            },
            EnvelopeStage::Sustain => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);
                }

                operator_envelope.decay_end_value.0
            },
            EnvelopeStage::Release => {
                if effective_duration < operator_envelope.release_duration.0 {
                    ((1.0 - (effective_duration / operator_envelope.release_duration.0)) * self.pre_state_change_volume)
                }
                else {
                    self.change_stage(EnvelopeStage::Ended, NoteDuration(0.0));

                    0.0
                }
            },
            EnvelopeStage::Ended => {
                0.0
            }
        };

        self.last_volume = volume;

        volume
    }

    pub fn change_stage(&mut self, new_stage: EnvelopeStage, note_duration: NoteDuration){
        self.stage = new_stage;
        self.duration_at_state_change = note_duration.0;
        self.pre_state_change_volume = self.last_volume;
    }
}

impl Default for NoteOperatorVolumeEnvelope {
    fn default() -> Self {
        Self {
            stage: EnvelopeStage::Attack,
            duration_at_state_change: 0.0,
            pre_state_change_volume: 0.0,
            last_volume: 0.0
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NoteOperator {
    pub volume_envelope: NoteOperatorVolumeEnvelope,
    pub last_phase: Phase,
}

impl Default for NoteOperator {
    fn default() -> Self {
        Self {
            volume_envelope: NoteOperatorVolumeEnvelope::default(),
            last_phase: Phase(0.0),
        }
    }
}


pub type NoteOperators = SmallVec<[NoteOperator; NUM_OPERATORS]>;


#[derive(Debug, Clone)]
pub struct Note {
    pub pressed: bool,
    pub active: bool,
    pub duration: NoteDuration,
    pub midi_pitch: MidiPitch,
    pub operators: NoteOperators,
}

impl Note {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let mut operators = SmallVec::new();

        for _ in 0..NUM_OPERATORS {
            operators.push(NoteOperator::default());
        }

        Self {
            pressed: false,
            active: false,
            midi_pitch: midi_pitch,
            duration: NoteDuration(0.0),
            operators: operators,
        }
    }

    pub fn press(&mut self){
        self.pressed = true;
        self.active = true;
        self.duration = NoteDuration(0.0);

        for operator in self.operators.iter_mut(){
            *operator = NoteOperator::default();
        }
    }

    pub fn release(&mut self){
        if self.active {
            self.pressed = false;
        }
    }

    pub fn deactivate_if_all_operators_finished(&mut self) {
        let finished = self.operators.iter().all(|note_operator|
            note_operator.volume_envelope.stage == EnvelopeStage::Ended
        );

        if finished {
            self.active = false;
        }
    }
}