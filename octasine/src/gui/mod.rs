// Interesting links:
// - https://github.com/hecrj/iced/blob/0.1/examples/integration/src/main.rs

use std::sync::Arc;

use vst::editor::Editor;

use super::SyncOnlyState;

mod bridge;
mod interface;

use bridge::Handler;
use interface::Application;


const GUI_WIDTH: usize = 1000;
const GUI_HEIGHT: usize = 750;


pub struct Gui {
    sync_only: Arc<SyncOnlyState>,
    opened_interface: Option<Handler<Application>>,
}


impl Gui {
    pub fn new(sync_only: Arc<SyncOnlyState>) -> Self {
        Self {
            sync_only,
            opened_interface: None,
        }
    }
}


impl Editor for Gui {
    fn size(&self) -> (i32, i32) {
        (GUI_WIDTH as i32, GUI_HEIGHT as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut ::core::ffi::c_void) -> bool {
        if self.opened_interface.is_none(){
            let (window, event_source) = vst_window::setup(
                parent,
                self.size()
            );

            let interface = Handler::build(
                window,
                event_source,
                GUI_WIDTH as u32,
                GUI_HEIGHT as u32
            );
            
            self.opened_interface = Some(interface);

            true
        } else {
            false
        }
    }

    fn close(&mut self) {
        self.opened_interface = None;
    }

    fn is_open(&mut self) -> bool {
        self.opened_interface.is_some()
    }

    fn idle(&mut self) {
        if let Some(interface) = self.opened_interface.as_mut() {
            interface.process_events();
        }
    }

    #[cfg(feature = "logging")]
    fn key_down(&mut self, keycode: vst::editor::KeyCode) -> bool {
        ::log::info!("key down: {:?}", keycode);

        true
    }

    #[cfg(feature = "logging")]
    fn key_up(&mut self, keycode: vst::editor::KeyCode) -> bool {
        ::log::info!("key up: {:?}", keycode);

        true
    }
}
