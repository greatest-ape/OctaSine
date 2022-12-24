use std::ffi::CStr;

use cfg_if::cfg_if;
use clap_sys::{
    ext::gui::{clap_gui_resize_hints, clap_plugin_gui, clap_window},
    plugin::clap_plugin,
};

use crate::gui::{GUI_HEIGHT, GUI_WIDTH};

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
    api: *const i8,
    is_floating: bool,
) -> bool {
    !is_floating && CStr::from_ptr(api) == SUPPORTED_API
}

unsafe extern "C" fn get_preferred_api(
    _plugin: *const clap_plugin,
    api: *mut *const i8,
    is_floating: *mut bool,
) -> bool {
    *api = SUPPORTED_API.as_ptr();
    *is_floating = false;

    true
}

unsafe extern "C" fn create(plugin: *const clap_plugin, api: *const i8, is_floating: bool) -> bool {
    if is_floating || CStr::from_ptr(api) != SUPPORTED_API {
        return false;
    }

    todo!()
}

extern "C" fn destroy(_plugin: *const clap_plugin) {
    todo!()
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

unsafe extern "C" fn set_parent(plugin: *const clap_plugin, parent: *const clap_window) -> bool {
    todo!()
}

extern "C" fn show(_plugin: *const clap_plugin) -> bool {
    todo!()
}

extern "C" fn hide(_plugin: *const clap_plugin) -> bool {
    todo!()
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
    adjust_size: None,
    set_size: None,
    set_parent: Some(set_parent),
    set_transient: None,
    suggest_title: None,
    show: Some(show),
    hide: Some(hide),
};
