use clap_sys::{
    ext::draft::voice_info::{clap_plugin_voice_info, clap_voice_info},
    plugin::clap_plugin,
};

unsafe extern "C" fn get(_plugin: *const clap_plugin, voice_info: *mut clap_voice_info) -> bool {
    *voice_info = clap_voice_info {
        voice_count: 128,
        voice_capacity: 128,
        flags: 0,
    };

    true
}

pub const CONFIG: clap_plugin_voice_info = clap_plugin_voice_info { get: Some(get) };
