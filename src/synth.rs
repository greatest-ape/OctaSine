use smallvec::{SmallVec, smallvec};

use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::HostCallback;

use crate::common::*;
use crate::constants::*;
use crate::notes::*;
use crate::parameters::*;
use crate::operators::*;


pub type Notes = SmallVec<[Note; 128]>;
pub type Operators = SmallVec<[Operator; NUM_OPERATORS]>;
pub type Parameters = SmallVec<[Box<Parameter>; 256]>;


/// Non-automatable state (but not necessarily impossible to change from host)
pub struct InternalState {
    pub global_time: TimeCounter,
    pub sample_rate: SampleRate,
    pub parameters: Parameters,
    pub bpm: BeatsPerMinute,
}


/// State that can be automated
pub struct AutomatableState {
    pub master_frequency: MasterFrequency,
    pub operators: Operators,
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
        let mut operators = smallvec![];

        for _ in 0..NUM_OPERATORS {
            operators.push(Operator::default());
        }

        let mut parameters: SmallVec<[Box<Parameter>; 256]> = SmallVec::new();

        for (i, _) in operators.iter().enumerate(){
            parameters.push(Box::new(OperatorVolumeParameter::new(i)));
            parameters.push(Box::new(OperatorSkipChainFactorParameter::new(i)));
            parameters.push(Box::new(OperatorModulationIndexParameter::new(i)));
            parameters.push(Box::new(OperatorFeedbackParameter::new(i)));
            parameters.push(Box::new(OperatorFrequencyRatioParameter::new(i)));
            parameters.push(Box::new(OperatorFrequencyFreeParameter::new(i)));
            parameters.push(Box::new(OperatorFrequencyFineParameter::new(i)));
            parameters.push(Box::new(OperatorVolumeEnvelopeAttackDurationParameter::new(i)));
            parameters.push(Box::new(OperatorVolumeEnvelopeAttackValueParameter::new(i)));
            parameters.push(Box::new(OperatorVolumeEnvelopeDecayDurationParameter::new(i)));
            parameters.push(Box::new(OperatorVolumeEnvelopeDecayValueParameter::new(i)));
            parameters.push(Box::new(OperatorVolumeEnvelopeReleaseDurationParameter::new(i)));
        }

        let mut notes = SmallVec::new();

        for i in 0..128 {
            notes.push(Note::new(MidiPitch(i)));
        }

        let external = AutomatableState {
            master_frequency: MasterFrequency(440.0),
            notes: notes,
            operators: operators,
        };

        let internal = InternalState {
            global_time: TimeCounter(0.0),
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
        sample_rate: SampleRate,
        master_frequency: MasterFrequency,
        operators: &mut Operators,
        note: &mut Note,
        time: TimeCounter,
    ) -> f64 {
        let base_frequency = note.midi_pitch.get_frequency(master_frequency);

        let mut side_signal = 0.0;
        let mut chain_signal = 0.0;

        for (operator_index, operator) in (operators.iter_mut().enumerate()).rev() {
            // New signal generation for sine FM
            let new_signal = {
                let frequency = base_frequency *
                    operator.frequency_ratio.0 *
                    operator.frequency_free.0 *
                    operator.frequency_fine.0;

                let phase_increment = (frequency / sample_rate.0) * TAU;
                let new_phase = note.operators[operator_index].last_phase.0 + phase_increment;

                let new_feedback = operator.feedback.get_value(time) * new_phase.sin();

                let signal = (
                    new_phase +
                    operator.modulation_index.get_value(time) *
                    (chain_signal + new_feedback)
                ).sin();

                note.operators[operator_index].last_phase.0 = new_phase;

                signal
            };

            // Volume envelope
            let new_signal = new_signal * {
                let note_envelope = &mut note.operators[operator_index].volume_envelope;

                note_envelope.calculate_volume(
                    &operator.volume_envelope,
                    note.pressed,
                    note.duration
                )
            };

            side_signal += operator.volume.get_value(time) * new_signal *
                operator.skip_chain_factor.get_value(time);

            chain_signal =
                chain_signal * operator.skip_chain_factor.get_value(time) +
                operator.volume.get_value(time) *
                (1.0 - operator.skip_chain_factor.get_value(time)) *
                new_signal;
        }

        let signal = chain_signal + side_signal;

        (signal * 0.1)
    }

    pub fn generate_audio(&mut self, audio_buffer: &mut AudioBuffer<f32>){
        let time_per_sample = self.time_per_sample();

        let outputs = audio_buffer.split().1;

        for (output_sample_left, output_sample_right) in outputs.get_mut(0)
            .iter_mut()
            .zip(outputs.get_mut(1).iter_mut()) {

            let mut out = 0.0f32;

            for note in self.automatable.notes.iter_mut(){
                if note.active {
                    out += Self::generate_note_sample(
                        self.internal.sample_rate,
                        self.automatable.master_frequency,
                        &mut self.automatable.operators,
                        note,
                        self.internal.global_time,
                    ) as f32;

                    note.deactivate_if_all_operators_finished();

                    note.duration.0 += time_per_sample;

                    for operator in self.automatable.operators.iter_mut(){
                        operator.duration.0 += time_per_sample;
                    }
                }
            }

            self.internal.global_time.0 += time_per_sample;

            let output_sample = self.limit(out);

            *output_sample_left = output_sample;
            *output_sample_right = output_sample;
        }
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _   => ()
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