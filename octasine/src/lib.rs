#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

pub mod common;
pub mod constants;
pub mod gen;
pub mod gui;
pub mod voices;
pub mod processing_parameters;
pub mod preset_parameters;
pub mod presets;

use std::sync::Arc;

use array_init::array_init;
use fastrand::Rng;

use vst::api::{Supported, Events};
use vst::editor::Editor;
use vst::event::Event;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};
use vst::host::Host;

use vst2_helpers::approximations::*;
use vst2_helpers::presets::*;
use vst2_helpers::{crate_version_to_vst_format, crate_version, impl_plugin_parameters};

use crate::common::*;
use crate::constants::*;
use crate::gui::Gui;
use crate::voices::*;
use crate::processing_parameters::*;
use crate::preset_parameters::*;


pub type OctaSinePresetBank = PresetBank<OctaSinePresetParameters>;


pub fn built_in_preset_bank<P>() -> PresetBank<P> where P: PresetParameters {
    PresetBank::new_from_bytes(include_bytes!("../presets/preset-bank.json"))
}


/// State used for processing
pub struct ProcessingState {
    pub global_time: TimeCounter,
    pub sample_rate: SampleRate,
    pub time_per_sample: TimePerSample,
    pub rng: Rng,
    pub log10_table: Log10Table,
    pub voices: [Voice; 128],
    pub parameters: ProcessingParameters,
}


/// Trait passed to GUI code for encapsulation
pub trait GuiSyncHandle: Send + Sync + 'static {
    fn get_presets(&self) -> &OctaSinePresetBank;
    fn update_host_display(&self);
}


impl GuiSyncHandle for SyncOnlyState {
    fn get_presets(&self) -> &OctaSinePresetBank {
        &self.presets
    }
    fn update_host_display(&self){
        self.host.update_display();
    }
}


impl <H: GuiSyncHandle>GuiSyncHandle for Arc<H> {
    fn get_presets(&self) -> &OctaSinePresetBank {
        ::std::ops::Deref::deref(self).get_presets()
    }
    fn update_host_display(&self){
        ::std::ops::Deref::deref(self).update_host_display()
    }
}


/// Thread-safe state used for parameter and preset calls
pub struct SyncOnlyState {
    pub host: HostCallback,
    pub presets: OctaSinePresetBank,
}


/// Main structure
pub struct OctaSine {
    processing: ProcessingState,
    pub sync_only: Arc<SyncOnlyState>,
    editor: Option<Gui>,
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
    cfg_if::cfg_if! {
        // Simdeez only supports x86 intrinsics. Although code written with
        // simdeez provides a scalar fallback, it doesn't work due to my
        // implementation. So we only want to use the SIMD audio generation
        // with at least SSE2 present. It is provided by x86_64 by
        // specification, and since f64s are used in the code anyway, just
        // feature gate on that, as well as the "simd" feature flag.
        if #[cfg(all(feature = "simd", target_arch = "x86_64"))] {
            fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>){
                gen::simd::process_f32_runtime_select(self, buffer);
            }
        } else {
            fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>){
                gen::fallback::process_f32(self, buffer);
            }
        }
    }

    fn new(host: HostCallback) -> Self {
        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            global_time: TimeCounter(0.0),
            sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            rng: Rng::new(),
            log10_table: Log10Table::default(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: ProcessingParameters::default(),
        };

        let sync_only = Arc::new(SyncOnlyState {
            host,
            presets: built_in_preset_bank()
        });

        let editor = Gui::new(sync_only.clone());

        Self {
            processing,
            sync_only,
            editor: Some(editor),
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

        let log_config = simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

		let _ = simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            log_config,
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

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}


impl_plugin_parameters!(SyncOnlyState, presets);
