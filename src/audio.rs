extern crate vst;
extern crate smallvec;

use std::f64::consts::PI;

use smallvec::{SmallVec, smallvec};
use vst::buffer::AudioBuffer;

use crate::utils::*;


pub const TAU: f64 = 2.0 * PI;

pub const NUM_WAVES: usize = 4;

pub const WAVE_DEFAULT_VOLUME: f64 = 1.0;
pub const WAVE_DEFAULT_RATIO: f64 = 1.0;
pub const WAVE_DEFAULT_FREQUENCY_FREE: f64 = 1.0;
pub const WAVE_DEFAULT_FEEDBACK: f64 = 0.0;
pub const WAVE_DEFAULT_BETA: f64 = 1.0;

pub const WAVE_RATIO_STEPS: [f64; 18] = [0.125, 0.2, 0.25, 0.33, 0.5, 0.66, 0.75, 1.0, 1.25, 1.33, 1.5, 1.66, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];
pub const WAVE_BETA_STEPS: [f64; 16] = [0.0, 0.01, 0.1, 0.2, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 35.0, 50.0, 75.0, 100.0, 1000.0];


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
pub struct WaveDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct WaveVolume(pub f64);

impl WaveVolume {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_VOLUME / 2.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveRatio(pub f64);

impl WaveRatio {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step(&WAVE_RATIO_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_RATIO_STEPS[..], 1.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFrequencyFree(pub f64);

impl WaveFrequencyFree {
    pub fn from_host_value(&self, value: f64) -> f64 {
        (value + 0.5).powf(3.0)
    }
    pub fn get_default_host_value(&self) -> f64 {
        0.5
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveFeedback(pub f64);

impl WaveFeedback {
    pub fn from_host_value(&self, value: f64) -> f64 {
        value * 5.0
    }
    pub fn get_default_host_value(&self) -> f64 {
        WAVE_DEFAULT_FEEDBACK / 5.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaveBeta(pub f64);

impl WaveBeta {
    pub fn from_host_value(&self, value: f64) -> f64 {
        map_host_param_value_to_step_smooth(&WAVE_BETA_STEPS[..], value)
    }
    pub fn get_default_host_value(&self) -> f64 {
        get_host_value_for_default_step(&WAVE_BETA_STEPS[..], WAVE_DEFAULT_BETA)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch(pub u8);

impl MidiPitch {
    pub fn get_frequency(&self, master_frequency: MasterFrequency) -> f64 {
        let note_diff = (self.0 as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency.0
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Wave {
    duration: WaveDuration,
    volume: WaveVolume,
    ratio: WaveRatio,
    frequency_free: WaveFrequencyFree,
    feedback: WaveFeedback,
    beta: WaveBeta,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            duration: WaveDuration(0.0),
            volume: WaveVolume(WAVE_DEFAULT_VOLUME),
            ratio: WaveRatio(WAVE_DEFAULT_RATIO),
            frequency_free: WaveFrequencyFree(WAVE_DEFAULT_FREQUENCY_FREE),
            feedback: WaveFeedback(WAVE_DEFAULT_FEEDBACK),
            beta: WaveBeta(WAVE_DEFAULT_BETA),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Note {
    duration: NoteDuration,
    midi_pitch: MidiPitch,
}

impl Note {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        Self {
            midi_pitch: midi_pitch,
            duration: NoteDuration(0.0),
        }
    }
}


pub trait Parameter {
    fn get_name(&self, state: &AutomatableState) -> String;
    fn get_unit_of_measurement(&self, _: &AutomatableState) -> String {
        "".to_string()
    }

    fn get_value_float(&self, state: &AutomatableState) -> f64;
    fn get_value_text(&self, state: &AutomatableState) -> String {
        format!("{:.2}", self.get_value_float(state))
    }

    fn set_value_float(&mut self, state: &mut AutomatableState, value: f64);
    fn set_value_text(&mut self, _: &mut AutomatableState, _: String) -> bool {
        false
    }
}


#[macro_export]
macro_rules! derive_wave_field_parameter {
    ($parameter_struct:ident, $field:ident, $field_name:expr) => {
        impl $parameter_struct {
            pub fn get_wave_index(&self) -> usize {
                self.wave_index
            }

            pub fn new(waves: &Waves, wave_index: usize) -> Self {
                Self {
                    wave_index: wave_index,
                    host_value: waves[wave_index].$field.get_default_host_value(),
                }
            }
        }
        impl Parameter for $parameter_struct {
            fn get_name(&self, _: &AutomatableState) -> String {
                format!("Wave {} {}", self.wave_index + 1, $field_name)
            }

            fn get_value_float(&self, _: &AutomatableState) -> f64 {
                self.host_value
            }
            fn get_value_text(&self, state: &AutomatableState) -> String {
                format!("{:.2}", state.waves[self.get_wave_index()].$field.0)
            }

            fn set_value_float(&mut self, state: &mut AutomatableState, value: f64) {
                let transformed = state.waves[
                    self.get_wave_index()
                ].$field.from_host_value(value);

                state.waves[self.get_wave_index()].$field.0 = transformed;

                state.waves[self.get_wave_index()].duration.0 = 0.0;

                self.host_value = value;
            }
        }
    };  
}


pub struct WaveRatioParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveRatioParameter, ratio, "ratio");


pub struct WaveFrequencyFreeParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveFrequencyFreeParameter, frequency_free, "free");


pub struct WaveFeedbackParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveFeedbackParameter, feedback, "feedback");


/// Frequency modulation index
pub struct WaveBetaParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveBetaParameter, beta, "beta");


/// Frequency modulation index
pub struct WaveVolumeParameter {
    wave_index: usize,
    host_value: f64,
}

derive_wave_field_parameter!(WaveVolumeParameter, volume, "volume");



pub type Notes = SmallVec<[Option<Note>; 128]>;
pub type Waves = SmallVec<[Wave; NUM_WAVES]>;
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

        for _ in 0..NUM_WAVES {
            waves.push(Wave::default());
        }

        let mut parameters: Vec<Box<Parameter>> = Vec::new();

        for (i, _) in waves.iter().enumerate(){
            parameters.push(Box::new(WaveVolumeParameter::new(&waves, i)));
            parameters.push(Box::new(WaveRatioParameter::new(&waves, i)));
            // parameters.push(Box::new(WaveFeedbackParameter::new(&waves, i)));
            parameters.push(Box::new(WaveFrequencyFreeParameter::new(&waves, i)));
            parameters.push(Box::new(WaveBetaParameter::new(&waves, i)));
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

    /// Generate a sample for a note
    /// 
    /// Doesn't take self parameter due to conflicting borrowing (Self.notes
    /// is borrowed mutably in the generate_audio inner loop)
    fn generate_note_sample(
        master_frequency: MasterFrequency,
        waves: &mut Waves,
        note: &mut Note,
        time: NoteTime,
    ) -> f64 {

        let base_frequency = note.midi_pitch.get_frequency(master_frequency);
        let mut signal = 0.0;

        for wave in (waves.iter_mut()).rev() {
            let p = time.0 * base_frequency * wave.ratio.0 * wave.frequency_free.0;

            // Try to prevent popping by slowly adding the signal
            let attack = 0.0002;
            let alpha = if wave.duration.0 < attack {
                wave.duration.0 / attack
            } else {
                1.0
            };

            let new = alpha * p * TAU;
            let feedback = wave.feedback.0 * new.sin();

            signal = signal * (1.0 - wave.volume.0) + wave.volume.0 * (new + wave.beta.0 * signal + feedback).sin();
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
                        out += Self::generate_note_sample(
                            self.automatable.master_frequency,
                            &mut self.automatable.waves,
                            note,
                            time,
                        ) as f32;

                        note.duration.0 += time_per_sample;

                        for wave in self.automatable.waves.iter_mut(){
                            wave.duration.0 += time_per_sample;
                        }
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