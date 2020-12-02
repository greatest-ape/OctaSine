use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings};
use octasine::{SyncHandle, OctaSinePresetBank, built_in_preset_bank};
use octasine::gui::{GUI_WIDTH, GUI_HEIGHT};
use octasine::gui::interface::{self, OctaSineIcedApplication};


struct SyncState {
    pub presets: OctaSinePresetBank,
}


impl SyncHandle for SyncState {
    fn get_presets(&self) -> &OctaSinePresetBank {
        &self.presets
    }

    fn update_host_display(&self){

    }
}


fn main(){
    let sync_state = Arc::new(SyncState {
        presets: built_in_preset_bank(),
    });

    let settings = Settings {
        window: settings::Window {
            size: (GUI_WIDTH as u32, GUI_HEIGHT as u32),
        },
        flags: sync_state,
    };

    let runner = Runner::<OctaSineIcedApplication<SyncState>>::open(
        settings,
        Parent::None,
        Some(interface::Message::Frame)
    );

    runner.app_run_blocking();
}