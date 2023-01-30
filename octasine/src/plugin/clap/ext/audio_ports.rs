use clap_sys::{
    ext::audio_ports::{
        clap_audio_port_info, clap_plugin_audio_ports, CLAP_AUDIO_PORT_IS_MAIN, CLAP_PORT_STEREO,
    },
    id::CLAP_INVALID_ID,
    plugin::clap_plugin,
};

pub extern "C" fn count(_plugin: *const clap_plugin, is_input: bool) -> u32 {
    if is_input {
        0
    } else {
        1
    }
}
pub unsafe extern "C" fn get(
    _plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info: *mut clap_audio_port_info,
) -> bool {
    if index == 0 && !is_input {
        let info = &mut *info;

        info.id = 0;
        info.channel_count = 2;
        info.flags = CLAP_AUDIO_PORT_IS_MAIN;
        info.port_type = CLAP_PORT_STEREO.as_ptr();
        info.in_place_pair = CLAP_INVALID_ID;

        true
    } else {
        return false;
    }
}

pub const CONFIG: clap_plugin_audio_ports = clap_plugin_audio_ports {
    count: Some(count),
    get: Some(get),
};
