extern crate vst;
extern crate smallvec;

use std::f64::consts::PI;

use smallvec::{SmallVec, smallvec};
use vst::buffer::AudioBuffer;


pub const TAU: f64 = 2.0 * PI;


/// Number that gets incremented with 1.0 every second
#[derive(Debug, Copy, Clone)]
pub struct GlobalTime(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct NoteTime(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct MasterFrequency(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct SampleRate(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct NoteDuration(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct WaveScale(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch(pub u8);

impl MidiPitch {
    pub fn get_frequency(&self, master_frequency: MasterFrequency) -> f64 {
        let note_diff = (self.0 as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency.0
    }
}


#[derive(Debug, Copy, Clone)]
pub enum WaveForm {
    Sine,
    Square,
    Sawtooth,
}


#[derive(Debug, Copy, Clone)]
pub struct Wave {
    scale: WaveScale,
    form: WaveForm,
}


#[derive(Debug, Clone)]
pub struct Note {
    duration: NoteDuration,
    midi_pitch: MidiPitch,
}

impl Note {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        Self {
            duration: NoteDuration(0.0),
            midi_pitch: midi_pitch,
        }
    }
}

pub type Notes = SmallVec<[Option<Note>; 128]>;
pub type Waves = SmallVec<[Wave; 32]>;


#[derive(Debug)]
pub struct FmSynth {
    sample_rate: SampleRate,
    master_frequency: MasterFrequency,
    global_time: GlobalTime,
    notes: Notes,
    waves: Waves,
}

impl Default for FmSynth {
    fn default() -> Self {
        let base_wave = Wave {
            scale: WaveScale(1.0),
            form: WaveForm::Sine,
        };

        Self {
            sample_rate: SampleRate(44100.0),
            master_frequency: MasterFrequency(440.0),
            global_time: GlobalTime(0.0),
            notes: smallvec![None; 128],
            waves: smallvec![base_wave],
        }
    }
}

impl FmSynth {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate.0
    }

    fn limit(&self, value: f32) -> f32 {
        value.min(1.0).max(-1.0)
    }

    /// Generate a sample
    /// 
    /// Doesn't take self parameter due to conflicting borrowing (Self.notes
    /// is borrowed mutably in the generate_audio inner loop)
    fn generate_sample(
        master_frequency: MasterFrequency,
        waves: &mut Waves,
        note: &mut Note,
        time: NoteTime,
    ) -> f64 {

        let base_frequency = note.midi_pitch.get_frequency(master_frequency);
        let mut signal = 0.0;

        for wave in waves.iter() {
            let p = time.0 * base_frequency * wave.scale.0;

            signal += match wave.form {
                WaveForm::Sine => (p * TAU).sin(),
                WaveForm::Square => ((p % 1.0).round() - 0.5) * 2.0,
                WaveForm::Sawtooth => ((p % 1.0) - 0.5) * 2.0,
            }
        }

        // Apply a quick envelope to the attack of the signal to avoid popping.
        let attack = 0.5;
        let alpha = if note.duration.0 < attack {
            note.duration.0 / attack
        } else {
            1.0
        };

        (signal * alpha * 0.1)
    }

    pub fn generate_audio(&mut self, audio_buffer: &mut AudioBuffer<f32>){
        let num_samples = audio_buffer.samples();
        let time_per_sample = self.time_per_sample();

        let outputs = audio_buffer.split().1;

        for output_buffer in outputs {
            let mut time = NoteTime(self.global_time.0);

            for output_sample in output_buffer {
                let mut out = 0.0f32;

                for opt_note in self.notes.iter_mut(){
                    if let Some(note) = opt_note {
                        out += FmSynth::generate_sample(
                            self.master_frequency,
                            &mut self.waves,
                            note,
                            time
                        ) as f32;

                        note.duration.0 += time_per_sample;
                    }
                }

                time.0 += time_per_sample;

                *output_sample = self.limit(out);
            }
        }

        self.global_time.0 += num_samples as f64 * time_per_sample;
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
            self.notes[pitch as usize] = Some(Note::new(MidiPitch(pitch)));
        }
    }

    fn note_off(&mut self, pitch: u8) {
        self.notes[pitch as usize] = None;
    }

    pub fn set_sample_rate(&mut self, rate: SampleRate) {
        self.sample_rate = rate;
    }
}