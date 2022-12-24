mod common;
pub mod gen;
pub mod parameters;
pub mod voices;

use std::collections::VecDeque;

use array_init::array_init;
use fastrand::Rng;

use crate::{common::*, parameters::Parameter};

use parameters::*;
use voices::*;

use self::{gen::AudioGenData, voices::log10_table::Log10Table};

pub struct AudioState {
    sample_rate: SampleRate,
    time_per_sample: TimePerSample,
    bpm: BeatsPerMinute,
    bpm_lfo_multiplier: BpmLfoMultiplier,
    sustain_pedal_on: bool,
    parameters: AudioParameters,
    rng: Rng,
    log10table: Log10Table,
    pub voices: [Voice; 128],
    pending_note_events: VecDeque<NoteEvent>,
    audio_gen_data_w2: Box<AudioGenData<2>>,
    audio_gen_data_w4: Box<AudioGenData<4>>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            sample_rate: SampleRate::default(),
            time_per_sample: SampleRate::default().into(),
            bpm: Default::default(),
            bpm_lfo_multiplier: BeatsPerMinute::default().into(),
            sustain_pedal_on: false,
            parameters: AudioParameters::default(),
            rng: Rng::new(),
            log10table: Default::default(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            // Start with some capacity to cut down on later allocations
            pending_note_events: VecDeque::with_capacity(512),
            audio_gen_data_w2: Default::default(),
            audio_gen_data_w4: Default::default(),
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

    pub fn enqueue_note_events<I: Iterator<Item = NoteEvent>>(&mut self, events: I) {
        for event in events {
            self.pending_note_events.push_back(event);
        }
    }

    pub fn enqueue_note_event(&mut self, event: NoteEvent) {
        self.pending_note_events.push_back(event);
    }

    pub fn sort_note_events(&mut self) {
        self.pending_note_events
            .make_contiguous()
            .sort_unstable_by_key(|e| e.delta_frames);
    }

    fn process_events_for_sample(&mut self, buffer_offset: usize) {
        loop {
            match self
                .pending_note_events
                .get(0)
                .map(|e| e.delta_frames as usize)
            {
                Some(event_delta_frames) if event_delta_frames == buffer_offset => {
                    let event = self.pending_note_events.pop_front().unwrap();

                    self.process_note_event(event.event);
                }
                _ => break,
            }
        }
    }

    fn process_note_event(&mut self, event: NoteEventInner) {
        match event {
            NoteEventInner::Midi { mut data } => {
                // Discard channel bits of status byte
                data[0] >>= 4;

                match data {
                    [0b_1000, pitch, _] => self.key_off(pitch),
                    [0b_1001, pitch, 0] => self.key_off(pitch),
                    [0b_1001, pitch, velocity] => {
                        self.key_on(pitch, KeyVelocity::from_midi_velocity(velocity), None)
                    }
                    [0b_1011, 64, v] => {
                        self.sustain_pedal_on = v >= 64;
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
            NoteEventInner::ClapNoteOff { key } => {
                self.key_off(key);
            }
            NoteEventInner::ClapBpm { bpm } => {
                self.set_bpm(bpm);
            }
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: KeyVelocity, opt_clap_note_id: Option<i32>) {
        self.voices[pitch as usize].press_key(velocity, opt_clap_note_id);
    }

    fn key_off(&mut self, pitch: u8) {
        self.voices[pitch as usize].release_key();
    }

    #[cfg(test)]
    pub fn advance_one_sample(&mut self, sample_rate: SampleRate) {
        self.parameters.advance_one_sample(sample_rate);
    }

    #[cfg(test)]
    pub fn compare_parameter_patch_value(&mut self, parameter: Parameter, value: f32) -> bool {
        self.parameters
            .compare_patch_value(parameter, value)
            .unwrap()
    }
}
