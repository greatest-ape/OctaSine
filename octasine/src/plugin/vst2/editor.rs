use iced_baseview::{open_blocking, open_parented};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::{
    gui::{get_iced_baseview_settings, GUI_HEIGHT, GUI_WIDTH},
    plugin::vst2::PLUGIN_SEMVER_NAME,
    sync::GuiSyncHandle,
};

use crate::gui::OctaSineIcedApplication;

pub struct Editor<H: GuiSyncHandle> {
    sync_state: H,
    opened: bool,
}

impl<H: GuiSyncHandle> Editor<H> {
    pub fn new(sync_state: H) -> Self {
        Self {
            sync_state,
            opened: false,
        }
    }

    pub fn open_parented(parent: ParentWindow, sync_handle: H) {
        open_parented::<OctaSineIcedApplication<H>, ParentWindow>(
            &parent,
            get_iced_baseview_settings(sync_handle, PLUGIN_SEMVER_NAME.to_string()),
        );
    }

    pub fn open_blocking(sync_handle: H) {
        open_blocking::<OctaSineIcedApplication<H>>(get_iced_baseview_settings(
            sync_handle,
            PLUGIN_SEMVER_NAME.to_string(),
        ));
    }
}

impl<H: GuiSyncHandle> vst::editor::Editor for Editor<H> {
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

        Self::open_parented(ParentWindow(parent), self.sync_state.clone());

        true
    }

    fn close(&mut self) {
        self.opened = false;
    }

    fn is_open(&mut self) -> bool {
        self.opened
    }
}

pub struct ParentWindow(pub *mut ::core::ffi::c_void);

unsafe impl HasRawWindowHandle for ParentWindow {
    #[cfg(target_os = "macos")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::AppKitHandle::empty();

        handle.ns_view = self.0;

        RawWindowHandle::AppKit(handle)
    }

    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::Win32Handle::empty();

        handle.hwnd = self.0;

        RawWindowHandle::Win32(handle)
    }

    #[cfg(target_os = "linux")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::XcbHandle::empty();

        handle.window = self.0 as u32;

        RawWindowHandle::Xcb(handle)
    }
}
