// Interesting links:
// - https://github.com/hecrj/iced/blob/0.1/examples/integration/src/main.rs

use std::sync::Arc;

use baseview::{Parent, Size, WindowScalePolicy, Window, WindowOpenOptions};
use vst::editor::Editor;
use raw_window_handle::RawWindowHandle;

use super::SyncOnlyState;

mod bridge;
mod interface;

use bridge::Handler;
use interface::Application;


const GUI_WIDTH: usize = 1000;
const GUI_HEIGHT: usize = 750;


pub struct Gui {
    sync_only: Arc<SyncOnlyState>,
    opened: bool,
    // opened_interface: Option<Handler<Application>>,
}


impl Gui {
    pub fn new(sync_only: Arc<SyncOnlyState>) -> Self {
        Self {
            sync_only,
            opened: true,
            // opened_interface: None,
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
        if !self.opened{
            let window_options = WindowOpenOptions {
                scale: WindowScalePolicy::SystemScaleFactor,
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                parent: Parent::WithParent(raw_window_handle_from_parent(parent)),
                title: crate::constants::PLUGIN_NAME.into(),
            };

            let _ = Window::open( window_options, |window| {
                Handler::<Application>::build(window, GUI_WIDTH as u32, GUI_HEIGHT as u32)
            });
            
            true
        } else {
            false
        }
    }

    fn close(&mut self) {
        // FIXME
        self.opened = false;
    }

    fn is_open(&mut self) -> bool {
        self.opened
    }
}


#[cfg(target_os = "macos")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::macos::MacOSHandle;

    RawWindowHandle::MacOS(MacOSHandle {
        ns_view: parent,
        ..MacOSHandle::empty()
    })
}


#[cfg(target_os = "windows")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::windows::WindowsHandle;

    RawWindowHandle::Windows(WindowsHandle {
        hwnd: parent,
        ..WindowsHandle::empty()
    })
}


#[cfg(target_os = "linux")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::unix::XcbHandle;

    RawWindowHandle::Xcb(XcbHandle {
        window: parent as u32,
        ..XcbHandle::empty()
    })
}