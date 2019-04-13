extern crate vst;
extern crate smallvec;

use std::f64::consts::PI;

use smallvec::{SmallVec, smallvec};
use vst::buffer::AudioBuffer;


pub const TAU: f64 = 2.0 * PI;


#[derive(Debug, Copy, Clone)]
pub enum WaveForm {
    Sine,
    Square,
    Sawtooth,
}


#[derive(Debug, Copy, Clone)]
pub struct Wave {
    scale: f64,
    form: WaveForm,
}


#[derive(Debug, Clone)]
pub struct Note {
    duration: f64,
    midi_pitch: u8,
    waves: SmallVec<[Wave; 32]>,
}

impl Note {
    pub fn new(midi_pitch: u8) -> Self {
        let base_wave = Wave {
            scale: 1.0,
            form: WaveForm::Sine,
        };

        Self {
            duration: 0.0,
            midi_pitch: midi_pitch,
            waves: smallvec![base_wave]
        }
    }

    fn get_base_frequency(&self, master_frequency: f64) -> f64 {
        let note_diff = (self.midi_pitch as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency
    }

    pub fn generate_sample(&self, master_frequency: f64, time: f64) -> f64 {
        let base_frequency = self.get_base_frequency(master_frequency);
        let mut signal = 0.0;

        for wave in self.waves.iter() {
            let p = time * base_frequency * wave.scale;

            signal += match wave.form {
                WaveForm::Sine => (p * TAU).sin(),
                WaveForm::Square => ((p % 1.0).round() - 0.5) * 2.0,
                WaveForm::Sawtooth => ((p % 1.0) - 0.5) * 2.0,
            }
        }

        // Apply a quick envelope to the attack of the signal to avoid popping.
        let attack = 0.5;
        let alpha = if self.duration < attack {
            self.duration / attack
        } else {
            1.0
        };

        (signal * alpha * 0.1)
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

    pub fn generate_audio(&mut self, audio_buffer: &mut AudioBuffer<f32>){
        let num_samples = audio_buffer.samples();
        let time_per_sample = self.time_per_sample();

        let outputs = audio_buffer.split().1;

        for output_buffer in outputs {
            let mut time = self.global_time;

            for output_sample in output_buffer {
                let mut out = 0.0f32;

                for opt_note in self.notes.iter_mut(){
                    if let Some(note) = opt_note {
                        out += note.generate_sample(self.master_frequency, time) as f32;

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