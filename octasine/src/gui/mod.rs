use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{IcedWindow, Settings};
use vst::editor::Editor;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use super::GuiSyncHandle;
use crate::constants::PLUGIN_NAME;

mod interface;

use interface::OctaSineIcedApplication;


pub const GUI_WIDTH: usize = 14 * 66;
pub const GUI_HEIGHT: usize = 14 * 58;


pub struct Gui<H: GuiSyncHandle> {
    sync_state: H,
    opened: bool,
}


impl <H: GuiSyncHandle> Gui<H> {
    pub fn new(sync_state: H) -> Self {
        Self {
            sync_state,
            opened: false,
        }
    }

    fn get_iced_baseview_settings(
        sync_handle: H,
    ) -> Settings<H> {
        Settings {
            window: WindowOpenOptions {
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
                title: PLUGIN_NAME.to_string(),
            },
            flags: sync_handle
        }
    }

    pub fn open_parented(parent: ParentWindow, sync_handle: H){
        IcedWindow::<OctaSineIcedApplication<_>>::open_parented(
            &parent,
            Self::get_iced_baseview_settings(sync_handle)
        );
    }

    pub fn open_blocking(sync_handle: H){
        let settings = Self::get_iced_baseview_settings(sync_handle);

        IcedWindow::<OctaSineIcedApplication<_>>::open_blocking(settings);
    }
}


impl <H: GuiSyncHandle>Editor for Gui<H> {
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
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0,
            ..MacOSHandle::empty()
        })
    }

    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }

    #[cfg(target_os = "linux")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}
