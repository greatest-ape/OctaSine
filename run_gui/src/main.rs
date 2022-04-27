use std::sync::Arc;

use octasine::{gui::Gui, settings::Settings, sync::SyncState};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new().set_time_to_local(true).build(),
    )
    .unwrap();

    let sync_state = Arc::new(SyncState::new(None, Settings::load().unwrap_or_default()));

    Gui::open_blocking(sync_state);
}
