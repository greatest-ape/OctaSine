pub mod envelopes;
pub mod lfos;
pub mod log10_table;

use array_init::array_init;

use crate::common::*;

use envelopes::*;
use lfos::*;

use super::{
    interpolation::{InterpolationDuration, Interpolator},
    parameters::AudioParameters,
};

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
    key: u8,
}

impl MidiPitch {
    pub fn new(midi_pitch: u8) -> Self {
        Self {
            frequency_factor: Self::calculate_frequency_factor(midi_pitch),
            key: midi_pitch,
        }
    }

    fn calculate_frequency_factor(midi_pitch: u8) -> f64 {
        let note_diff = f64::from(midi_pitch as i8 - 69);

        (note_diff / 12.0).exp2()
    }

    pub fn get_frequency(self, master_frequency: f64) -> f64 {
        self.frequency_factor * master_frequency
    }

    pub fn key(&self) -> u8 {
        self.key
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
    pub is_monophonic: bool,
    /// Has received at least one key press and has at least one envelope still running
    pub active: bool,
    pub midi_pitch: MidiPitch,
    pub key_pressed: bool,
    pub pitch_interpolator: Interpolator,
    key_velocity_interpolator: Interpolator,
    pub operators: [VoiceOperator; NUM_OPERATORS],
    pub lfos: [VoiceLfo; NUM_LFOS],
    #[cfg(feature = "clap")]
    pub clap_note_id: Option<i32>,
}

impl Voice {
    pub fn new(midi_pitch: MidiPitch, is_monophonic: bool) -> Self {
        let operators = [VoiceOperator::default(); NUM_OPERATORS];

        Self {
            is_monophonic,
            active: false,
            midi_pitch,
            key_pressed: false,
            pitch_interpolator: Interpolator::new(
                midi_pitch.frequency_factor as f32,
                InterpolationDuration::exactly_1s(),
            ),
            key_velocity_interpolator: Interpolator::new(
                KeyVelocity::default().0,
                VELOCITY_INTERPOLATION_DURATION,
            ),
            operators,
            lfos: array_init(|_| VoiceLfo::default()),
            #[cfg(feature = "clap")]
            clap_note_id: None,
        }
    }

    pub fn advance_interpolators_one_sample(&mut self, sample_rate: SampleRate) {
        self.key_velocity_interpolator
            .advance_one_sample(sample_rate, &mut |_| ());
        self.pitch_interpolator
            .advance_one_sample(sample_rate, &mut |_| ());
    }

    pub fn get_key_velocity(&mut self) -> KeyVelocity {
        KeyVelocity(self.key_velocity_interpolator.get_value())
    }

    #[inline]
    pub fn press_key(
        &mut self,
        parameters: &AudioParameters,
        velocity: KeyVelocity,
        initial_key: Option<u8>,
        target_key: Option<(u8, f64)>,
        #[cfg_attr(not(feature = "clap"), allow(unused_variables))] opt_clap_note_id: Option<i32>,
    ) {
        if self.active {
            self.key_velocity_interpolator.set_value(velocity.0)
        } else {
            self.key_velocity_interpolator.force_set_value(velocity.0)
        }

        if let Some(key) = initial_key {
            self.change_pitch(key, None);
        }

        if let Some((key, glide_time)) = target_key {
            self.change_pitch(key, Some(glide_time));
        }

        self.key_pressed = true;

        for operator in self.operators.iter_mut() {
            operator.volume_envelope.restart(self.is_monophonic);
        }

        for (lfo, parameters) in self.lfos.iter_mut().zip(parameters.lfos.iter()) {
            lfo.restart(parameters);
        }

        #[cfg(feature = "clap")]
        {
            self.clap_note_id = opt_clap_note_id;
        }

        self.active = true;
    }

    pub fn change_pitch(&mut self, key: u8, interpolate: Option<f64>) {
        self.midi_pitch = MidiPitch::new(key);

        if let Some(glide_time) = interpolate {
            self.pitch_interpolator
                .change_duration(InterpolationDuration(glide_time));

            self.pitch_interpolator
                .set_value(self.midi_pitch.frequency_factor as f32);
        } else {
            self.pitch_interpolator
                .force_set_value(self.midi_pitch.frequency_factor as f32);
        }
    }

    pub fn aftertouch(&mut self, velocity: KeyVelocity) {
        self.key_velocity_interpolator.set_value(velocity.0)
    }

    pub fn key(&self) -> u8 {
        self.midi_pitch.key
    }

    #[inline]
    pub fn release_key(&mut self) {
        self.key_pressed = false;
    }

    pub fn kill_envelopes(&mut self) {
        for operator in self.operators.iter_mut() {
            operator.volume_envelope.kill();
        }
    }

    #[inline]
    pub fn deactivate_if_envelopes_ended(&mut self) -> bool {
        let all_envelopes_ended = self
            .operators
            .iter()
            .all(|voice_operator| voice_operator.volume_envelope.is_ended());

        if all_envelopes_ended {
            for lfo in self.lfos.iter_mut() {
                lfo.envelope_ended();
            }

            for operator in self.operators.iter_mut() {
                operator.last_phase.0 = 0.0;
            }

            self.active = false;
        }

        all_envelopes_ended
    }
}
