use std::{ffi::CStr, ptr::null, sync::Arc};

use clap_sys::{
    host::clap_host,
    plugin::{clap_plugin, clap_plugin_descriptor},
    plugin_factory::clap_plugin_factory,
};
use once_cell::sync::Lazy;

use super::{descriptor::DESCRIPTOR, plugin::OctaSine};

pub const FACTORY: clap_plugin_factory = clap_plugin_factory {
    get_plugin_count: Some(get_plugin_count),
    get_plugin_descriptor: Some(get_plugin_descriptor),
    create_plugin: Some(create_plugin),
};

pub extern "C" fn get_plugin_count(_factory: *const clap_plugin_factory) -> u32 {
    1
}

pub extern "C" fn get_plugin_descriptor(
    _factory: *const clap_plugin_factory,
    index: u32,
) -> *const clap_plugin_descriptor {
    if index == 0 {
        Lazy::force(&DESCRIPTOR) as *const _
    } else {
        null()
    }
}

pub unsafe extern "C" fn create_plugin(
    _factory: *const clap_plugin_factory,
    host: *const clap_host,
    plugin_id: *const i8,
) -> *const clap_plugin {
    if !plugin_id.is_null() && CStr::from_ptr(plugin_id) == CStr::from_ptr(super::descriptor::ID) {
        (*Arc::into_raw(OctaSine::new(host))).clap_plugin.as_ptr()
    } else {
        null()
    }
}
