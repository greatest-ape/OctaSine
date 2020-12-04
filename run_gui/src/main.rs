use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings, WindowScalePolicy};
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

    // Set envelope data for easier testing
    sync_state.presets.set_parameter_value_float_from_gui(10, 1.0 / 16.0);
    sync_state.presets.set_parameter_value_float_from_gui(12, 1.0 / 64.0);
    sync_state.presets.set_parameter_value_float_from_gui(13, 0.7);

    let settings = Settings {
        window: settings::Window {
            logical_size: (GUI_WIDTH as u32, GUI_HEIGHT as u32),
            scale: WindowScalePolicy::SystemScaleFactor,
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