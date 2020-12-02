use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings};
use octasine::{SyncOnlyState, built_in_preset_bank, gui::{GUI_WIDTH, GUI_HEIGHT}};
use vst::plugin::HostCallback;

use octasine::gui::interface::{self, OctaSineIcedApplication};


fn main(){
    let sync_only = Arc::new(SyncOnlyState {
        host: HostCallback::default(), // FIXME: crashes when accessed
        presets: built_in_preset_bank(),
    });

    let settings = Settings {
        window: settings::Window {
            size: (GUI_WIDTH as u32, GUI_HEIGHT as u32),
        },
        flags: sync_only,
    };

    let runner = Runner::<OctaSineIcedApplication>::open(
        settings,
        Parent::None,
        Some(interface::Message::Frame)
    );

    runner.app_run_blocking();
}