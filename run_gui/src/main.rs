use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings, WindowScalePolicy};
use octasine::{SyncHandle, OctaSinePresetBank, built_in_preset_bank};
use octasine::constants::PLUGIN_NAME;
use octasine::gui::{GUI_WIDTH, GUI_HEIGHT};
use octasine::gui::interface::OctaSineIcedApplication;
use simplelog::{ConfigBuilder, SimpleLogger, LevelFilter};


struct SyncState {
    pub presets: OctaSinePresetBank,
}


impl SyncHandle for SyncState {
    fn get_presets(&self) -> &OctaSinePresetBank {
        &self.presets
    }

    fn update_host_display(&self) {

    }
}


fn main(){
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_to_local(true)
            .build()
    ).unwrap();

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
            scale_policy: WindowScalePolicy::SystemScaleFactor,
            title: PLUGIN_NAME.to_string(),
        },
        flags: sync_state.clone(),
    };

    let (_, opt_runner) = Runner::<OctaSineIcedApplication<Arc<SyncState>>>::open(
        settings,
        Parent::None,
    );

    opt_runner.unwrap().app_run_blocking();
}