pub mod approximations;
pub mod common;
pub mod constants;
pub mod gen;
pub mod parameters;
pub mod preset_bank;
pub mod settings;
pub mod voices;

#[cfg(feature = "gui")]
pub mod gui;

use std::collections::VecDeque;
use std::sync::Arc;

use array_init::array_init;
use fastrand::Rng;

use gen::VoiceData;
use vst::api::{Events, Supported};
use vst::event::{Event, MidiEvent};
use vst::host::Host;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};

use common::*;
use constants::*;
use parameters::processing::*;
use preset_bank::PresetBank;
use settings::Settings;
use voices::*;

/// State used for processing
pub struct ProcessingState {
    pub sample_rate: SampleRate,
    pub time_per_sample: TimePerSample,
    pub rng: Rng,
    pub voices: [Voice; 128],
    pub parameters: ProcessingParameters,
    pub pending_midi_events: VecDeque<MidiEvent>,
    pub audio_gen_voice_data: [VoiceData; 128],
}

/// Thread-safe state used for parameter and preset calls
pub struct SyncState {
    /// Host should always be set when running as real plugin, but having the
    /// option of leaving this field empty is useful when benchmarking.
    pub host: Option<HostCallback>,
    pub presets: PresetBank,
    pub settings: Settings,
}

/// Main structure
pub struct OctaSine {
    processing: ProcessingState,
    pub sync: Arc<SyncState>,
    #[cfg(feature = "gui")]
    editor: Option<crate::gui::Gui<Arc<SyncState>>>,
}

impl Default for OctaSine {
    fn default() -> Self {
        Self::create(None)
    }
}

impl OctaSine {
    fn create(host: Option<HostCallback>) -> Self {
        // If initialization of logging fails, we can't do much about it, but
        // we shouldn't panic
        let _ = Self::init_logging();

        let settings = match Settings::load() {
            Ok(settings) => settings,
            Err(err) => {
                ::log::info!("Couldn't load settings: {}", err);

                Settings::default()
            }
        };

        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            rng: Rng::new(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: ProcessingParameters::default(),
            // Start with some capacity to cut down on later allocations
            pending_midi_events: VecDeque::with_capacity(128),
            audio_gen_voice_data: array_init::array_init(|_| VoiceData::default()),
        };

        let sync = Arc::new(SyncState {
            host,
            presets: built_in_preset_bank(),
            settings,
        });

        #[cfg(feature = "gui")]
        let editor = crate::gui::Gui::new(sync.clone());

        Self {
            processing,
            sync,
            #[cfg(feature = "gui")]
            editor: Some(editor),
        }
    }

    fn init_logging() -> anyhow::Result<()> {
        let log_folder = dirs::home_dir()
            .ok_or(anyhow::anyhow!("Couldn't extract home dir"))?
            .join("tmp");

        // Ignore any creation error
        let _ = ::std::fs::create_dir(log_folder.clone());

        let log_file = ::std::fs::File::create(log_folder.join(format!("{}.log", PLUGIN_NAME)))?;

        let log_config = simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

        simplelog::WriteLogger::init(simplelog::LevelFilter::Info, log_config, log_file)?;

        log_panics::init();

        ::log::info!("init");

        ::log::info!("OS: {}", ::os_info::get());
        ::log::info!("OctaSine build: {}", get_version_info());

        ::log::set_max_level(simplelog::LevelFilter::Error);

        Ok(())
    }

    fn time_per_sample(sample_rate: SampleRate) -> TimePerSample {
        TimePerSample(1.0 / sample_rate.0)
    }

    fn get_bpm(&self) -> BeatsPerMinute {
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        self.sync
            .host
            .and_then(|host| host.get_time_info(1 << 10))
            .map(|time_info| BeatsPerMinute(time_info.tempo as f64))
            .unwrap_or_default()
    }

    pub fn enqueue_midi_events<I: Iterator<Item = MidiEvent>>(&mut self, events: I) {
        for event in events {
            self.processing.pending_midi_events.push_back(event);
        }

        self.processing
            .pending_midi_events
            .make_contiguous()
            .sort_by_key(|e| e.delta_frames);
    }

    fn process_midi_event(&mut self, event: MidiEvent) {
        match event.data[0] {
            128 => self.key_off(event.data[1]),
            144 => self.key_on(event.data[1], event.data[2]),
            _ => (),
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
        self.processing.voices[pitch as usize].press_key(velocity);
    }

    fn key_off(&mut self, pitch: u8) {
        self.processing.voices[pitch as usize].release_key();
    }

    pub fn update_processing_parameters(&mut self) {
        let changed_sync_parameters = self.sync.presets.get_changed_parameters_from_processing();

        if let Some(indeces) = changed_sync_parameters {
            for (index, opt_new_value) in indeces.iter().enumerate() {
                if let Some(new_value) = opt_new_value {
                    self.processing.parameters.set_from_sync(index, *new_value);
                }
            }
        }
    }
}

impl Plugin for OctaSine {
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        gen::process_f32_runtime_select(self, buffer);
    }

    fn new(host: HostCallback) -> Self {
        Self::create(Some(host))
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
            presets: self.sync.presets.num_presets() as i32,
            parameters: self.sync.presets.num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: false,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        self.enqueue_midi_events(events.events().filter_map(|event| {
            if let Event::Midi(event) = event {
                Some(event)
            } else {
                None
            }
        }))
    }

    fn set_sample_rate(&mut self, rate: f32) {
        let sample_rate = SampleRate(f64::from(rate));

        self.processing.sample_rate = sample_rate;
        self.processing.time_per_sample = Self::time_per_sample(sample_rate);
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent
            | CanDo::ReceiveTimeInfo
            | CanDo::SendEvents
            | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.sync) as Arc<dyn PluginParameters>
    }

    #[cfg(feature = "gui")]
    fn get_editor(&mut self) -> Option<Box<dyn ::vst::editor::Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn ::vst::editor::Editor>)
        } else {
            None
        }
    }
}

impl vst::plugin::PluginParameters for SyncState {
    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, _: i32) -> String {
        "".to_string()
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.presets
            .get_parameter_value_text(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.presets
            .get_parameter_name(index as usize)
            .unwrap_or_else(|| "".to_string())
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.presets
            .get_parameter_value(index as usize)
            .unwrap_or(0.0) as f32
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        self.presets
            .set_parameter_from_host(index as usize, value as f64);
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        self.presets
            .set_parameter_text_from_host(index as usize, text)
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
        self.presets
            .get_preset_name(index as usize)
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
        if let Err(err) = self.presets.import_bank_from_bytes(data) {
            ::log::error!("Couldn't load bank data: {}", err)
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "gui")] {
        use preset_bank::MAX_NUM_PARAMETERS;

        /// Trait passed to GUI code for encapsulation
        pub trait GuiSyncHandle: Clone + Send + Sync + 'static {
            fn set_parameter(&self, index: usize, value: f64);
            fn get_parameter(&self, index: usize) -> f64;
            fn format_parameter_value(&self, index: usize, value: f64) -> String;
            fn get_presets(&self) -> (usize, Vec<String>);
            fn set_preset_index(&self, index: usize);
            fn get_changed_parameters(&self) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]>;
            fn have_presets_changed(&self) -> bool;
            fn get_gui_settings(&self) -> gui::GuiSettings;
        }

        impl GuiSyncHandle for Arc<SyncState> {
            fn set_parameter(&self, index: usize, value: f64){
                if let Some(host) = self.host {
                    // Host will occasionally set the value again, but that's
                    // ok
                    host.automate(index as i32, value as f32);
                }

                self.presets.set_parameter_from_gui(index, value);
            }
            fn get_parameter(&self, index: usize) -> f64 {
                self.presets.get_parameter_value(index).unwrap() // FIXME: unwrap
            }
            fn format_parameter_value(&self, index: usize, value: f64) -> String {
                self.presets.format_parameter_value(index, value).unwrap() // FIXME: unwrap
            }
            fn get_presets(&self) -> (usize, Vec<String>){
                let index = self.presets.get_preset_index();
                let names = self.presets.get_preset_names();

                (index, names)
            }
            fn set_preset_index(&self, index: usize){
                self.presets.set_preset_index(index);

                if let Some(host) = self.host {
                    host.update_display();
                }
            }
            fn get_changed_parameters(&self) -> Option<[Option<f64>; MAX_NUM_PARAMETERS]> {
                self.presets.get_changed_parameters_from_gui()
            }
            fn have_presets_changed(&self) -> bool {
                self.presets.have_presets_changed()
            }
            fn get_gui_settings(&self) -> gui::GuiSettings {
                self.settings.gui.clone()
            }
        }
    }
}

pub fn built_in_preset_bank() -> PresetBank {
    PresetBank::new_from_bytes(include_bytes!("../presets/preset-bank.json"))
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

fn get_version_info() -> String {
    use git_testament::{git_testament, CommitKind};

    let mut info = format!("v{}", env!("CARGO_PKG_VERSION"));

    git_testament!(GIT_TESTAMENT);

    match GIT_TESTAMENT.commit {
        CommitKind::NoTags(commit, _) | CommitKind::FromTag(_, commit, _, _) => {
            let commit = commit.chars().take(7).collect::<String>();

            info.push_str(&format!(" ({})", commit));
        }
        _ => (),
    };

    if !GIT_TESTAMENT.modifications.is_empty() {
        info.push_str(" (M)");
    }

    #[cfg(feature = "gui_wgpu")]
    info.push_str(" (wgpu)");

    #[cfg(feature = "gui_glow")]
    info.push_str(" (gl)");

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format() {
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}
