use smallvec::{SmallVec, smallvec};

use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::HostCallback;

use crate::constants::*;
use crate::parameters::*;
use crate::waves::*;


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
pub struct BeatsPerMinute(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct NoteDuration(pub f64);


#[derive(Debug, Copy, Clone)]
pub struct MidiPitch(pub u8);

impl MidiPitch {
    pub fn get_frequency(&self, master_frequency: MasterFrequency) -> f64 {
        let note_diff = (self.0 as i8 - 69) as f64;

        (note_diff / 12.0).exp2() * master_frequency.0
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EnvelopeStage {
    Attack,
    Sustain,
    Release,
}


#[derive(Debug, Copy, Clone)]
pub struct NoteWaveVolumeEnvelope {
    stage: EnvelopeStage,
    duration_at_state_change: f64,
    pre_state_change_volume: f64,
    last_volume: f64,
}

impl NoteWaveVolumeEnvelope {

    /// Calculate volume and possibly advance envelope stage
    pub fn calculate_volume(
        &mut self,
        wave_envelope: &WaveVolumeEnvelope,
        note_active: &mut bool,
        note_pressed: bool,
        note_duration: NoteDuration,
    ) -> f64 {
        let effective_duration = note_duration.0 - self.duration_at_state_change;

        let volume = match self.stage {
            EnvelopeStage::Attack => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);

                    self.last_volume
                }
                else if effective_duration < wave_envelope.attack_duration.0 {
                    (effective_duration / wave_envelope.attack_duration.0) * wave_envelope.attack_end_value
                }
                else {
                    self.change_stage(EnvelopeStage::Sustain, note_duration);

                    wave_envelope.attack_end_value
                }
            },
            EnvelopeStage::Sustain => {
                if !note_pressed {
                    self.change_stage(EnvelopeStage::Release, note_duration);
                }

                wave_envelope.attack_end_value
            },
            EnvelopeStage::Release => {
                if effective_duration < wave_envelope.release_duration.0 {
                    ((1.0 - (effective_duration / wave_envelope.release_duration.0)) * self.pre_state_change_volume)
                }
                else {
                    self.change_stage(EnvelopeStage::Attack, NoteDuration(0.0));

                    *note_active = false;

                    0.0
                }
            },
        };

        self.last_volume = volume;

        volume
    }

    pub fn change_stage(&mut self, new_stage: EnvelopeStage, note_duration: NoteDuration){
        self.stage = new_stage;
        self.duration_at_state_change = note_duration.0;
        self.pre_state_change_volume = self.last_volume;
    }
}

impl Default for NoteWaveVolumeEnvelope {
    fn default() -> Self {
        Self {
            stage: EnvelopeStage::Attack,
            duration_at_state_change: 0.0,
            pre_state_change_volume: 0.0,
            last_volume: 0.0
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NoteWave {
    volume_envelope: NoteWaveVolumeEnvelope,
}

impl Default for NoteWave {
    fn default() -> Self {
        Self {
            volume_envelope: NoteWaveVolumeEnvelope::default(),
        }
    }
}


pub type NoteWaves = SmallVec<[NoteWave; NUM_WAVES]>;


#[derive(Debug, Clone)]
pub struct Note {
    pressed: bool,
    active: bool,
    duration: NoteDuration,
    midi_pitch: MidiPitch,
    waves: NoteWaves,
}

impl Note {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        let mut waves = SmallVec::new();

        for _ in 0..NUM_WAVES {
            waves.push(NoteWave::default());
        }

        Self {
            pressed: false,
            active: false,
            midi_pitch: midi_pitch,
            duration: NoteDuration(0.0),
            waves: waves,
        }
    }

    pub fn press(&mut self){
        self.pressed = true;
        self.active = true;
        self.duration = NoteDuration(0.0);

        for wave in self.waves.iter_mut(){
            *wave = NoteWave::default();
        }
    }

    pub fn release(&mut self){
        if self.active {
            self.pressed = false;
        }
    }
}


pub type Notes = SmallVec<[Note; 128]>;
pub type Waves = SmallVec<[Wave; NUM_WAVES]>;
pub type Parameters = Vec<Box<Parameter>>;


/// Non-automatable state (but not necessarily impossible to change from host)
pub struct InternalState {
    pub global_time: GlobalTime,
    pub sample_rate: SampleRate,
    pub parameters: Parameters,
    pub bpm: BeatsPerMinute,
}


/// State that can be automated
pub struct AutomatableState {
    pub master_frequency: MasterFrequency,
    pub waves: Waves,
    pub notes: Notes,
}


/// Main structure
/// 
/// Split state between internal/automatable could maybe be avoided using
/// references and explicit lifetimes
pub struct FmSynth {
    internal: InternalState,
    automatable: AutomatableState,
    host: HostCallback,
}

impl FmSynth {
    pub fn new(host: HostCallback) -> Self {
        let mut waves = smallvec![];

        for _ in 0..NUM_WAVES {
            waves.push(Wave::default());
        }

        let mut parameters: Vec<Box<Parameter>> = Vec::new();

        for (i, _) in waves.iter().enumerate(){
            parameters.push(Box::new(WaveMixParameter::new(&waves, i)));
            parameters.push(Box::new(WaveRatioParameter::new(&waves, i)));
            parameters.push(Box::new(WaveFrequencyFreeParameter::new(&waves, i)));
            parameters.push(Box::new(WaveBetaParameter::new(&waves, i)));
            // parameters.push(Box::new(WaveFeedbackParameter::new(&waves, i)));
            parameters.push(Box::new(WaveVolumeEnvelopeAttackDurationParameter::new(&waves, i)));
            parameters.push(Box::new(WaveVolumeEnvelopeReleaseDurationParameter::new(&waves, i)));
        }

        let mut notes = SmallVec::new();

        for i in 0..128 {
            notes.push(Note::new(MidiPitch(i)));
        }

        let external = AutomatableState {
            master_frequency: MasterFrequency(440.0),
            notes: notes,
            waves: waves,
        };

        let internal = InternalState {
            global_time: GlobalTime(0.0),
            sample_rate: SampleRate(44100.0),
            parameters: parameters,
            bpm: BeatsPerMinute(120.0),
        };

        Self {
            internal: internal,
            automatable: external,
            host: host,
        }
    }

    pub fn init(&mut self){
        self.request_bpm();
    }

    pub fn set_sample_rate(&mut self, rate: SampleRate) {
        self.internal.sample_rate = rate;
    }

    fn request_bpm(&mut self){
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        if let Some(time_info) = self.host.get_time_info(1 << 10) {
            self.internal.bpm = BeatsPerMinute(time_info.tempo);
        }
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


        for (wave_index, wave) in (waves.iter_mut().enumerate()).rev() {
            let p = time.0 * base_frequency * wave.ratio.0 * wave.frequency_free.0;

            // Calculate attack to use to try to prevent popping
            let attack = 0.0002;
            let alpha = if wave.duration.0 < attack {
                wave.duration.0 / attack
            } else {
                1.0
            };

            // New signal generation for sine FM
            let new_signal = {
                let new = alpha * p * TAU;
                let new_feedback = new.sin();

                (new + wave.feedback.0 * new_feedback + wave.beta.0 * signal).sin()
            };

            // Volume envelope
            let new_signal = new_signal * {
                let note_envelope = &mut note.waves[wave_index].volume_envelope;

                note_envelope.calculate_volume(
                    &wave.volume_envelope,
                    &mut note.active,
                    note.pressed,
                    note.duration
                )
            };

            // Calculate mix between old and new signal
            let mix = {
                let old_signal_mix = signal * (1.0 - wave.mix.0);
                let new_signal_mix = wave.mix.0 * new_signal;

                old_signal_mix + new_signal_mix
            };

            signal = mix;
        }

        // Apply a quick envelope to the attack of the signal to avoid popping.
        let attack = 0.01;
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

        let mut time = NoteTime(self.internal.global_time.0);

        for (output_sample_left, output_sample_right) in outputs.get_mut(0).iter_mut().zip(outputs.get_mut(1).iter_mut()) {
            let mut out = 0.0f32;

            for note in self.automatable.notes.iter_mut(){
                if note.active {
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

            let output_sample = self.limit(out);

            *output_sample_left = output_sample;
            *output_sample_right = output_sample;
        }

        self.internal.global_time.0 += num_samples as f64 * time_per_sample;
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            m => {
                info!("got midi message {}", m);
            }
        }
    }

    fn note_on(&mut self, pitch: u8) {
        self.automatable.notes[pitch as usize].press();
    }

    fn note_off(&mut self, pitch: u8) {
        self.automatable.notes[pitch as usize].release();
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