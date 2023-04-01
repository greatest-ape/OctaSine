use std::ffi::c_void;

use clap_sys::{
    ext::state::clap_plugin_state,
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};

use crate::plugin::clap::plugin::OctaSine;

const VERSION: u8 = 1;

pub const CONFIG: clap_plugin_state = clap_plugin_state {
    save: Some(save),
    load: Some(load),
};

unsafe extern "C" fn save(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if stream.is_null() {
        return false;
    }

    let write = if let Some(write) = (&*stream).write {
        write
    } else {
        return false;
    };

    let mut bytes = plugin.sync.patches.export_plain_bytes();

    // Add format version as first byte for future proofing
    bytes.insert(0, VERSION);

    let mut offset = 0;

    loop {
        let buffer = &bytes[offset..];
        let result = write(
            stream,
            buffer.as_ptr() as *const c_void,
            buffer.len() as u64,
        );

        if result > 0 {
            offset += result as u64 as usize;

            if offset == bytes.len() {
                return true;
            }
        } else {
            return false;
        }
    }
}

unsafe extern "C" fn load(plugin: *const clap_plugin, stream: *const clap_istream) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if stream.is_null() {
        return false;
    }

    let read = if let Some(read) = (&*stream).read {
        read
    } else {
        return false;
    };

    let mut full_buffer = Vec::new();

    loop {
        let mut buffer = [0u8; 4096];

        match read(
            stream,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as u64,
        ) {
            -1 => return false,
            0 => break,
            n => {
                full_buffer.extend_from_slice(&buffer[..n as u64 as usize]);
            }
        }
    }

    if full_buffer.len() < 2 {
        return false;
    }

    // Remove first byte, it is the version signifier
    let full_buffer = &full_buffer[1..];

    match plugin.sync.patches.import_bank_from_bytes(full_buffer) {
        Ok(()) => true,
        Err(err) => {
            ::log::error!("load OctaSineClapState: {:#}", err);

            false
        }
    }
}
