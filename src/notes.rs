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
        let duration_since_state_change = note_duration.0 - self.duration_at_state_change;

        let volume = match self.stage {
            EnvelopeStage::Attack => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);

                    self.last_volume
                }
                else if duration_since_state_change < operator_envelope.attack_duration.value {
                    calculate_envelope_volume(
                        0.0,
                        operator_envelope.attack_end_value.value,
                        duration_since_state_change,
                        operator_envelope.attack_duration.value,
                    )
                }
                else {
                    self.change_stage(EnvelopeStage::Decay, note_duration);

                    operator_envelope.attack_end_value.value
                }
            },
            EnvelopeStage::Decay => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);

                    self.last_volume
                }
                else if duration_since_state_change < operator_envelope.decay_duration.value {
                    calculate_envelope_volume(
                        self.pre_state_change_volume,
                        operator_envelope.decay_end_value.value,
                        duration_since_state_change,
                        operator_envelope.decay_duration.value,
                    )
                }
                else {
                    self.change_stage(EnvelopeStage::Sustain, note_duration);

                    operator_envelope.decay_end_value.value
                }
            },
            EnvelopeStage::Sustain => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);
                }

                operator_envelope.decay_end_value.value
            },
            EnvelopeStage::Release => {
                if duration_since_state_change < operator_envelope.release_duration.value {
                    calculate_envelope_volume(
                        self.pre_state_change_volume,
                        0.0,
                        duration_since_state_change,
                        operator_envelope.release_duration.value,
                    )
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
        self.active = true;
        self.duration = NoteDuration(0.0);
        self.duration_at_key_release = None;

        for operator in self.operators.iter_mut(){
            *operator = NoteOperator::default();
        }
    }

    pub fn release(&mut self){
        self.pressed = false;
        self.duration_at_key_release = Some(self.duration);
    }

    pub fn deactivate_if_finished(&mut self) {
        // When CPU load gets very high, envelopes seem not to be completed,
        // correctly, causing lots of fadeout noted to be left in the list
        // still set to active although they should be silent. I try to check
        // for that here.
        let left_behind = if let Some(d) = self.duration_at_key_release {
            self.duration.0 > d.0 + OPERATOR_ENVELOPE_MAX_DURATION + 1.0
        }
        else {
            false
        };

        let envelope_finished = self.operators.iter().all(|note_operator|
            note_operator.volume_envelope.stage == EnvelopeStage::Ended
        );

        if left_behind || envelope_finished {
            self.active = false;
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
        calculate_curve(CurveType::Sqrt3, time_progress)
}


fn calculate_curve(curve: CurveType, v: f64) -> f64 {
    match curve {
        CurveType::Exp => (v.exp() - 1.0) / (E - 1.0),
        CurveType::Ln => (1.0 + v * (E - 1.0)).ln(),
        CurveType::Log2 => (1.0 + v * (2.0 - 1.0)).log2(),
        CurveType::Log10 => (1.0 + v * (10.0 - 1.0)).log10(),
        CurveType::Sqrt3 => v.powf(1.0/3.0),
        CurveType::Sqrt => v.sqrt(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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