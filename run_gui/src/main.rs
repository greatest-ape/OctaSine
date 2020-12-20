use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings, WindowScalePolicy};
use octasine::{GuiSyncHandle, OctaSinePresetBank};
use octasine::constants::PLUGIN_NAME;
use octasine::gui::{GUI_WIDTH, GUI_HEIGHT};
use octasine::gui::interface::OctaSineIcedApplication;
use simplelog::{ConfigBuilder, SimpleLogger, LevelFilter};


struct SyncState {
    pub presets: OctaSinePresetBank,
}


impl GuiSyncHandle for SyncState {
    fn get_bank(&self) -> &OctaSinePresetBank {
        &self.presets
    }
    fn set_parameter(&self, index: usize, value: f64){
        self.presets.set_parameter_from_gui(index, value);
    }
    fn get_parameter(&self, index: usize) -> f64 {
        self.presets.get_current_preset()
            .get_parameter_value(index)
            .unwrap() // FIXME: unwrap
    }
    fn format_parameter_value(&self, index: usize, value: f64) -> String {
        self.presets.get_current_preset()
            .format_parameter_value(index, value)
            .unwrap() // FIXME: unwrap
    }
    fn update_host_display(&self){

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
        presets: OctaSinePresetBank::default(),
    });

    // Set envelope data for easier testing
    GuiSyncHandle::set_parameter(&sync_state, 10, 1.0 / 16.0);
    GuiSyncHandle::set_parameter(&sync_state, 12, 1.0 / 64.0);
    GuiSyncHandle::set_parameter(&sync_state, 13, 0.7);

    // Operator 4 additive
    GuiSyncHandle::set_parameter(&sync_state, 47, 0.7);

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