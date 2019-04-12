extern crate vst;
extern crate smallvec;

use std::f64::consts::PI;

use smallvec::{SmallVec, smallvec};
use vst::buffer::AudioBuffer;


pub const TAU: f64 = 2.0 * PI;


#[derive(Debug, Copy, Clone)]
pub struct Note {
    duration: f64,
    midi_pitch: u8,
}

impl Note {
    pub fn new(midi_pitch: u8) -> Self {
        Self {
            duration: 0.0,
            midi_pitch: midi_pitch,
        }
    }

    pub fn get_frequency(&self, master_frequency: f64) -> f64 {
        let note_diff = (self.midi_pitch as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency
    }
}


pub type Notes = SmallVec<[Option<Note>; 128]>;


#[derive(Debug)]
pub struct FmSynth {
    pub sample_rate: f64,
    pub master_frequency: f64,
    pub global_time: f64,
    pub notes: Notes,
}

impl Default for FmSynth {
    fn default() -> Self {
        Self {
            sample_rate: 44100.0,
            master_frequency: 440.0,
            global_time: 0.0,
            notes: smallvec![None; 128],
        }
    }
}

impl FmSynth {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn limit(&self, value: f32) -> f32 {
        value.min(1.0).max(-1.0)
    }

    pub fn generate_audio(&mut self, buffer: &mut AudioBuffer<f32>){
        let num_samples = buffer.samples();
        let time_per_sample = self.time_per_sample();

        for (input_buffer, output_buffer) in buffer.zip() {
            let mut time = self.global_time;

            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
                let mut out = 0.0f32;

                for opt_note in self.notes.iter_mut(){
                    if let Some(note) = opt_note {
                        let signal = (time * note.get_frequency(self.master_frequency) * TAU).sin();

                        // Apply a quick envelope to the attack of the signal to avoid popping.
                        let attack = 0.5;
                        let alpha = if note.duration < attack {
                            note.duration / attack
                        } else {
                            1.0
                        };

                        out += (signal * alpha * 0.1) as f32;

                        note.duration += time_per_sample;
                    }
                }

                time += time_per_sample;

                *output_sample = self.limit(out);
            }
        }

        self.global_time += num_samples as f64 * time_per_sample;
    }

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, pitch: u8) {
        if self.notes[pitch as usize].is_none(){
            self.notes[pitch as usize] = Some(Note::new(pitch));
        }
    }

    fn note_off(&mut self, pitch: u8) {
        self.notes[pitch as usize] = None;
    }
}