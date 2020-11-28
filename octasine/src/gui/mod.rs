use std::sync::Arc;

use iced_baseview::{settings, Parent, Runner, Settings};
use vst::editor::Editor;
use raw_window_handle::RawWindowHandle;

use super::SyncOnlyState;

mod interface;

use interface::OctaSineIcedApplication;


const GUI_WIDTH: usize = 1000;
const GUI_HEIGHT: usize = 750;


pub struct Gui {
    sync_only: Arc<SyncOnlyState>,
    opened: bool,
}


impl Gui {
    pub fn new(sync_only: Arc<SyncOnlyState>) -> Self {
        Self {
            sync_only,
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
                size: (GUI_WIDTH as u32, GUI_HEIGHT as u32),
            },
            flags: self.sync_only.clone(),
        };

        Runner::<OctaSineIcedApplication>::open(
            settings,
            Parent::WithParent(raw_window_handle_from_parent(parent)
        ));

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