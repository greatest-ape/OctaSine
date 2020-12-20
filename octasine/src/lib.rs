#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

pub mod approximations;
pub mod common;
pub mod constants;
pub mod gen;
pub mod gui;
pub mod voices;
pub mod parameters;
pub mod preset_bank;

use std::sync::Arc;
use std::ops::Deref;

use array_init::array_init;
use fastrand::Rng;

use vst::api::{Supported, Events};
use vst::editor::Editor;
use vst::event::Event;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};
use vst::host::Host;

use approximations::*;
use common::*;
use constants::*;
use gui::Gui;
use voices::*;
use parameters::processing::*;


pub type OctaSinePresetBank = preset_bank::PresetBank;


// pub fn built_in_preset_bank<P>() -> PresetBank<P> where P: PresetParameters {
//     PresetBank::new_from_bytes(include_bytes!("../presets/preset-bank.json"))
// }


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


/// Thread-safe state used for parameter and preset calls
pub struct SyncOnlyState {
    pub host: HostCallback,
    pub presets: OctaSinePresetBank,
}


/// Trait passed to GUI code for encapsulation
pub trait GuiSyncHandle: Send + Sync + 'static {
    fn get_bank(&self) -> &OctaSinePresetBank;
    fn set_parameter(&self, index: usize, value: f64);
    fn get_parameter(&self, index: usize) -> f64;
    fn format_parameter_value(&self, index: usize, value: f64) -> String;
    fn update_host_display(&self);
}


impl GuiSyncHandle for SyncOnlyState {
    fn get_bank(&self) -> &OctaSinePresetBank {
        &self.presets
    }
    fn set_parameter(&self, index: usize, value: f64){
        self.presets.set_parameter_from_gui(index, value);
    }
    fn get_parameter(&self, index: usize) -> f64 {
        self.presets.get_parameter_value(index).unwrap() // FIXME: unwrap
    }
    fn format_parameter_value(&self, index: usize, value: f64) -> String {
        self.presets.format_parameter_value(index, value).unwrap() // FIXME: unwrap
    }
    fn update_host_display(&self){
        self.host.update_display();
    }
}


impl <H: GuiSyncHandle>GuiSyncHandle for Arc<H> {
    fn get_bank(&self) -> &OctaSinePresetBank {
        Deref::deref(self).get_bank()
    }
    fn set_parameter(&self, index: usize, value: f64){
        Deref::deref(self).set_parameter(index, value)
    }
    fn get_parameter(&self, index: usize) -> f64 {
        Deref::deref(self).get_parameter(index)
    }
    fn format_parameter_value(&self, index: usize, value: f64) -> String {
        Deref::deref(self).format_parameter_value(index, value)
    }
    fn update_host_display(&self){
        Deref::deref(self).update_host_display()
    }
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
            presets: OctaSinePresetBank::new(parameters::preset::create_parameters), // built_in_preset_bank()
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
            presets: self.sync_only.presets.num_presets() as i32,
            parameters: self.sync_only.presets.num_parameters() as i32,
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


impl vst::plugin::PluginParameters for SyncOnlyState {
    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, index: i32) -> String {
        self.presets.get_parameter_unit(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.presets.get_parameter_value_text(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.presets.get_parameter_name(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.presets.get_parameter_value(index as usize)
            .unwrap_or(0.0) as f32
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        self.presets.set_parameter_from_host(index as usize, value as f64);
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        self.presets.set_parameter_text_from_host(index as usize, text)
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.presets.num_parameters() < index as usize
    }

    /// Set the current preset to the index specified by `preset`.
    ///
    /// This method can be called on the processing thread for automation.
    fn change_preset(&self, index: i32) {
        self.presets.set_preset_index(index as usize);
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.presets.get_preset_index() as i32
    }

    /// Set the current preset name.
    fn set_preset_name(&self, name: String) {
        self.presets.set_preset_name(name);
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, index: i32) -> String {
        self.presets.get_preset_name(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.presets.export_current_preset_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.presets.export_bank_as_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
    /// chunk data.
    fn load_preset_data(&self, data: &[u8]) {
        self.presets.import_bytes_into_current_preset(data);
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
    /// given chunk data.
    fn load_bank_data(&self, data: &[u8]) {
        if let Err(err) = self.presets.import_bank_from_bytes(data){
            #[cfg(feature = "logging")]
            ::log::error!("Couldn't load bank data: {}", err)
        }
    }
}


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


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format(){
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}