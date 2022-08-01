use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::get_file_storage_dir;

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
    fn get_config_file_path() -> anyhow::Result<PathBuf> {
        get_file_storage_dir().map(|path| path.join("OctaSine.json"))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let _ = ::std::fs::create_dir(&get_file_storage_dir()?); // Ignore creation errors

        let file = ::std::fs::File::create(Self::get_config_file_path()?)?;

        let _ = ::serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let file = ::std::fs::File::open(Self::get_config_file_path()?)?;

        let settings = ::serde_json::from_reader(file)?;

        Ok(settings)
    }

    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(settings) => settings,
            Err(err) => {
                ::log::warn!("Couldn't load settings: {}", err);

                Settings::default()
            }
        }
    }
}
