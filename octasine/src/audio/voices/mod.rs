pub mod envelopes;
pub mod lfos;
pub mod log10_table;

use array_init::array_init;

use crate::common::*;

use envelopes::*;
use lfos::*;

use super::common::{InterpolationDuration, Interpolator};

const VELOCITY_INTERPOLATION_DURATION: InterpolationDuration =
    InterpolationDuration::exactly_10ms();

#[derive(Debug, Copy, Clone)]
pub struct VoiceDuration(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct KeyVelocity(pub f32);

impl Default for KeyVelocity {
    fn default() -> Self {
        Self(100.0 / 127.0)
    }
}

impl KeyVelocity {
    pub fn from_midi_velocity(midi_velocity: u8) -> Self {
        Self(f32::from(midi_velocity) / 127.0)
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
    key_velocity_interpolator: Interpolator,
    pub operators: [VoiceOperator; NUM_OPERATORS],
    pub lfos: [VoiceLfo; NUM_LFOS],
    pub clap_note_data: Option<(i32, u32)>,
}

impl Voice {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let operators = [VoiceOperator::default(); NUM_OPERATORS];

        Self {
            active: false,
            midi_pitch,
            key_pressed: false,
            key_velocity_interpolator: Interpolator::new(
                KeyVelocity::default().0,
                VELOCITY_INTERPOLATION_DURATION,
            ),
            operators,
            lfos: array_init(|_| VoiceLfo::default()),
            clap_note_data: None,
        }
    }

    pub fn advance_velocity_interpolator_one_sample(&mut self, sample_rate: SampleRate) {
        self.key_velocity_interpolator
            .advance_one_sample(sample_rate, &mut |_| ())
    }

    pub fn get_key_velocity(&mut self) -> KeyVelocity {
        KeyVelocity(self.key_velocity_interpolator.get_value())
    }

    #[inline]
    pub fn press_key(&mut self, velocity: KeyVelocity, opt_clap_note_id: Option<i32>) {
        if self.active {
            self.key_velocity_interpolator.set_value(velocity.0)
        } else {
            self.key_velocity_interpolator.force_set_value(velocity.0)
        }

        self.key_pressed = true;

        for operator in self.operators.iter_mut() {
            operator.volume_envelope.restart();
        }

        for lfo in self.lfos.iter_mut() {
            lfo.restart();
        }

        if let Some(clap_note_id) = opt_clap_note_id {
            self.clap_note_data = Some((clap_note_id, 0));
        }

        self.active = true;
    }

    #[inline]
    pub fn release_key(&mut self) {
        self.key_pressed = false;
    }

    #[inline]
    pub fn deactivate_if_envelopes_ended(&mut self, sample_index: usize) {
        let all_envelopes_ended = self
            .operators
            .iter()
            .all(|voice_operator| voice_operator.volume_envelope.is_ended());

        if all_envelopes_ended {
            for lfo in self.lfos.iter_mut() {
                lfo.envelope_ended();
            }

            if let Some((_, ended)) = self.clap_note_data.as_mut() {
                *ended = sample_index as u32;
            }

            self.active = false;
        }
    }
}
