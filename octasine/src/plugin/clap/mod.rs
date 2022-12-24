pub mod descriptor;
pub mod ext;
pub mod factory;
pub mod plugin;
pub mod sync;

use std::{
    ffi::{c_void, CStr},
    ptr::null,
};

use clap_sys::{
    entry::clap_plugin_entry, plugin_factory::CLAP_PLUGIN_FACTORY_ID, version::CLAP_VERSION,
};

#[allow(non_upper_case_globals)]
pub const clap_entry: clap_plugin_entry = clap_plugin_entry {
    clap_version: CLAP_VERSION,
    init: None,
    deinit: None,
    get_factory: Some(entry_get_factory),
};

pub unsafe extern "C" fn entry_get_factory(factory_id: *const i8) -> *const c_void {
    let factory_id = unsafe { CStr::from_ptr(factory_id) };

    if factory_id == CLAP_PLUGIN_FACTORY_ID {
        &factory::FACTORY as *const _ as *const c_void
    } else {
        null()
    }
}
