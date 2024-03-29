pub mod gen;
mod interpolation;
pub mod parameters;
pub mod voices;

use std::mem::MaybeUninit;

use fastrand::Rng;
use ringbuf::{LocalRb, Rb};

use crate::{
    common::*,
    parameters::{
        glide_active::GlideActive, glide_mode::GlideMode, voice_mode::VoiceMode, Parameter,
    },
};

use parameters::*;
use voices::*;

use self::{
    gen::AudioGenData, parameters::common::AudioParameter, voices::log10_table::Log10Table,
};

#[cfg(feature = "clap")]
#[derive(Debug)]
pub struct ClapNoteEnded {
    pub key: u8,
    pub clap_note_id: i32,
    pub sample_index: u32,
}

#[cfg(feature = "clap")]
pub type ClapEndedNotesRb =
    ringbuf::LocalRb<ClapNoteEnded, Vec<::std::mem::MaybeUninit<ClapNoteEnded>>>;

pub struct AudioState {
    sample_rate: SampleRate,
    time_per_sample: TimePerSample,
    bpm: BeatsPerMinute,
    bpm_lfo_multiplier: BpmLfoMultiplier,
    pub global_pitch_bend: GlobalPitchBend,
    sustain_pedal_on: bool,
    parameters: AudioParameters,
    rng: Rng,
    log10table: Log10Table,
    pub polyphonic_voices: IndexMap<u8, Voice>,
    pub monophonic_voice: Voice,
    monophonic_pressed_keys: IndexMap<u8, Option<i32>>,
    pending_note_events: LocalRb<NoteEvent, Vec<MaybeUninit<NoteEvent>>>,
    opt_last_voice_mode: Option<VoiceMode>,
    audio_gen_data_w2: Box<AudioGenData<2>>,
    #[cfg(target_arch = "x86_64")]
    audio_gen_data_w4: Box<AudioGenData<4>>,
    #[cfg(feature = "clap")]
    pub clap_ended_notes: ClapEndedNotesRb,
}

impl Default for AudioState {
    fn default() -> Self {
        let polyphonic_voices = {
            let mut voices = IndexMap::default();

            voices.reserve(128);

            voices
        };
        let monophonic_pressed_keys = {
            let mut pressed_keys = IndexMap::default();

            pressed_keys.reserve(128);

            pressed_keys
        };

        Self {
            sample_rate: SampleRate::default(),
            time_per_sample: SampleRate::default().into(),
            bpm: Default::default(),
            bpm_lfo_multiplier: BeatsPerMinute::default().into(),
            global_pitch_bend: Default::default(),
            sustain_pedal_on: false,
            parameters: AudioParameters::default(),
            rng: Rng::new(),
            log10table: Default::default(),
            polyphonic_voices,
            monophonic_voice: Voice::new(MidiPitch::new(0), true),
            monophonic_pressed_keys,
            pending_note_events: LocalRb::new(1024),
            opt_last_voice_mode: None,
            audio_gen_data_w2: Default::default(),
            #[cfg(target_arch = "x86_64")]
            audio_gen_data_w4: Default::default(),
            #[cfg(feature = "clap")]
            clap_ended_notes: ringbuf::LocalRb::new(256),
        }
    }
}

impl AudioState {
    pub fn set_parameter_from_patch(&mut self, parameter: Parameter, value: f32) {
        self.parameters.set_parameter_from_patch(parameter, value);
    }

    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        self.sample_rate = sample_rate;
        self.time_per_sample = sample_rate.into();
    }

    pub fn set_bpm(&mut self, bpm: BeatsPerMinute) {
        self.bpm = bpm;
        self.bpm_lfo_multiplier = bpm.into();
    }

    pub fn enqueue_note_events<I: Iterator<Item = NoteEvent>>(&mut self, mut events: I) {
        self.pending_note_events.push_iter(&mut events);

        if events.next().is_some() {
            ::log::error!("Audio note event buffer full");
        }
    }

    pub fn enqueue_note_event(&mut self, event: NoteEvent) {
        if self.pending_note_events.push(event).is_err() {
            ::log::error!("Audio note event buffer full");
        }
    }

    pub fn advance_one_sample(&mut self) {
        self.parameters.advance_one_sample(self.sample_rate);

        let voice_mode = self.parameters.voice_mode.get_value();

        if let Some(last_voice_mode) = self.opt_last_voice_mode {
            match (last_voice_mode, voice_mode) {
                (VoiceMode::Polyphonic, VoiceMode::Monophonic) => {
                    self.monophonic_pressed_keys.clear();

                    for voice in self.polyphonic_voices.values_mut() {
                        voice.kill_envelopes();
                    }
                }
                (VoiceMode::Monophonic, VoiceMode::Polyphonic) => {
                    self.monophonic_pressed_keys.clear();

                    self.monophonic_voice.kill_envelopes();
                }
                _ => (),
            }
        }

        self.opt_last_voice_mode = Some(voice_mode);
    }

    #[cfg(feature = "vst2")]
    pub fn sort_note_events(&mut self) {
        let (a, b) = self.pending_note_events.as_mut_slices();

        a.sort_unstable_by_key(|e| e.delta_frames);
        b.sort_unstable_by_key(|e| e.delta_frames);
    }

    fn process_events_for_sample(&mut self, buffer_offset: usize) {
        loop {
            match self
                .pending_note_events
                .iter()
                .next()
                .map(|e| e.delta_frames as usize)
            {
                Some(event_delta_frames) if event_delta_frames == buffer_offset => {
                    let event = self.pending_note_events.pop().unwrap();

                    self.process_note_event(event.event, event_delta_frames);
                }
                _ => break,
            }
        }
    }

    fn process_note_event(&mut self, event: NoteEventInner, sample_index: usize) {
        match event {
            NoteEventInner::Midi { mut data } => {
                // Discard channel bits of status byte
                data[0] >>= 4;

                match data {
                    [0b_1000, key, _] => self.key_off(key, sample_index),
                    [0b_1001, key, 0] => self.key_off(key, sample_index),
                    [0b_1001, key, velocity] => {
                        self.key_on(key, KeyVelocity::from_midi_velocity(velocity), None)
                    }
                    [0b_1010, key, pressure] => {
                        self.aftertouch(key, KeyVelocity::from_midi_velocity(pressure));
                    }
                    [0b_1011, 64, v] => {
                        self.sustain_pedal_on = v >= 64;
                    }
                    [0b_1110, lsb, msb] => {
                        self.global_pitch_bend.update_from_midi(lsb, msb);
                    }
                    _ => (),
                }
            }
            NoteEventInner::ClapNoteOn {
                key,
                velocity,
                clap_note_id,
            } => {
                self.key_on(key, KeyVelocity(velocity as f32), Some(clap_note_id));
            }
            NoteEventInner::ClapNotePressure { key, pressure } => {
                self.aftertouch(key, KeyVelocity(pressure as f32));
            }
            NoteEventInner::ClapNoteOff { key } => {
                self.key_off(key, sample_index);
            }
            NoteEventInner::ClapBpm { bpm } => {
                self.set_bpm(bpm);
            }
        }
    }

    fn key_on(&mut self, key: u8, velocity: KeyVelocity, opt_clap_note_id: Option<i32>) {
        let voice_mode = self.parameters.voice_mode.get_value();
        let glide_active = self.parameters.glide_active.get_value();
        let glide_retrigger = self.parameters.glide_retrigger.get_value();

        match voice_mode {
            VoiceMode::Polyphonic => {
                let mut most_recent_still_pressed_keys = self
                    .polyphonic_voices
                    .iter()
                    .rev()
                    .filter(|(k, v)| **k != key && v.key_pressed)
                    .map(|(key, _)| *key);

                let opt_glide_from_key = match glide_active {
                    GlideActive::Off => None,
                    GlideActive::Legato => most_recent_still_pressed_keys.next(),
                    GlideActive::On => {
                        most_recent_still_pressed_keys
                            // Additionally look at voices in release phase. Don't filter out
                            // current voice here, since if is most recently added, we want to
                            // return None later instead of gliding from next one
                            .chain(self.polyphonic_voices.iter().rev().map(|(key, _)| *key))
                            .next()
                            .filter(|k| *k != key)
                    }
                };

                let voice = if let Some(voice) = self.polyphonic_voices.shift_remove(&key) {
                    // Shift voice to last position (most recently pressed)
                    self.polyphonic_voices.entry(key).or_insert(voice)
                } else {
                    self.polyphonic_voices
                        .entry(key)
                        .or_insert(Voice::new(MidiPitch::new(key), false))
                };

                if let Some(glide_from_key) = opt_glide_from_key {
                    let glide = VoiceGlide {
                        to_key: key,
                        time: Self::glide_time(&self.parameters, self.bpm, glide_from_key, key),
                        retrigger_envelopes: true,
                        retrigger_lfos: true,
                    };

                    voice.press_key(
                        &self.parameters,
                        velocity,
                        Some(glide_from_key),
                        Some(glide),
                        opt_clap_note_id,
                    );
                } else {
                    voice.press_key(
                        &self.parameters,
                        velocity,
                        Some(key),
                        None,
                        opt_clap_note_id,
                    );
                }
            }
            VoiceMode::Monophonic => {
                self.monophonic_pressed_keys.shift_remove(&key);
                self.monophonic_pressed_keys.insert(key, opt_clap_note_id);

                if glide_active == GlideActive::Off || !self.monophonic_voice.active {
                    self.monophonic_voice.press_key(
                        &self.parameters,
                        velocity,
                        Some(key),
                        None,
                        opt_clap_note_id,
                    );
                } else if self.monophonic_voice.key() == key {
                    // mono_voice is active and for current key: retrigger key, but don't
                    // force an initial key in case there are previous glides
                    self.monophonic_voice.press_key(
                        &self.parameters,
                        velocity,
                        None,
                        None,
                        opt_clap_note_id,
                    )
                } else if !self.monophonic_voice.key_pressed {
                    // mono voice is active for another key, but in release stage

                    if glide_active == GlideActive::Legato {
                        // trigger key press for voice with new key without glide
                        self.monophonic_voice.press_key(
                            &self.parameters,
                            velocity,
                            Some(key),
                            None,
                            opt_clap_note_id,
                        )
                    } else {
                        // in always glide mode: glide to new key and retrigger
                        // envelopes since voice is in release phase

                        let glide = VoiceGlide {
                            to_key: key,
                            time: Self::glide_time(
                                &self.parameters,
                                self.bpm,
                                self.monophonic_voice.key(),
                                key,
                            ),
                            retrigger_envelopes: true,
                            retrigger_lfos: glide_retrigger,
                        };

                        self.monophonic_voice.press_key(
                            &self.parameters,
                            velocity,
                            None,
                            Some(glide),
                            opt_clap_note_id,
                        )
                    }
                } else {
                    // mono_voice is active for a different key and is in
                    // attack/decay/sustain phase (e.g. key is pressed):
                    // trigger key press for voice with new key with glide,
                    // use glide_retrigger parameter to determine whether to
                    // retrigger envelopes and LFOs

                    let glide = VoiceGlide {
                        to_key: key,
                        time: Self::glide_time(
                            &self.parameters,
                            self.bpm,
                            self.monophonic_voice.key(),
                            key,
                        ),
                        retrigger_envelopes: glide_retrigger,
                        retrigger_lfos: glide_retrigger,
                    };

                    self.monophonic_voice.press_key(
                        &self.parameters,
                        velocity,
                        None,
                        Some(glide),
                        opt_clap_note_id,
                    )
                }
            }
        }
    }

    fn key_off(
        &mut self,
        key: u8,
        #[cfg_attr(not(feature = "clap"), allow(unused_variables))] sample_index: usize,
    ) {
        let voice_mode = self.parameters.voice_mode.get_value();
        let glide_mode = self.parameters.glide_active.get_value();
        let glide_retrigger = self.parameters.glide_retrigger.get_value();

        match voice_mode {
            VoiceMode::Polyphonic => {
                if let Some(voice) = self.polyphonic_voices.get_mut(&key) {
                    voice.release_key();
                }
            }
            VoiceMode::Monophonic => {
                let key_was_most_recently_pressed = self
                    .monophonic_pressed_keys
                    .last()
                    .map(|(k, _)| *k == key)
                    .unwrap_or(false);

                #[cfg_attr(not(feature = "clap"), allow(unused_variables))]
                let opt_removed_clap_note_id =
                    self.monophonic_pressed_keys.shift_remove(&key).flatten();

                if key_was_most_recently_pressed {
                    if let Some(next_most_recently_pressed_key) =
                        self.monophonic_pressed_keys.last().map(|(k, _)| *k)
                    {
                        // FIXME: maybe previous velocity should be stored in pressed_keys?
                        let current_velocity = self.monophonic_voice.get_key_velocity();

                        if let GlideActive::Off = glide_mode {
                            self.monophonic_voice.press_key(
                                &self.parameters,
                                current_velocity,
                                Some(next_most_recently_pressed_key),
                                None,
                                opt_removed_clap_note_id,
                            );
                        } else {
                            let glide = VoiceGlide {
                                to_key: next_most_recently_pressed_key,
                                time: Self::glide_time(
                                    &self.parameters,
                                    self.bpm,
                                    key,
                                    next_most_recently_pressed_key,
                                ),
                                retrigger_envelopes: glide_retrigger,
                                retrigger_lfos: glide_retrigger,
                            };

                            self.monophonic_voice.press_key(
                                &self.parameters,
                                current_velocity,
                                None,
                                Some(glide),
                                opt_removed_clap_note_id,
                            );
                        };

                        #[cfg(feature = "clap")]
                        if let Some(clap_note_id) = opt_removed_clap_note_id {
                            if let Err(err) = self.clap_ended_notes.push(ClapNoteEnded {
                                key,
                                clap_note_id,
                                sample_index: sample_index as u32,
                            }) {
                                ::log::error!(
                                    "clap_ended_notes buffer full, couldn't push {:?}",
                                    err
                                );
                            }
                        }
                    } else {
                        self.monophonic_voice.release_key();
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn aftertouch(&mut self, key: u8, velocity: KeyVelocity) {
        // Disabled for now
        // if let Some(voice) = self.voices.get_mut(&key) {
        //     voice.aftertouch(velocity);
        // }
    }

    #[cfg(test)]
    pub fn compare_parameter_patch_value(&mut self, parameter: Parameter, value: f32) -> bool {
        self.parameters
            .compare_patch_value(parameter, value)
            .unwrap()
    }

    fn glide_time(
        parameters: &AudioParameters,
        bpm: BeatsPerMinute,
        from_key: u8,
        to_key: u8,
    ) -> f64 {
        let mut glide_time = parameters.glide_time.get_value() as f64;

        if parameters.glide_bpm_sync.get_value() {
            glide_time *= 120.0 / bpm.0;
        }
        if let GlideMode::Lcr = parameters.glide_mode.get_value() {
            glide_time *= (from_key as f64 - to_key as f64).abs() * (1.0 / 12.0);
        }

        glide_time
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalPitchBend {
    factor: f32,
}

impl Default for GlobalPitchBend {
    fn default() -> Self {
        Self { factor: 0.0 }
    }
}

impl GlobalPitchBend {
    pub fn update_from_midi(&mut self, lsb: u8, msb: u8) {
        let amount = ((msb as u16) << 7) | (lsb as u16);

        let mut x = (amount as f32) - 8_192.0;

        // Do we really want to do this? Another option is to clamp negative
        // values at -8191 (e.g. treat -8192 as equivalent to -8191)
        if x > 0.0 {
            x *= 1.0 / 8_191.0;
        }
        if x < 0.0 {
            x *= 1.0 / 8_192.0;
        }

        self.factor = x;
    }
    pub fn as_frequency_multiplier(&self, range_up: f32, range_down: f32) -> f64 {
        let semitone_range = if self.factor >= 0.0 {
            range_up
        } else {
            -range_down
        };

        crate::math::exp2_fast(self.factor * semitone_range * (1.0 / 12.0)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::GlobalPitchBend;

    #[test]
    fn test_global_pitch_bend_from_midi() {
        let mut pitch_bend = GlobalPitchBend::default();

        pitch_bend.update_from_midi(0, 64);
        assert_eq!(pitch_bend.factor, 0.0);

        pitch_bend.update_from_midi(0, 0);
        assert_eq!(pitch_bend.factor, -1.0);

        pitch_bend.update_from_midi(127, 127);
        assert_eq!(pitch_bend.factor, 1.0);
    }
}
