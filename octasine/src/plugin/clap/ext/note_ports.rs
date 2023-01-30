use clap_sys::{
    ext::note_ports::{
        clap_note_port_info, clap_plugin_note_ports, CLAP_NOTE_DIALECT_CLAP, CLAP_NOTE_DIALECT_MIDI,
    },
    plugin::clap_plugin,
};

pub extern "C" fn count(_plugin: *const clap_plugin, _is_input: bool) -> u32 {
    // One input port, one output port
    1
}
pub unsafe extern "C" fn get(
    _plugin: *const clap_plugin,
    index: u32,
    is_input: bool,
    info: *mut clap_note_port_info,
) -> bool {
    if index < 2 {
        let info = &mut *info;

        info.id = 0;
        info.supported_dialects = if is_input {
            CLAP_NOTE_DIALECT_MIDI | CLAP_NOTE_DIALECT_CLAP
        } else {
            CLAP_NOTE_DIALECT_CLAP
        };
        info.preferred_dialect = CLAP_NOTE_DIALECT_CLAP;

        true
    } else {
        false
    }
}

pub const CONFIG: clap_plugin_note_ports = clap_plugin_note_ports {
    count: Some(count),
    get: Some(get),
};
