use std::path::PathBuf;

use crate::{audio::AudioState, parameters::Parameter, sync::SyncState};

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

pub fn update_audio_parameters<T>(audio: &mut AudioState, sync: &SyncState<T>) {
    if let Some(indeces) = sync.patches.get_changed_parameters_from_audio() {
        for (index, opt_new_value) in indeces.iter().enumerate() {
            if let Some(new_value) = opt_new_value {
                if let Some(parameter) = Parameter::from_index(index) {
                    audio.set_parameter_from_patch(parameter, *new_value);
                }
            }
        }
    }
}

pub fn init_logging() -> anyhow::Result<()> {
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

pub fn get_version_info() -> String {
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

    #[cfg(feature = "wgpu")]
    info.push_str(" (wgpu)");

    #[cfg(feature = "glow")]
    info.push_str(" (gl)");

    info
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub fn get_file_storage_dir() -> anyhow::Result<PathBuf> {
            ::directories::UserDirs::new()
                .and_then(|d| d.document_dir().map(|d| d.join("OctaSine")))
                .ok_or(anyhow::anyhow!("Couldn't extract file storage dir"))
        }
    } else {
        pub fn get_file_storage_dir() -> anyhow::Result<PathBuf> {
            ::directories::ProjectDirs::from("com", "OctaSine", "OctaSine")
                .map(|d| d.config_dir().to_owned())
                .ok_or(anyhow::anyhow!("Couldn't extract file storage dir"))
        }
    }
}
