use std::{
    ffi::{c_char, CStr},
    sync::Arc,
};

use cfg_if::cfg_if;
use clap_sys::{
    ext::gui::{clap_gui_resize_hints, clap_plugin_gui, clap_window},
    plugin::clap_plugin,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::{
    gui::{get_iced_baseview_settings, OctaSineIcedApplication, GUI_HEIGHT, GUI_WIDTH},
    plugin::clap::{plugin::OctaSine, sync::ClapGuiSyncHandle},
    sync::SyncState,
};

cfg_if! {
    if #[cfg(target_os = "macos")] {
        const SUPPORTED_API: &CStr = clap_sys::ext::gui::CLAP_WINDOW_API_COCOA;
    } else if #[cfg(target_os = "windows")] {
        const SUPPORTED_API: &CStr = clap_sys::ext::gui::CLAP_WINDOW_API_WIN32;
    } else {
        const SUPPORTED_API: &CStr = clap_sys::ext::gui::CLAP_WINDOW_API_X11;
    }
}

unsafe extern "C" fn is_api_supported(
    _plugin: *const clap_plugin,
    api: *const c_char,
    is_floating: bool,
) -> bool {
    CStr::from_ptr(api) == SUPPORTED_API && !is_floating
}

unsafe extern "C" fn get_preferred_api(
    _plugin: *const clap_plugin,
    api: *mut *const c_char,
    is_floating: *mut bool,
) -> bool {
    *api = SUPPORTED_API.as_ptr();
    *is_floating = false;

    true
}

unsafe extern "C" fn create(
    _plugin: *const clap_plugin,
    api: *const c_char,
    is_floating: bool,
) -> bool {
    CStr::from_ptr(api) == SUPPORTED_API && !is_floating
}

unsafe extern "C" fn destroy(plugin: *const clap_plugin) {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if let Some(mut handle) = plugin.gui_window_handle.lock().take() {
        handle.close_window();
    }
}

extern "C" fn set_scale(_plugin: *const clap_plugin, _scale: f64) -> bool {
    false
}

unsafe extern "C" fn get_size(
    _plugin: *const clap_plugin,
    width: *mut u32,
    height: *mut u32,
) -> bool {
    *width = GUI_WIDTH as u32;
    *height = GUI_HEIGHT as u32;

    true
}

extern "C" fn can_resize(_plugin: *const clap_plugin) -> bool {
    false
}

extern "C" fn get_resize_hints(
    _plugin: *const clap_plugin,
    _hints: *mut clap_gui_resize_hints,
) -> bool {
    false
}

unsafe extern "C" fn adjust_size(
    _plugin: *const clap_plugin,
    _width: *mut u32,
    _height: *mut u32,
) -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
unsafe extern "C" fn set_size(_plugin: *const clap_plugin, _width: u32, _height: u32) -> bool {
    false
}

unsafe extern "C" fn set_parent(plugin: *const clap_plugin, parent: *const clap_window) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    *plugin.gui_parent.lock() = Some(ParentWindow(*parent));

    true
}

unsafe extern "C" fn set_transient(
    _plugin: *const clap_plugin,
    _parent: *const clap_window,
) -> bool {
    false
}

unsafe extern "C" fn suggest_title(_plugin: *const clap_plugin, _title: *const c_char) {}

unsafe extern "C" fn show(plugin: *const clap_plugin) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if plugin.gui_window_handle.lock().is_some() {
        return true;
    }

    if let Some(parent) = plugin.gui_parent.lock().as_ref() {
        let handle = iced_baseview::open_parented::<
            OctaSineIcedApplication<Arc<SyncState<ClapGuiSyncHandle>>>,
            ParentWindow,
        >(
            &parent,
            get_iced_baseview_settings(plugin.sync.clone(), "OctaSine".to_string()),
        );

        *plugin.gui_window_handle.lock() = Some(handle);

        true
    } else {
        false
    }
}

unsafe extern "C" fn hide(_plugin: *const clap_plugin) -> bool {
    true
}

pub const CONFIG: clap_plugin_gui = clap_plugin_gui {
    is_api_supported: Some(is_api_supported),
    get_preferred_api: Some(get_preferred_api),
    create: Some(create),
    destroy: Some(destroy),
    set_scale: Some(set_scale),
    get_size: Some(get_size),
    can_resize: Some(can_resize),
    get_resize_hints: Some(get_resize_hints),
    adjust_size: Some(adjust_size),
    // Hack to disable Bitwig GUI support on macOS until issues with
    // cleaning up resources when destroying window are resolved.
    // REAPER currently doesn't care if this field is a null pointer,
    // while Bitwig disables GUI support if it is.
    #[cfg(target_os = "macos")]
    set_size: None,
    #[cfg(not(target_os = "macos"))]
    set_size: Some(set_size),
    set_parent: Some(set_parent),
    set_transient: Some(set_transient),
    suggest_title: Some(suggest_title),
    show: Some(show),
    hide: Some(hide),
};

pub struct ParentWindow(clap_window);

unsafe impl HasRawWindowHandle for ParentWindow {
    #[cfg(target_os = "macos")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::AppKitHandle::empty();

        unsafe {
            handle.ns_view = self.0.specific.cocoa;
        }

        RawWindowHandle::AppKit(handle)
    }

    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::Win32Handle::empty();

        unsafe {
            handle.hwnd = self.0.specific.win32;
        }

        RawWindowHandle::Win32(handle)
    }

    #[cfg(target_os = "linux")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::XcbHandle::empty();

        unsafe {
            handle.window = self.0.specific.x11 as u32;
        }

        RawWindowHandle::Xcb(handle)
    }
}
