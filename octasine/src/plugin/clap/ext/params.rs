use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use clap_sys::{
    events::{clap_input_events, clap_output_events},
    ext::params::{clap_param_info, clap_plugin_params, CLAP_PARAM_IS_AUTOMATABLE},
    plugin::clap_plugin,
};

use crate::{parameters::ParameterKey, plugin::clap::plugin::OctaSine};

fn make_c_char_arr<const N: usize>(text: &str) -> [i8; N] {
    let text = CString::new(text).unwrap();
    let text = bytemuck::cast_slice(text.as_bytes_with_nul());

    assert!(text.len() <= N);

    let mut out = [0i8; N];

    (&mut out[..text.len()]).copy_from_slice(text);

    out
}

pub unsafe extern "C" fn count(plugin: *const clap_plugin) -> u32 {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    plugin.sync.patches.num_parameters() as u32
}

pub unsafe extern "C" fn get_info(
    plugin: *const clap_plugin,
    param_index: u32,
    param_info: *mut clap_param_info,
) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if let Some(parameter) = plugin
        .sync
        .patches
        .get_parameter_by_index(param_index as usize)
    {
        *param_info = clap_param_info {
            id: parameter.parameter.key().0,
            flags: CLAP_PARAM_IS_AUTOMATABLE,
            cookie: null_mut(),
            name: make_c_char_arr(&parameter.clap_name),
            module: make_c_char_arr(&parameter.clap_path),
            min_value: 0.0,
            max_value: 1.0,
            default_value: parameter.default_value as f64,
        };

        true
    } else {
        false
    }
}

pub unsafe extern "C" fn get_value(
    plugin: *const clap_plugin,
    param_id: u32,
    value: *mut f64,
) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if let Some(p) = plugin
        .sync
        .patches
        .get_parameter_by_key(&ParameterKey(param_id))
    {
        *value = p.get_value() as f64;

        true
    } else {
        false
    }
}

pub unsafe extern "C" fn value_to_text(
    plugin: *const clap_plugin,
    param_id: u32,
    value: f64,
    c_str_ptr: *mut i8,
    c_str_len: u32,
) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if c_str_ptr.is_null() {
        return false;
    }

    if let Some(parameter) = plugin
        .sync
        .patches
        .get_parameter_by_key(&ParameterKey(param_id))
    {
        if let Ok(text) = CString::new((parameter.format)(value as f32)) {
            let bytes = bytemuck::cast_slice(text.as_bytes_with_nul());

            if bytes.len() > c_str_len as usize {
                return false;
            }

            c_str_ptr.copy_from_nonoverlapping(bytes.as_ptr(), bytes.len());

            return true;
        }
    }

    false
}

pub unsafe extern "C" fn text_to_value(
    plugin: *const clap_plugin,
    param_id: u32,
    text: *const i8,
    value: *mut f64,
) -> bool {
    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if let Some(parameter) = plugin
        .sync
        .patches
        .get_parameter_by_key(&ParameterKey(param_id))
    {
        if let Ok(text) = CStr::from_ptr(text).to_str() {
            if let Some(v) = (parameter.value_from_text)(text.into()) {
                *value = v as f64;
            }

            return true;
        }
    }

    false
}

pub unsafe extern "C" fn flush(
    plugin: *const clap_plugin,
    in_events: *const clap_input_events,
    out_events: *const clap_output_events,
) {
    // TODO: send note end events

    let plugin = &*((*plugin).plugin_data as *const OctaSine);

    if !in_events.is_null() {
        let (size_fn, get_fn) = match ((*(in_events)).size, (*(in_events)).get) {
            (Some(size_fn), Some(get_fn)) => (size_fn, get_fn),
            _ => {
                return;
            }
        };

        for i in 0..size_fn(in_events) {
            plugin.handle_event_from_host(get_fn(in_events, i));
        }
    }

    if !out_events.is_null() {
        plugin.handle_events_from_gui(&*out_events, 0);
        plugin.send_note_end_events_to_host(&*out_events);
    }
}

pub const CONFIG: clap_plugin_params = clap_plugin_params {
    count: Some(count),
    get_info: Some(get_info),
    get_value: Some(get_value),
    value_to_text: Some(value_to_text),
    text_to_value: Some(text_to_value),
    flush: Some(flush),
};
