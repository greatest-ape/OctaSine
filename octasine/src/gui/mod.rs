use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings, WindowScalePolicy};
use vst::editor::Editor;
use raw_window_handle::RawWindowHandle;

use super::SyncState;
use crate::constants::PLUGIN_NAME;

pub mod interface;

use interface::OctaSineIcedApplication;


pub const GUI_WIDTH: usize = 1000;
pub const GUI_HEIGHT: usize = 750;


pub struct Gui {
    sync_state: Arc<SyncState>,
    opened: bool,
}


impl Gui {
    pub fn new(sync_state: Arc<SyncState>) -> Self {
        Self {
            sync_state,
            opened: false,
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
        if self.opened {
            return false;
        }

        let settings = Settings {
            window: settings::Window {
                logical_size: (GUI_WIDTH as u32, GUI_HEIGHT as u32),
                scale_policy: WindowScalePolicy::SystemScaleFactor,
                title: PLUGIN_NAME.to_string(),
            },
            flags: self.sync_state.clone(),
        };

        Runner::<OctaSineIcedApplication<Arc<SyncState>>>::open(
            settings,
            Parent::WithParent(raw_window_handle_from_parent(parent)),
        );

        true
    }

    fn close(&mut self) {
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
