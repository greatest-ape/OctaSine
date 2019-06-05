#[macro_use]
extern crate log;

use std::sync::Arc;

use array_init::array_init;
use parking_lot::Mutex;
use rand::{FromEntropy, Rng};
use rand::rngs::SmallRng;

use vst::api::{Supported, Events};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::host::Host;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};
use vst::plugin_main;

pub mod common;
pub mod constants;
pub mod voices;
pub mod parameters;
pub mod presets;

use crate::common::*;
use crate::constants::*;
use crate::voices::*;
use crate::parameters::*;
use crate::presets::*;


#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION").to_string()
    };
}

fn crate_version_to_vst_format(crate_version: String) -> i32 {
    format!("{:0<4}", crate_version.replace(".", ""))
        .parse()
        .expect("convert crate version to i32")
}


#[macro_export]
macro_rules! impl_parameter_value_access_interpolatable {
    ($struct_name:ident) => {
        impl ParameterInternalValueAccess<f64> for $struct_name {
            fn set_converted_parameter_value(&mut self, value: f64){
                self.value.set_value(value);
            }
            fn get_unconverted_parameter_value(&self) -> f64 {
                self.value.target_value
            }
        }
    };
}


#[macro_export]
macro_rules! impl_parameter_value_access_simple {
    ($struct_name:ident) => {
        impl ParameterInternalValueAccess<f64> for $struct_name {
            fn set_converted_parameter_value(&mut self, value: f64){
                self.value = value;
            }
            fn get_unconverted_parameter_value(&self) -> f64 {
                self.value
            }
        }
    };
}


#[macro_export]
macro_rules! impl_parameter_string_parsing_simple {
    ($struct_name:ident) => {
        impl ParameterStringParsing<f64> for $struct_name {
            fn parse_string_value(&self, value: String) -> Option<f64> {
                value.parse::<f64>().ok().map(|value| {
                    let max = self.from_parameter_value(1.0);
                    let min = self.from_parameter_value(0.0);

                    value.max(min).min(max)
                })
            }
        }
    };
}


#[macro_export]
/// Implement ParameterValueConversion<f64> with 1-to-1 conversion
macro_rules! impl_parameter_value_conversion_identity {
    ($struct_name:ident) => {
        impl ParameterValueConversion<f64> for $struct_name {
            fn from_parameter_value(&self, value: f64) -> f64 {
                value
            }
            fn to_parameter_value(&self, value: f64) -> f64 {
                value
            }
        }
    };
}


#[macro_export]
macro_rules! impl_get_value_for_interpolatable_parameter {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn get_value(&mut self, time: TimeCounter) -> f64 {
                self.value.get_value(time, &mut |_| ())
            }
        }
    };
}


/// State used for processing
pub struct ProcessingState {
    pub global_time: TimeCounter,
    pub sample_rate: SampleRate,
    pub time_per_sample: TimePerSample,
    pub bpm: BeatsPerMinute,
    pub rng: SmallRng,
    pub voices: [Voice; 128],
    pub parameters: Arc<Mutex<Parameters>>,
}


/// Thread-safe state used for parameter and preset calls
pub struct SyncOnlyState {
    pub host: HostCallback,
    pub parameters: Arc<Mutex<Parameters>>,
    pub presets: Arc<Mutex<Presets>>,
}

impl SyncOnlyState {
    fn modify_presets_and_set_parameters_from_current<F: Fn(&mut Presets)>(
        &self,
        f: &F
    ){
        let mut presets = self.presets.lock();

        f(&mut presets);

        presets.set_parameters_from_current_preset(
            &mut self.parameters.lock()
        );
    }

    fn set_preset_from_parameters_and_get_data<F>(
        &self,
        f: &F
    ) -> Vec<u8> where F: Fn(&mut Presets) -> Vec<u8> {
        let parameters = (*self.parameters.lock()).clone();

        let mut presets = self.presets.lock();

        presets.set_current_preset_from_parameters(parameters);

        f(&mut presets)
    }
}


/// One for left channel, one for right
pub struct OutputChannel {
    pub additive: f64,
    pub operator_inputs: [f64; NUM_OPERATORS],
}

impl Default for OutputChannel {
    fn default() -> Self {
        Self {
            additive: 0.0,
            operator_inputs: [0.0; NUM_OPERATORS],
        }
    }
}


/// Main structure
pub struct FmSynth {
    processing: ProcessingState,
    sync_only: Arc<SyncOnlyState>,
}

impl Default for FmSynth {
    fn default() -> Self {
        Self::new(HostCallback::default())
    }
}


impl FmSynth {
    fn time_per_sample(sample_rate: SampleRate) -> TimePerSample {
        TimePerSample(1.0 / sample_rate.0)
    }

    fn hard_limit(value: f64) -> f64 {
        value.min(1.0).max(-1.0)
    }

    /// Generate stereo samples for a voice
    /// 
    /// Doesn't take self parameter due to conflicting borrowing of Notes
    /// in calling function `process`
    fn generate_voice_samples(
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut Parameters,
        voice: &mut Voice,
    ) -> (f64, f64) {
        let operators = &mut parameters.operators;

        let base_frequency = voice.midi_pitch.get_frequency(
            parameters.master_frequency
        );

        let mut output_channels = [
            OutputChannel::default(),
            OutputChannel::default()
        ];

        for (operator_index, operator) in (operators.iter_mut().enumerate()).rev() {
            // Fetch all operator values here to make sure all interpolatable
            // ones are advanced even if calculations are skipped below.

            let operator_volume = operator.volume.get_value(time);
            let operator_feedback = operator.feedback.get_value(time);
            let operator_modulation_index = operator.modulation_index.get_value(time);
            let operator_panning = operator.panning.get_value(time);

            // Get additive factor; use 1.0 for operator 1
            let operator_additive = if let Some(o) = &mut operator.additive_factor {
                o.get_value(time)
            } else {
                1.0
            };

            // Get modulation target; use operator 1 for operator 1 and 2.
            // (Since additive factor is 1.0 for operator 1, its target is
            // irrelevant.)
            let operator_mod_output = if let Some(ref o) = operator.output_operator {
                o.value
            } else {
                0
            };

            let operator_frequency = base_frequency *
                operator.frequency_ratio.value *
                operator.frequency_free.value *
                operator.frequency_fine.value;

            // Always calculate envelope to make sure it advances
            let envelope_volume = {
                voice.operators[operator_index].volume_envelope.get_volume(
                    &operator.volume_envelope,
                    voice.key_pressed,
                    voice.duration
                )
            };

            // If volume is off, skip sound generation and panning
            if operator_volume < ZERO_VALUE_LIMIT ||
                envelope_volume < ZERO_VALUE_LIMIT {
                continue;
            }

            let mut operator_inputs = [
                output_channels[0].operator_inputs[operator_index],
                output_channels[1].operator_inputs[operator_index],
            ];

            // Mix modulator into current operator depending on panning of
            // current operator. If panned to the middle, just pass through
            // the stereo signals: if panned to any side, mix out the
            // original stereo signals and mix in mono.
            if operator_panning != 0.5 {
                let pan_transformed = 2.0 * (operator_panning - 0.5);

                let right_tendency = pan_transformed.max(0.0);
                let left_tendency = (-pan_transformed).max(0.0);

                let mono = operator_inputs[0] + operator_inputs[1];

                operator_inputs[0] = (1.0 - left_tendency) * operator_inputs[0] +
                    left_tendency * mono;
                operator_inputs[1] = (1.0 - right_tendency) * operator_inputs[1] +
                    right_tendency * mono;
            }

            // Calculate, save and return new phase
            let new_phase = {
                let phase_increment = TAU *
                    (operator_frequency * time_per_sample.0);

                voice.operators[operator_index].last_phase.0 += phase_increment;

                voice.operators[operator_index].last_phase.0
            };

            let mut new_signals = [0.0, 0.0];

            // Generate FM sine / noise signals for each channel
            match operator.wave_type.value {
                WaveType::Sine => {
                    // Do feedback calculation only if feedback is on
                    let new_feedback = if operator_feedback > ZERO_VALUE_LIMIT {
                        operator_feedback * new_phase.sin()
                    } else {
                        0.0
                    };

                    let inputs_identical = operator_inputs[0] == operator_inputs[1];

                    for channel in 0..2 {
                        // Skip generating right channel signal if inputs
                        // are identical - just use the left channel signal
                        if channel == 1 && inputs_identical {
                            new_signals[1] = new_signals[0];
                        } else {
                            let modulation = operator_modulation_index *
                                (operator_inputs[channel] + new_feedback);

                            let signal = (new_phase + modulation).sin();

                            new_signals[channel] = envelope_volume * signal;
                        }
                    }
                },
                WaveType::WhiteNoise => {
                    let signal = envelope_volume *
                        (rng.gen::<f64>() - 0.5) * 2.0;

                    new_signals[0] = signal;
                    new_signals[1] = signal;
                }
            }

            // Pan signals and write to output_channels
            for channel in 0..2 {
                let pan_volume = operator.panning.left_and_right[channel];

                let out = pan_volume * operator_volume * new_signals[channel];

                let additive_out = operator_additive * out;
                let mod_out = out - additive_out;

                output_channels[channel].additive += additive_out;
                output_channels[channel]
                    .operator_inputs[operator_mod_output] += mod_out;
            }
        }

        let signal_left = output_channels[0].additive;
        let signal_right = output_channels[1].additive;

        let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 *
            parameters.master_volume.get_value(time);

        (signal_left * volume_factor, signal_right * volume_factor)
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            _   => ()
        }
    }

    fn note_on(&mut self, pitch: u8, velocity: u8) {
        self.processing.voices[pitch as usize].press_key(velocity);
    }

    fn note_off(&mut self, pitch: u8) {
        self.processing.voices[pitch as usize].release_key();
    }

    fn fetch_bpm(&mut self){
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        if let Some(time_info) = self.sync_only.host.get_time_info(1 << 10) {
            self.processing.bpm = BeatsPerMinute(time_info.tempo);
        }
    }
}


impl Plugin for FmSynth {
    fn process(&mut self, audio_buffer: &mut AudioBuffer<f32>){
        let time_per_sample = self.processing.time_per_sample;

        let outputs = audio_buffer.split().1;
        let lefts = outputs.get_mut(0).iter_mut();
        let rights = outputs.get_mut(1).iter_mut();

        for (output_sample_left, output_sample_right) in lefts.zip(rights) {
            *output_sample_left = 0.0;
            *output_sample_right = 0.0;

            let mut parameters = self.processing.parameters.lock();

            for voice in self.processing.voices.iter_mut(){
                if voice.active {
                    let (out_left, out_right) = Self::generate_voice_samples(
                        &mut self.processing.rng,
                        self.processing.global_time,
                        time_per_sample,
                        &mut parameters,
                        voice,
                    );

                    *output_sample_left += Self::hard_limit(out_left) as f32;
                    *output_sample_right += Self::hard_limit(out_right) as f32;

                    voice.duration.0 += time_per_sample.0;

                    voice.deactivate_if_envelopes_ended();
                }
            }

            self.processing.global_time.0 += time_per_sample.0;
        }

        for voice in self.processing.voices.iter_mut(){
            voice.deactivate_extra_check();
        }
    }

    fn new(host: HostCallback) -> Self {
        let parameters = Arc::new(Mutex::new(Parameters::new()));
        let presets = Arc::new(Mutex::new(Presets::new()));

        let sync_only = Arc::new(SyncOnlyState {
            host: host,
            parameters: parameters.clone(),
            presets: presets.clone(),
        });

        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            global_time: TimeCounter(0.0),
            sample_rate: sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            bpm: BeatsPerMinute(120.0),
            rng: SmallRng::from_entropy(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: parameters.clone(),
        };

        Self {
            processing,
            sync_only
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: "Joakim Frostegård".to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: 43789,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            presets: self.sync_only.presets.lock().len() as i32,
            parameters: self.sync_only.parameters.lock().len() as i32,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }

	fn init(&mut self) {
        let log_folder = dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

		let log_file = ::std::fs::File::create(
            log_folder.join("rust-vst.log")
        ).unwrap();

		let _ = simplelog::CombinedLogger::init(vec![
            simplelog::WriteLogger::new(
                simplelog::LogLevelFilter::Info,
                simplelog::Config::default(),
                log_file
            )
        ]);

        log_panics::init();

		info!("init");

        self.fetch_bpm();
	}

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            if let Event::Midi(ev) = event {
                self.process_midi_event(ev.data);
            } 
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        let sample_rate = SampleRate(f64::from(rate));

        self.processing.sample_rate = sample_rate;
        self.processing.time_per_sample = Self::time_per_sample(sample_rate);
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent | CanDo::ReceiveTimeInfo
            | CanDo::SendEvents | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<PluginParameters> {
        Arc::clone(&self.sync_only) as Arc<PluginParameters>
    }
}


impl PluginParameters for SyncOnlyState {

    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, index: i32) -> String {
        self.parameters.lock().get_index(index as usize)
            .map_or("".to_string(), |p| p.get_parameter_unit_of_measurement())
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.parameters.lock().get_index(index as usize)
            .map_or("".to_string(), |p| p.get_parameter_value_text())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.parameters.lock().get_index(index as usize)
            .map_or("".to_string(), |p| p.get_parameter_name())
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.parameters.lock().get_index(index as usize)
            .map_or(0.0, |p| p.get_parameter_value_float()) as f32
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        if let Some(p) = self.parameters.lock().get_index(index as usize) {
            p.set_parameter_value_float(f64::from(value).min(1.0).max(0.0))
        }
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        if let Some(p) = self.parameters.lock().get_index(index as usize){
            p.set_parameter_value_text(text)
        }
        else {
            false
        }
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.parameters.lock().get_index(index as usize).is_some()
    }

    /// Set the current preset to the index specified by `preset`.
    ///
    /// This method can be called on the processing thread for automation.
    fn change_preset(&self, preset: i32) {
        self.modify_presets_and_set_parameters_from_current(&|presets|
            presets.change_preset(preset as usize)
        );
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.presets.lock().get_current_index() as i32
    }

    /// Set the current preset name.
    fn set_preset_name(&self, name: String) {
        self.presets.lock().set_name_of_current(name);
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, preset: i32) -> String {
        self.presets.lock().get_name_by_index(preset as usize)
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.set_preset_from_parameters_and_get_data(&|presets|
            presets.get_current_preset_as_bytes()
        )
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.set_preset_from_parameters_and_get_data(&|presets|
            presets.get_preset_bank_as_bytes()
        )
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
    /// chunk data.
    fn load_preset_data(&self, data: &[u8]) {
        self.modify_presets_and_set_parameters_from_current(&|presets|
            presets.set_current_preset_from_bytes(data)
        );
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
    /// given chunk data.
    fn load_bank_data(&self, data: &[u8]) {
        self.modify_presets_and_set_parameters_from_current(&|presets|
            presets.set_preset_bank_from_bytes(data)
        );
    }
}

plugin_main!(FmSynth);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version_to_vst_format(){
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}