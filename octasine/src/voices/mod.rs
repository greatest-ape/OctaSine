use array_init::array_init;

use crate::common::*;
use crate::constants::*;

pub mod envelopes;
pub mod lfos;

use envelopes::*;
use lfos::*;

#[derive(Debug, Copy, Clone)]
pub struct VoiceDuration(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct KeyVelocity(pub f64);

impl Default for KeyVelocity {
    fn default() -> Self {
        Self(100.0 / 127.0)
    }
}

impl KeyVelocity {
    pub fn from_midi_velocity(midi_velocity: u8) -> Self {
        if midi_velocity == 0 {
            Self::default()
        } else {
            Self(f64::from(midi_velocity) / 127.0)
        }
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
        let note_diff = f64::from(midi_pitch as i8 - 69);

        (note_diff / 12.0).exp2()
    }

    pub fn get_frequency(self, master_frequency: f64) -> f64 {
        self.frequency_factor * master_frequency
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
    pub key_pressed: bool,
    pub key_velocity: KeyVelocity,
    pub operators: [VoiceOperator; NUM_OPERATORS],
    pub lfos: [VoiceLfo; NUM_LFOS],
}

impl Voice {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let operators = [VoiceOperator::default(); NUM_OPERATORS];

        Self {
            active: false,
            midi_pitch,
            key_pressed: false,
            key_velocity: KeyVelocity::default(),
            operators,
            lfos: array_init(|_| VoiceLfo::default()),
        }
    }

    #[inline]
    pub fn press_key(&mut self, velocity: u8) {
        self.key_velocity = KeyVelocity::from_midi_velocity(velocity);
        self.key_pressed = true;

        for operator in self.operators.iter_mut() {
            operator.volume_envelope.restart();
        }

        for lfo in self.lfos.iter_mut() {
            lfo.restart();
        }

        self.active = true;
    }

    #[inline]
    pub fn release_key(&mut self) {
        self.key_pressed = false;
    }

    #[inline]
    pub fn deactivate_if_envelopes_ended(&mut self) {
        let all_envelopes_ended = self
            .operators
            .iter()
            .all(|voice_operator| voice_operator.volume_envelope.is_ended());

        if all_envelopes_ended {
            for lfo in self.lfos.iter_mut() {
                lfo.request_stop();
            }

            self.active = false;
        }
    }
}
