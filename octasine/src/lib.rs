#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

pub mod common;
pub mod constants;
pub mod gen;
pub mod voices;
pub mod processing_parameters;
pub mod preset_parameters;

use std::sync::Arc;

use array_init::array_init;

use rand::FromEntropy;
use rand::rngs::SmallRng;

use vst::api::{Supported, Events};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::host::Host;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};

use vst2_helpers::approximations::*;
use vst2_helpers::presets::*;
use vst2_helpers::processing_parameters::*;
use vst2_helpers::{crate_version_to_vst_format, crate_version, impl_plugin_parameters};

use crate::gen::*;


#[allow(clippy::let_and_return)]
#[allow(unused_mut)]
pub fn built_in_presets<P>() -> Vec<Preset<P>> where P: PresetParameters {
    let mut presets = Vec::new();

    // presets.push(preset_from_file!("../presets/test.fxp"));

    presets
}


/// State used for processing
pub struct ProcessingState {
    pub global_time: TimeCounter,
    pub sample_rate: SampleRate,
    pub time_per_sample: TimePerSample,
    pub bpm: BeatsPerMinute,
    pub rng: SmallRng,
    pub envelope_curve_table: EnvelopeCurveTable,
    pub voices: [Voice; 128],
    pub parameters: ProcessingParameters,
}


/// Thread-safe state used for parameter and preset calls
pub struct SyncOnlyState {
    pub host: HostCallback,
    pub presets: PresetBank<OctaSinePresetParameters>,
}


/// Main structure
pub struct OctaSine {
    processing: ProcessingState,
    pub sync_only: Arc<SyncOnlyState>,
}

impl Default for OctaSine {
    fn default() -> Self {
        Self::new(HostCallback::default())
    }
}


impl OctaSine {
    fn time_per_sample(sample_rate: SampleRate) -> TimePerSample {
        TimePerSample(1.0 / sample_rate.0)
    }

    fn hard_limit(value: f64) -> f64 {
        value.min(1.0).max(-1.0)
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.key_off(data[1]),
            144 => self.key_on(data[1], data[2]),
            _   => ()
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
        self.processing.voices[pitch as usize].press_key(velocity);
    }

    fn key_off(&mut self, pitch: u8) {
        self.processing.voices[pitch as usize].release_key();
    }

    /// Fetch BPM. Currently not used
    #[allow(dead_code)]
    fn fetch_bpm(&mut self){
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        if let Some(time_info) = self.sync_only.host.get_time_info(1 << 10) {
            self.processing.bpm = BeatsPerMinute(time_info.tempo as f64);
        }
    }
    
    #[inline]
    fn gen_samples_for_voices(&mut self) -> (f64, f64) {
        let changed_preset_parameters = self.sync_only.presets
            .get_changed_parameters();

        if let Some(indeces) = changed_preset_parameters {
            for (index, opt_new_value) in indeces.iter().enumerate(){
                if let Some(new_value) = opt_new_value {
                    if let Some(p) = self.processing.parameters.get(index){
                        p.set_from_preset_value(*new_value);
                    }
                }
            }
        }

        let mut voice_sum_left: f64 = 0.0;
        let mut voice_sum_right: f64 = 0.0;

        let time_per_sample = self.processing.time_per_sample;

        for voice in self.processing.voices.iter_mut(){
            if voice.active {
                #[cfg(feature = "simd")]
                let (out_left, out_right) = generate_voice_samples_simd(
                    &self.processing.envelope_curve_table,
                    &mut self.processing.rng,
                    self.processing.global_time,
                    time_per_sample,
                    &mut self.processing.parameters,
                    voice,
                );
                #[cfg(not(feature = "simd"))]
                let (out_left, out_right) = generate_voice_samples(
                    &self.processing.envelope_curve_table,
                    &mut self.processing.rng,
                    self.processing.global_time,
                    time_per_sample,
                    &mut self.processing.parameters,
                    voice,
                );

                voice_sum_left += Self::hard_limit(out_left);
                voice_sum_right += Self::hard_limit(out_right);

                voice.duration.0 += time_per_sample.0;

                voice.deactivate_if_envelopes_ended();
            }
        }

        self.processing.global_time.0 += time_per_sample.0;

        (voice_sum_left, voice_sum_right)
    }
}


/// OctaSine process functions (for f32 and f64)
macro_rules! create_process_fn {
    ($fn_name:ident, $type:ty) => {
        #[inline]
        fn $fn_name(&mut self, audio_buffer: &mut AudioBuffer<$type>){
            let outputs = audio_buffer.split().1;
            let lefts = outputs.get_mut(0).iter_mut();
            let rights = outputs.get_mut(1).iter_mut();

            for (buffer_sample_left, buffer_sample_right) in lefts.zip(rights){
                let (left, right) = self.gen_samples_for_voices();

                *buffer_sample_left = left as $type;
                *buffer_sample_right = right as $type;
            }
        }
    };
}


impl Plugin for OctaSine {
    create_process_fn!(process, f32);
    create_process_fn!(process_f64, f64);

    fn new(host: HostCallback) -> Self {
        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            global_time: TimeCounter(0.0),
            sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            bpm: BeatsPerMinute(120.0),
            rng: SmallRng::from_entropy(),
            envelope_curve_table: EnvelopeCurveTable::default(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: ProcessingParameters::default(),
        };

        let sync_only = Arc::new(SyncOnlyState {
            host,
            presets: PresetBank::new_from_presets(built_in_presets()),
        });

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
            presets: self.sync_only.presets.len() as i32,
            parameters: self.sync_only.presets.get_num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: true,
            ..Info::default()
        }
    }

    #[cfg(feature = "logging")]
	fn init(&mut self) {
        let log_folder = dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

		let log_file = ::std::fs::File::create(
            log_folder.join(format!("{}.log", PLUGIN_NAME))
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

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.sync_only) as Arc<dyn PluginParameters>
    }
}


impl_plugin_parameters!(SyncOnlyState, presets);