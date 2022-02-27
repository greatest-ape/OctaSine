use std::sync::Arc;

use octasine::{built_in_preset_bank, gui::Gui, settings::Settings, GuiSyncHandle, SyncState};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().set_time_to_local(true).build(),
    )
    .unwrap();

    let sync_state = Arc::new(SyncState {
        presets: built_in_preset_bank(),
        host: None,
        settings: Settings::load().unwrap_or_default(),
    });

    Gui::open_blocking(sync_state);
}
