use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::get_project_dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub schema_version: usize,
    #[cfg(feature = "gui")]
    pub gui: super::gui::GuiSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            #[cfg(feature = "gui")]
            gui: Default::default(),
        }
    }
}

impl Settings {
    fn get_config_dir() -> anyhow::Result<PathBuf> {
        get_project_dirs()
            .ok_or_else(|| anyhow::anyhow!("Could not extract project directory"))
            .map(|dirs| dirs.preference_dir().into())
    }

    fn get_config_file_path() -> anyhow::Result<PathBuf> {
        Self::get_config_dir().map(|path| path.join("OctaSine.json"))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = Self::get_config_dir()?;

        let _ = ::std::fs::create_dir(&config_dir); // Ignore creation errors

        let file = ::std::fs::File::create(config_dir.join("OctaSine.json"))?;

        let _ = ::serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let path = Self::get_config_file_path()?;
        let file = ::std::fs::File::open(path)?;

        let settings = ::serde_json::from_reader(file)?;

        Ok(settings)
    }

    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(settings) => settings,
            Err(err) => {
                ::log::info!("Couldn't load settings: {}", err);

                Settings::default()
            }
        }
    }
}
