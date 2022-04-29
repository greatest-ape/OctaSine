use baseview::{gl::GlConfig, Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{IcedWindow, Settings};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use serde::{Deserialize, Serialize};
use vst::editor::Editor;

use crate::sync::GuiSyncHandle;
use crate::PLUGIN_NAME;

mod interface;

use interface::OctaSineIcedApplication;

pub const GUI_WIDTH: usize = 12 * 78;
pub const GUI_HEIGHT: usize = 12 * 61;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

pub struct GuiSettings {
    pub theme: interface::style::Theme,
}

pub struct Gui<H: GuiSyncHandle> {
    sync_state: H,
    opened: bool,
}

impl<H: GuiSyncHandle> Gui<H> {
    pub fn new(sync_state: H) -> Self {
        Self {
            sync_state,
            opened: false,
        }
    }

    fn get_iced_baseview_settings(sync_handle: H) -> Settings<H> {
        Settings {
            window: WindowOpenOptions {
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                #[cfg(not(target_os = "windows"))]
                scale: WindowScalePolicy::SystemScaleFactor,
                // Windows currently needs scale factor 1.0, or GUI contents
                // will be too large for window
                #[cfg(target_os = "windows")]
                scale: WindowScalePolicy::ScaleFactor(1.0),
                title: PLUGIN_NAME.to_string(),
                gl_config: Some(GlConfig {
                    samples: Some(8),
                    ..Default::default()
                }),
            },
            iced_baseview: iced_baseview::IcedBaseviewSettings {
                ignore_non_modifier_keys: true,
                always_redraw: true,
            },
            flags: sync_handle,
        }
    }

    pub fn open_parented(parent: ParentWindow, sync_handle: H) {
        IcedWindow::<OctaSineIcedApplication<_>>::open_parented(
            &parent,
            Self::get_iced_baseview_settings(sync_handle),
        );
    }

    pub fn open_blocking(sync_handle: H) {
        let settings = Self::get_iced_baseview_settings(sync_handle);

        IcedWindow::<OctaSineIcedApplication<_>>::open_blocking(settings);
    }
}

impl<H: GuiSyncHandle> Editor for Gui<H> {
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
