use std::sync::Arc;

use octasine::{built_in_preset_bank, gui::Gui, GuiSyncHandle, SyncState, settings::Settings};
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

    // Set envelope data for easier testing
    GuiSyncHandle::set_parameter(&sync_state, 10, 1.0);
    GuiSyncHandle::set_parameter(&sync_state, 12, 1.0);
    GuiSyncHandle::set_parameter(&sync_state, 14, 1.0);

    // Operator 4 additive
    GuiSyncHandle::set_parameter(&sync_state, 47, 0.7);

    // Feedback
    GuiSyncHandle::set_parameter(&sync_state, 6, 1.0);
    GuiSyncHandle::set_parameter(&sync_state, 20, 0.9);

    Gui::open_blocking(sync_state);
}
