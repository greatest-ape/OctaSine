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
use rand::prelude::*;

use vst::api::{Supported, Events};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};

use vst2_helpers::approximations::*;
use vst2_helpers::presets::*;
use vst2_helpers::{crate_version_to_vst_format, crate_version, impl_plugin_parameters};

use crate::common::*;
use crate::constants::*;
use crate::voices::*;
use crate::processing_parameters::*;
use crate::preset_parameters::*;


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
    pub rng: SmallRng,
    pub log10_table: Log10Table,
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
}


impl Plugin for OctaSine {
    #[cfg(feature = "simd2")]
    fn process(&mut self, buffer: &mut AudioBuffer<f32>){
        gen::simdeez::process_f32_runtime_select(self, buffer);
    }

    #[cfg(not(feature = "simd2"))]
    fn process(&mut self, buffer: &mut AudioBuffer<f32>){
        gen::fallback::process_f32(self, buffer);
    }

    fn new(host: HostCallback) -> Self {
        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            global_time: TimeCounter(0.0),
            sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            rng: SmallRng::from_entropy(),
            log10_table: Log10Table::default(),
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
            unique_id: PLUGIN_UNIQUE_ID,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            presets: self.sync_only.presets.len() as i32,
            parameters: self.sync_only.presets.get_num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: false,
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

		let _ = simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            log_file
        );

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