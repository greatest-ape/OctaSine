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
use std::path::PathBuf;
use std::sync::Arc;

use approximations::Log10Table;
use array_init::array_init;
use directories::ProjectDirs;
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

pub struct OctaSine {
    pub processing: ProcessingState,
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
        let _ = init_logging();

        let settings = match Settings::load() {
            Ok(settings) => settings,
            Err(err) => {
                ::log::info!("Couldn't load settings: {}", err);

                Settings::default()
            }
        };

        let sync = Arc::new(SyncState {
            host,
            presets: built_in_preset_bank(),
            settings,
        });

        #[cfg(feature = "gui")]
        let editor = crate::gui::Gui::new(sync.clone());

        Self {
            processing: Default::default(),
            sync,
            #[cfg(feature = "gui")]
            editor: Some(editor),
        }
    }

    fn update_bpm(&mut self) {
        if let Some(bpm) = self.sync.get_bpm_from_host() {
            self.processing.bpm = bpm;
        }
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
        self.processing
            .enqueue_midi_events(events.events().filter_map(|event| {
                if let Event::Midi(event) = event {
                    Some(event)
                } else {
                    None
                }
            }))
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.processing.time_per_sample = SampleRate(f64::from(rate)).into();
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

pub struct ProcessingState {
    pub time_per_sample: TimePerSample,
    pub bpm: BeatsPerMinute,
    pub rng: Rng,
    pub log10table: Log10Table,
    pub voices: [Voice; 128],
    pub parameters: ProcessingParameters,
    pub pending_midi_events: VecDeque<MidiEvent>,
    pub audio_gen_voice_data: [VoiceData; 128],
}

impl Default for ProcessingState {
    fn default() -> Self {
        Self {
            time_per_sample: SampleRate::default().into(),
            bpm: Default::default(),
            rng: Rng::new(),
            log10table: Default::default(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: ProcessingParameters::default(),
            // Start with some capacity to cut down on later allocations
            pending_midi_events: VecDeque::with_capacity(128),
            audio_gen_voice_data: array_init::array_init(|_| VoiceData::default()),
        }
    }
}

impl ProcessingState {
    pub fn enqueue_midi_events<I: Iterator<Item = MidiEvent>>(&mut self, events: I) {
        for event in events {
            self.pending_midi_events.push_back(event);
        }

        self.pending_midi_events
            .make_contiguous()
            .sort_by_key(|e| e.delta_frames);
    }

    fn process_events_for_sample(&mut self, buffer_offset: usize) {
        loop {
            match self
                .pending_midi_events
                .get(0)
                .map(|e| e.delta_frames as usize)
            {
                Some(event_delta_frames) if event_delta_frames == buffer_offset => {
                    let event = self.pending_midi_events.pop_front().unwrap();

                    self.process_midi_event(event);
                }
                _ => break,
            }
        }
    }

    fn process_midi_event(&mut self, mut event: MidiEvent) {
        event.data[0] >>= 4;

        match event.data {
            [0b_1000, pitch, _] => self.key_off(pitch),
            [0b_1001, pitch, 0] => self.key_off(pitch),
            [0b_1001, pitch, velocity] => self.key_on(pitch, velocity),
            _ => (),
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
        self.voices[pitch as usize].press_key(velocity);
    }

    fn key_off(&mut self, pitch: u8) {
        self.voices[pitch as usize].release_key();
    }
}

/// Thread-safe state used for parameter and preset calls
pub struct SyncState {
    /// Host should always be set when running as real plugin, but having the
    /// option of leaving this field empty is useful when benchmarking.
    pub host: Option<HostCallback>,
    pub presets: PresetBank,
    pub settings: Settings,
}

impl SyncState {
    fn get_bpm_from_host(&self) -> Option<BeatsPerMinute> {
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        let mask = 1 << 10;

        let time_info = self.host?.get_time_info(mask)?;

        if (time_info.flags & mask) != 0 {
            Some(BeatsPerMinute(time_info.tempo as f64))
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
            fn begin_edit(&self, index: usize);
            fn end_edit(&self, index: usize);
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
            fn begin_edit(&self, index: usize) {
                if let Some(host) = self.host {
                    host.begin_edit(index as i32);
                }
            }
            fn end_edit(&self, index: usize) {
                if let Some(host) = self.host {
                    host.end_edit(index as i32);
                }
            }
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

fn init_logging() -> anyhow::Result<()> {
    let log_folder: PathBuf = get_project_dirs()
        .ok_or(anyhow::anyhow!("Couldn't extract home dir"))?
        .cache_dir()
        .into();

    // Ignore any creation error
    let _ = ::std::fs::create_dir(log_folder.clone());

    let log_file = ::std::fs::File::create(log_folder.join("OctaSine.log"))?;

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

pub fn built_in_preset_bank() -> PresetBank {
    PresetBank::default()
}

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION").to_string()
    };
}

fn crate_version_to_vst_format(crate_version: String) -> i32 {
    format!("{:0<4}", crate_version.replace('.', ""))
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

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "OctaSine", "OctaSine")
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
