use std::sync::Arc;

use baseview::{AppRunner, Parent, Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{Runner, Settings};
use vst::editor::Editor;
use raw_window_handle::RawWindowHandle;

use super::SyncState;
use crate::constants::PLUGIN_NAME;

pub mod interface;

use interface::OctaSineIcedApplication;


pub const GUI_WIDTH: usize = 14 * 70;
pub const GUI_HEIGHT: usize = 14 * 61;


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

    pub fn open_app_window(
        parent: Option<*mut ::core::ffi::c_void>,
        sync_state: Arc<SyncState>,
    ) -> Option<AppRunner> {
        let parent = if let Some(parent) = parent {
            Parent::WithParent(raw_window_handle_from_parent(parent))
        } else {
            Parent::None
        };

        let settings = Settings {
            window: WindowOpenOptions {
                parent,
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
                title: PLUGIN_NAME.to_string(),
            },
            flags: sync_state,
        };

        Runner::<OctaSineIcedApplication<_>>::open(settings).1
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

        Self::open_app_window(Some(parent), self.sync_state.clone());

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
