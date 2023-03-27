use std::ffi::c_void;

use anyhow::Context;
use clap_sys::{
    ext::state::clap_plugin_state,
    plugin::clap_plugin,
    stream::{clap_istream, clap_ostream},
};
use serde::{Deserialize, Serialize};

use crate::{plugin::clap::plugin::OctaSine, sync::serde::SerdePatchBank};

#[derive(Serialize, Deserialize)]
struct OctaSineClapState {
    /// Store a version in case of future changes
    clap_state_version: u32,
    patch_bank: SerdePatchBank,
}

pub const CONFIG: clap_plugin_state = clap_plugin_state {
    save: Some(save),
    load: Some(load),
};

unsafe extern "C" fn save(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    let write = if let Some(write) = (&*stream).write {
        write
    } else {
        return false;
    };

    let state = OctaSineClapState {
        clap_state_version: 1,
        patch_bank: plugin.sync.patches.export_bank(),
    };

    let bytes = match serde_json::to_vec(&state) {
        Ok(bytes) => bytes,
        Err(err) => {
            ::log::error!("serialize OctaSineClapState: {:#}", err);

            return false;
        }
    };

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

    match handle_buffer(plugin, &full_buffer) {
        Ok(()) => true,
        Err(err) => {
            ::log::error!("load OctaSineClapState: {:#}", err);

            false
        }
    }
}

fn handle_buffer(plugin: &OctaSine, buffer: &[u8]) -> anyhow::Result<()> {
    let mut state = serde_json::from_slice::<OctaSineClapState>(buffer)
        .with_context(|| "deserialize OctaSineState")?;

    state
        .patch_bank
        .run_compatibility_changes()
        .with_context(|| "run patch compatibility changes")?;

    plugin.sync.patches.import_bank_from_serde(state.patch_bank);

    Ok(())
}
