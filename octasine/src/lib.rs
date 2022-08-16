pub mod audio;
pub mod common;
mod math;
pub mod parameters;
pub mod settings;
pub mod simd;
pub mod sync;

#[cfg(feature = "gui")]
pub mod gui;

use std::path::PathBuf;
use std::sync::Arc;

use audio::AudioState;

use parameters::Parameter;
use sync::SyncState;
use vst::api::{Events, Supported};
use vst::event::Event;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};

use common::*;

pub const PLUGIN_NAME: &str = "OctaSine v0.7";
pub const PLUGIN_UNIQUE_ID: i32 = 1_438_048_624;

pub struct OctaSine {
    pub audio: AudioState,
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

        let sync = Arc::new(SyncState::new(host));

        #[cfg(feature = "gui")]
        let editor = crate::gui::Gui::new(sync.clone());

        Self {
            audio: Default::default(),
            sync,
            #[cfg(feature = "gui")]
            editor: Some(editor),
        }
    }

    fn update_bpm(&mut self) {
        if let Some(bpm) = self.sync.get_bpm_from_host() {
            self.audio.set_bpm(bpm);
        }
    }

    pub fn update_audio_parameters(&mut self) {
        if let Some(indeces) = self.sync.patches.get_changed_parameters_from_audio() {
            for (index, opt_new_value) in indeces.iter().enumerate() {
                if let Some(new_value) = opt_new_value {
                    if let Some(parameter) = Parameter::from_index(index) {
                        self.audio.set_parameter_from_patch(parameter, *new_value);
                    }
                }
            }
        }
    }
}

impl Plugin for OctaSine {
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        self.update_audio_parameters();
        self.update_bpm();

        audio::gen::process_f32_runtime_select(&mut self.audio, buffer);
    }

    fn new(host: HostCallback) -> Self {
        Self::create(Some(host))
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: "Joakim Frostegard".to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: PLUGIN_UNIQUE_ID,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            presets: self.sync.patches.num_patches() as i32,
            parameters: self.sync.patches.num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: false,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        self.audio
            .enqueue_midi_events(events.events().filter_map(|event| {
                if let Event::Midi(event) = event {
                    Some(event)
                } else {
                    None
                }
            }))
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.audio.set_sample_rate(SampleRate(f64::from(rate)));
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

fn init_logging() -> anyhow::Result<()> {
    let log_folder: PathBuf = get_file_storage_dir()?;

    // Ignore any creation error
    let _ = ::std::fs::create_dir(log_folder.clone());

    let log_file = ::std::fs::File::create(log_folder.join("OctaSine.log"))?;

    let log_config = match simplelog::ConfigBuilder::new().set_time_offset_to_local() {
        Ok(builder) => builder.build(),
        Err(builder) => builder.build(),
    };

    simplelog::WriteLogger::init(simplelog::LevelFilter::Info, log_config, log_file)?;

    log_panics::init();

    ::log::info!("init");

    ::log::info!("OS: {}", ::os_info::get());
    ::log::info!("OctaSine build: {}", get_version_info());

    ::log::set_max_level(simplelog::LevelFilter::Error);

    Ok(())
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

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn get_file_storage_dir() -> anyhow::Result<PathBuf> {
            ::directories::UserDirs::new()
                .and_then(|d| d.document_dir().map(|d| d.join("OctaSine")))
                .ok_or(anyhow::anyhow!("Couldn't extract file storage dir"))
        }
    } else {
        fn get_file_storage_dir() -> anyhow::Result<PathBuf> {
            ::directories::ProjectDirs::from("com", "OctaSine", "OctaSine")
                .map(|d| d.config_dir().to_owned())
                .ok_or(anyhow::anyhow!("Couldn't extract file storage dir"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::PARAMETERS;

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

    #[test]
    fn test_parameter_interaction() {
        let mut octasine = OctaSine::default();
        let mut patch_values = Vec::new();

        for i in 0..PARAMETERS.len() {
            let patch_value = fastrand::f32();

            octasine
                .sync
                .patches
                .set_parameter_from_host(i, patch_value);

            patch_values.push(patch_value)
        }

        octasine.update_audio_parameters();

        let sample_rate = SampleRate(44100.0);

        for _ in 0..44100 {
            octasine.audio.advance_one_sample(sample_rate);
        }

        for (i, parameter) in PARAMETERS.iter().copied().enumerate() {
            assert_eq!(i, parameter.to_index() as usize);

            let values_approx_eq = octasine
                .audio
                .compare_parameter_patch_value(parameter, patch_values[i]);

            if !values_approx_eq {
                println!("Parameter: {:?}", parameter);
                println!("Set patch value: {}", patch_values[i]);
            }

            assert!(values_approx_eq)
        }
    }
}
