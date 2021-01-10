use std::sync::Arc;

use baseview::{Parent, Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{Runner, Settings};
use octasine::{GuiSyncHandle, built_in_preset_bank, SyncState};
use octasine::constants::PLUGIN_NAME;
use octasine::gui::{GUI_WIDTH, GUI_HEIGHT};
use octasine::gui::interface::OctaSineIcedApplication;
use simplelog::{ConfigBuilder, SimpleLogger, LevelFilter};


fn main(){
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_to_local(true)
            .build()
    ).unwrap();

    let sync_state = Arc::new(SyncState {
        presets: built_in_preset_bank(),
        host: None,
    });

    // Set envelope data for easier testing
    GuiSyncHandle::set_parameter(&sync_state, 10, 1.0 / 16.0);
    GuiSyncHandle::set_parameter(&sync_state, 12, 1.0 / 64.0);
    GuiSyncHandle::set_parameter(&sync_state, 13, 0.7);

    // Operator 4 additive
    GuiSyncHandle::set_parameter(&sync_state, 47, 0.7);

    // Feedback
    GuiSyncHandle::set_parameter(&sync_state, 6, 1.0);
    GuiSyncHandle::set_parameter(&sync_state, 20, 0.9);

    let settings = Settings {
        window: WindowOpenOptions {
            parent: Parent::None,
            size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
            scale: WindowScalePolicy::SystemScaleFactor,
            title: PLUGIN_NAME.to_string(),
        },
        flags: sync_state.clone(),
    };

    Runner::<OctaSineIcedApplication<_>>::open(settings)
        .1
        .unwrap()
        .app_run_blocking();
}