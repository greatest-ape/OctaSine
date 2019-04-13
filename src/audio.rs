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


pub trait Parameter {
    fn get_name(&self, state: &AutomatableState) -> String;
    fn get_unit_of_measurement(&self, state: &AutomatableState) -> String;

    fn get_value_float(&self, state: &AutomatableState) -> f64;
    fn get_value_text(&self, state: &AutomatableState) -> String {
        format!("{:.2}", self.get_value_float(state))
    }

    fn set_value_float(&mut self, state: &mut AutomatableState, value: f64);
    fn set_value_text(&mut self, _: &mut AutomatableState, _: String) -> bool {
        false
    }
}


pub struct WaveFormParameter {
    wave_index: usize,
}


impl Parameter for WaveFormParameter {
    fn get_name(&self, _: &AutomatableState) -> String {
        format!("Wave {} waveform", self.wave_index + 1)
    }

    fn get_unit_of_measurement(&self, _: &AutomatableState) -> String {
        "".to_string()
    }

    fn get_value_float(&self, _: &AutomatableState) -> f64 {
        0.0
    }

    fn get_value_text(&self, state: &AutomatableState) -> String {
        let value = match state.waves[self.wave_index].form {
            WaveForm::Sine => "Sine",
            WaveForm::Square => "Square",
            WaveForm::Sawtooth => "Saw",
        };

        value.to_string()
    }

    fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
        state.waves[self.wave_index].form = {
            if value <= 0.33 {
                WaveForm::Sine
            }
            else if value <= 0.66 {
                WaveForm::Square
            }
            else {
                WaveForm::Sawtooth
            }
        }
    }
}


pub type Notes = SmallVec<[Option<Note>; 128]>;
pub type Waves = SmallVec<[Wave; 32]>;
pub type Parameters = Vec<Box<Parameter>>;


/// Non-automatable state (but not necessarily impossible to change from host)
pub struct InternalState {
    global_time: GlobalTime,
    sample_rate: SampleRate,
    parameters: Parameters,
}


/// State that can be automated
pub struct AutomatableState {
    master_frequency: MasterFrequency,
    waves: Waves,
    notes: Notes,
}


/// Main structure
/// 
/// Split state between internal/automatable could maybe be avoided using
/// references and explicit lifetimes
pub struct FmSynth {
    internal: InternalState,
    automatable: AutomatableState,
}

impl Default for FmSynth {
    fn default() -> Self {
        let mut waves = smallvec![];

        for _ in 0..1 {
            waves.push(Wave {
                scale: WaveScale(1.0),
                form: WaveForm::Sine,
            })
        }

        let mut parameters: Vec<Box<Parameter>> = Vec::new();

        for (i, _) in waves.iter().enumerate(){
            parameters.push(Box::new(WaveFormParameter {
                wave_index: i,
            }))
        }

        let external = AutomatableState {
            master_frequency: MasterFrequency(440.0),
            notes: smallvec![None; 128],
            waves: waves,
        };

        let internal = InternalState {
            global_time: GlobalTime(0.0),
            sample_rate: SampleRate(44100.0),
            parameters: parameters,
        };

        Self {
            internal: internal,
            automatable: external,
        }
    }
}

impl FmSynth {

    pub fn set_sample_rate(&mut self, rate: SampleRate) {
        self.internal.sample_rate = rate;
    }

    fn time_per_sample(&self) -> f64 {
        1.0 / self.internal.sample_rate.0
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
            let mut time = NoteTime(self.internal.global_time.0);

            for output_sample in output_buffer {
                let mut out = 0.0f32;

                for opt_note in self.automatable.notes.iter_mut(){
                    if let Some(note) = opt_note {
                        out += Self::generate_sample(
                            self.automatable.master_frequency,
                            &mut self.automatable.waves,
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

        self.internal.global_time.0 += num_samples as f64 * time_per_sample;
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, pitch: u8) {
        if self.automatable.notes[pitch as usize].is_none(){
            self.automatable.notes[pitch as usize] = Some(Note::new(MidiPitch(pitch)));
        }
    }

    fn note_off(&mut self, pitch: u8) {
        self.automatable.notes[pitch as usize] = None;
    }

    /// Parameter plumbing

    fn get_parameter(&self, index: usize) -> Option<&Box<Parameter>> {
        self.internal.parameters.get(index)
    }

    fn get_parameter_mut(
        internal: &mut InternalState,
        index: usize
    ) -> Option<&mut Box<Parameter>> {

        internal.parameters.get_mut(index)
    }

    pub fn get_num_parameters(&self) -> usize {
        self.internal.parameters.len()
    }

    pub fn can_parameter_be_automated(&self, index: usize) -> bool {
        self.get_parameter(index).is_some()
    }

    pub fn get_parameter_name(&self, index: usize) -> String {
        self.get_parameter(index)
            .map_or("".to_string(), |p| p.get_name(&self.automatable))
    }

    pub fn get_parameter_unit_of_measurement(&self, index: usize) -> String {
        self.get_parameter(index)
            .map_or("".to_string(), |p| p.get_unit_of_measurement(&self.automatable))
    }

    pub fn get_parameter_value_text(&self, index: usize) -> String {
        self.get_parameter(index)
            .map_or("".to_string(), |p| p.get_value_text(&self.automatable))
    }

    pub fn get_parameter_value_float(&self, index: usize) -> f64 {
        self.get_parameter(index)
            .map_or(0.0, |p| p.get_value_float(&self.automatable))
    }

    pub fn set_parameter_value_float(&mut self, index: usize, value: f64) {
        if let Some(p) = Self::get_parameter_mut(&mut self.internal, index) {
            p.set_value_float(&mut self.automatable, value.min(1.0).max(0.0))
        }
    }

    pub fn set_parameter_value_text(&mut self, index: usize, text: String) -> bool {
        if let Some(p) = Self::get_parameter_mut(&mut self.internal, index){
            p.set_value_text(&mut self.automatable, text)
        }
        else {
            false
        }
    }
}