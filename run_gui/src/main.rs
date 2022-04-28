use std::sync::Arc;

use octasine::{gui::Gui, sync::SyncState};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_offset_to_local()
            .unwrap()
            .build(),
    )
    .unwrap();

    let sync_state = Arc::new(SyncState::new(None));

    Gui::open_blocking(sync_state);
}
