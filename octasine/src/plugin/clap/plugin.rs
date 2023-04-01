use std::{
    ffi::{c_char, c_void, CStr},
    mem::{size_of, MaybeUninit},
    ptr::{null, null_mut},
    sync::Arc,
};

use atomic_refcell::AtomicRefCell;
use clap_sys::{
    events::{
        clap_event_header, clap_event_midi, clap_event_note, clap_event_param_gesture,
        clap_event_param_value, clap_event_transport, clap_output_events, CLAP_CORE_EVENT_SPACE_ID,
        CLAP_EVENT_IS_LIVE, CLAP_EVENT_MIDI, CLAP_EVENT_NOTE_END, CLAP_EVENT_NOTE_OFF,
        CLAP_EVENT_NOTE_ON, CLAP_EVENT_PARAM_GESTURE_BEGIN, CLAP_EVENT_PARAM_GESTURE_END,
        CLAP_EVENT_PARAM_VALUE, CLAP_EVENT_TRANSPORT, CLAP_TRANSPORT_HAS_TEMPO,
    },
    ext::{
        audio_ports::CLAP_EXT_AUDIO_PORTS,
        draft::voice_info::CLAP_EXT_VOICE_INFO,
        gui::CLAP_EXT_GUI,
        note_ports::CLAP_EXT_NOTE_PORTS,
        params::{clap_host_params, CLAP_EXT_PARAMS, CLAP_PARAM_RESCAN_VALUES},
        state::{clap_host_state, CLAP_EXT_STATE},
    },
    host::clap_host,
    plugin::clap_plugin,
    process::{clap_process, clap_process_status, CLAP_PROCESS_CONTINUE, CLAP_PROCESS_ERROR},
};
use iced_baseview::window::WindowHandle;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use ringbuf::{Consumer, Producer, Rb, SharedRb};

use crate::{
    audio::{gen::process_f32_runtime_select, AudioState},
    common::{BeatsPerMinute, EventToHost, NoteEvent, NoteEventInner, SampleRate},
    parameters::ParameterKey,
    sync::SyncState,
    utils::{init_logging, update_audio_parameters},
};

use super::{descriptor::DESCRIPTOR, ext::gui::ParentWindow, sync::ClapGuiSyncHandle};

pub type EventToHostConsumer =
    Consumer<EventToHost, Arc<SharedRb<EventToHost, Vec<MaybeUninit<EventToHost>>>>>;
pub type EventToHostProducer =
    Producer<EventToHost, Arc<SharedRb<EventToHost, Vec<MaybeUninit<EventToHost>>>>>;

pub struct OctaSine {
    pub host: *const clap_host,
    pub audio: Mutex<Box<AudioState>>,
    pub sync: Arc<SyncState<ClapGuiSyncHandle>>,
    pub gui_event_consumer: Mutex<EventToHostConsumer>,
    pub gui_parent: Mutex<Option<ParentWindow>>,
    pub gui_window_handle: Mutex<Option<WindowHandle<crate::gui::Message>>>,
    pub clap_plugin: AtomicRefCell<clap_plugin>,
}

impl OctaSine {
    pub fn new(host: *const clap_host) -> Arc<Self> {
        let _ = init_logging("clap");

        let (gui_event_producer, gui_event_consumer) = SharedRb::new(1024).split();

        let gui_sync_handle = ClapGuiSyncHandle {
            producer: Mutex::new(gui_event_producer),
            host,
        };

        let plugin = Self {
            host,
            audio: Default::default(),
            sync: Arc::new(SyncState::new(Some(gui_sync_handle))),
            gui_event_consumer: Mutex::new(gui_event_consumer),
            gui_parent: Default::default(),
            gui_window_handle: Default::default(),
            clap_plugin: AtomicRefCell::new(clap_plugin {
                desc: Lazy::force(&DESCRIPTOR) as *const _,
                plugin_data: null_mut(),
                init: Some(Self::init),
                destroy: Some(Self::destroy),
                activate: Some(Self::activate),
                deactivate: Some(Self::deactivate),
                start_processing: Some(Self::start_processing),
                stop_processing: Some(Self::stop_processing),
                reset: Some(Self::reset),
                process: Some(Self::process),
                get_extension: Some(Self::get_extension),
                on_main_thread: Some(Self::on_main_thread),
            }),
        };

        let plugin = Arc::new(plugin);

        plugin.clap_plugin.borrow_mut().plugin_data = Arc::as_ptr(&plugin) as *mut _;

        plugin
    }

    unsafe extern "C" fn init(_plugin: *const clap_plugin) -> bool {
        true
    }

    unsafe extern "C" fn destroy(plugin: *const clap_plugin) {
        assert!(!plugin.is_null() && !(*plugin).plugin_data.is_null());

        drop(Arc::from_raw((*plugin).plugin_data as *mut Self));
    }

    unsafe extern "C" fn activate(
        plugin: *const clap_plugin,
        sample_rate: f64,
        _min_frames_count: u32,
        _max_frames_count: u32,
    ) -> bool {
        let plugin = &*((*plugin).plugin_data as *const Self);

        plugin.audio.lock().set_sample_rate(SampleRate(sample_rate));

        true
    }

    unsafe extern "C" fn deactivate(_plugin: *const clap_plugin) {}

    unsafe extern "C" fn start_processing(_plugin: *const clap_plugin) -> bool {
        true
    }

    unsafe extern "C" fn stop_processing(_plugin: *const clap_plugin) {}

    unsafe extern "C" fn reset(_plugin: *const clap_plugin) {}

    unsafe extern "C" fn process(
        plugin: *const clap_plugin,
        process: *const clap_process,
    ) -> clap_process_status {
        if plugin.is_null() | (*plugin).plugin_data.is_null() | process.is_null() {
            return CLAP_PROCESS_ERROR;
        }

        let plugin = &*((*plugin).plugin_data as *const Self);
        let process = &*process;

        if process.audio_outputs_count != 1 || process.audio_outputs.is_null() {
            return CLAP_PROCESS_ERROR;
        }

        let audio_outputs = &*process.audio_outputs;

        if (audio_outputs.channel_count != 2) | audio_outputs.data32.is_null() {
            return CLAP_PROCESS_ERROR;
        }

        let audio_outputs =
            ::std::slice::from_raw_parts_mut(audio_outputs.data32 as *mut *mut f32, 2);

        if audio_outputs[0].is_null() | audio_outputs[1].is_null() {
            return CLAP_PROCESS_ERROR;
        }

        let lefts =
            ::std::slice::from_raw_parts_mut(audio_outputs[0], process.frames_count as usize);
        let rights =
            ::std::slice::from_raw_parts_mut(audio_outputs[1], process.frames_count as usize);

        let opt_in_event_data = if !process.in_events.is_null() {
            match ((*(process.in_events)).size, (*(process.in_events)).get) {
                (Some(size_fn), Some(get_fn)) => {
                    let num_events = size_fn(process.in_events);

                    if num_events == 0 {
                        None
                    } else {
                        Some((num_events, get_fn))
                    }
                }
                _ => {
                    return CLAP_PROCESS_ERROR;
                }
            }
        } else {
            None
        };

        if !process.transport.is_null() {
            plugin.handle_transport_event_from_host(&*(process.transport));
        }

        let opt_process_out_events = if !process.out_events.is_null() {
            Some(&*(process.out_events))
        } else {
            None
        };

        let mut process_start_index = 0u32;
        let mut process_end_index = process.frames_count;
        let mut event_index = 0u32;

        // Split buffer into segments by events, generate audio
        loop {
            if let Some((num_events, get_fn)) = opt_in_event_data {
                while event_index < num_events {
                    let event_header = get_fn(process.in_events, event_index);

                    if (*event_header).time != process_start_index {
                        process_end_index = (*event_header).time;

                        break;
                    }

                    plugin.handle_event_from_host(event_header);

                    event_index += 1;
                }
            }

            {
                let mut audio = plugin.audio.lock();

                let lefts = &mut lefts[process_start_index as usize..process_end_index as usize];
                let rights = &mut rights[process_start_index as usize..process_end_index as usize];

                process_f32_runtime_select(
                    &mut audio,
                    lefts,
                    rights,
                    process_start_index as usize,
                    |audio| {
                        if let Some(process_out_events) = opt_process_out_events {
                            plugin.send_gui_events_to_host(process_out_events, process_start_index);
                        }

                        update_audio_parameters(audio, &plugin.sync);
                    },
                );
            }

            if let Some(process_out_events) = opt_process_out_events {
                plugin.send_note_end_events_to_host(process_out_events);
            }

            if process_end_index == process.frames_count {
                break;
            }

            process_start_index = process_end_index;
            process_end_index = process.frames_count;
        }

        // Log any unhandled events. Should never happen.
        if let Some((num_events, get_fn)) = opt_in_event_data {
            while event_index < num_events {
                let event_header = get_fn(process.in_events, event_index);

                if !event_header.is_null() {
                    ::log::error!("OctaSine::process: unhandled event: {:?}", *event_header);
                }

                event_index += 1;
            }
        }

        CLAP_PROCESS_CONTINUE
    }

    unsafe extern "C" fn get_extension(
        _plugin: *const clap_plugin,
        id: *const c_char,
    ) -> *const c_void {
        let id = CStr::from_ptr(id);

        if id == CLAP_EXT_AUDIO_PORTS {
            &super::ext::audio_ports::CONFIG as *const _ as *const c_void
        } else if id == CLAP_EXT_NOTE_PORTS {
            &super::ext::note_ports::CONFIG as *const _ as *const c_void
        } else if id == CLAP_EXT_PARAMS {
            &super::ext::params::CONFIG as *const _ as *const c_void
        } else if id == CLAP_EXT_GUI {
            &super::ext::gui::CONFIG as *const _ as *const c_void
        } else if id == CLAP_EXT_VOICE_INFO {
            &super::ext::voice_info::CONFIG as *const _ as *const c_void
        } else if id == CLAP_EXT_STATE {
            &super::ext::state::CONFIG as *const _ as *const c_void
        } else {
            null()
        }
    }

    unsafe extern "C" fn on_main_thread(_plugin: *const clap_plugin) {}

    pub unsafe fn handle_event_from_host(&self, event_header: *const clap_event_header) {
        if (*event_header).space_id != CLAP_CORE_EVENT_SPACE_ID {
            return;
        }

        match (*event_header).type_ {
            CLAP_EVENT_NOTE_ON => {
                let event = &*(event_header as *const clap_event_note);

                let event = NoteEvent {
                    delta_frames: event.header.time,
                    event: NoteEventInner::ClapNoteOn {
                        key: event.key as u8,
                        velocity: event.velocity,
                        clap_note_id: event.note_id,
                    },
                };

                self.audio.lock().enqueue_note_event(event);
            }
            CLAP_EVENT_NOTE_OFF => {
                let event = &*(event_header as *const clap_event_note);

                let event = NoteEvent {
                    delta_frames: event.header.time,
                    event: NoteEventInner::ClapNoteOff {
                        key: event.key as u8,
                    },
                };

                self.audio.lock().enqueue_note_event(event);
            }
            CLAP_EVENT_MIDI => {
                let event = &*(event_header as *const clap_event_midi);

                let event = NoteEvent {
                    delta_frames: event.header.time,
                    event: NoteEventInner::Midi { data: event.data },
                };

                self.audio.lock().enqueue_note_event(event);
            }
            CLAP_EVENT_PARAM_VALUE => {
                let event = &*(event_header as *const clap_event_param_value);

                let opt_index_and_parameter = if event.cookie.is_null() {
                    let key = ParameterKey(event.param_id);

                    self.sync.patches.get_index_and_parameter_by_key(&key)
                } else {
                    let index = event.cookie as u64 as usize;

                    self.sync
                        .patches
                        .get_parameter_by_index(index)
                        .map(|p| (index, p))
                };

                if let Some((index, p)) = opt_index_and_parameter {
                    let value = event.value as f32;

                    p.set_value(value);

                    self.sync
                        .patches
                        .parameter_change_info_gui
                        .mark_as_changed(index);

                    self.audio
                        .lock()
                        .set_parameter_from_patch(p.parameter.parameter(), value)
                }
            }
            CLAP_EVENT_TRANSPORT => {
                let event = &*(event_header as *const clap_event_transport);

                self.handle_transport_event_from_host(event);
            }
            _ => {}
        }
    }

    pub fn handle_transport_event_from_host(&self, event: &clap_event_transport) {
        if event.header.space_id != CLAP_CORE_EVENT_SPACE_ID {
            return;
        }

        if event.flags & CLAP_TRANSPORT_HAS_TEMPO != 0 {
            let event = NoteEvent {
                delta_frames: event.header.time,
                event: NoteEventInner::ClapBpm {
                    bpm: BeatsPerMinute(event.tempo),
                },
            };

            self.audio.lock().enqueue_note_event(event);
        }
    }

    pub unsafe fn send_gui_events_to_host(&self, out_events: &clap_output_events, time: u32) {
        if let Some(try_push_fn) = out_events.try_push {
            let mut event_consumer = self.gui_event_consumer.lock();

            for event in event_consumer.pop_iter() {
                match event {
                    EventToHost::StartAutomating(parameter_key) => {
                        let event = clap_event_param_gesture {
                            header: clap_event_header {
                                size: size_of::<clap_event_param_gesture>() as u32,
                                time,
                                space_id: CLAP_CORE_EVENT_SPACE_ID,
                                type_: CLAP_EVENT_PARAM_GESTURE_BEGIN,
                                flags: CLAP_EVENT_IS_LIVE,
                            },
                            param_id: parameter_key.0,
                        };

                        try_push_fn(out_events, &event as *const _ as *const _);
                    }
                    EventToHost::EndAutomating(parameter_key) => {
                        let event = clap_event_param_gesture {
                            header: clap_event_header {
                                size: size_of::<clap_event_param_gesture>() as u32,
                                time,
                                space_id: CLAP_CORE_EVENT_SPACE_ID,
                                type_: CLAP_EVENT_PARAM_GESTURE_END,
                                flags: CLAP_EVENT_IS_LIVE,
                            },
                            param_id: parameter_key.0,
                        };

                        try_push_fn(out_events, &event as *const _ as *const _);
                    }
                    EventToHost::Automate(parameter_key, value) => {
                        let event = clap_event_param_value {
                            header: clap_event_header {
                                size: size_of::<clap_event_param_value>() as u32,
                                time,
                                space_id: CLAP_CORE_EVENT_SPACE_ID,
                                type_: CLAP_EVENT_PARAM_VALUE,
                                flags: CLAP_EVENT_IS_LIVE,
                            },
                            param_id: parameter_key.0,
                            cookie: null_mut(),
                            note_id: -1,
                            port_index: 0,
                            channel: -1,
                            key: -1,
                            value: value as f64,
                        };

                        try_push_fn(out_events, &event as *const _ as *const _);
                    }
                    EventToHost::RescanValues => {
                        self.tell_host_to_rescan_values();
                    }
                    EventToHost::StateChanged => {
                        self.tell_host_state_is_dirty();
                    }
                };
            }
        }
    }

    pub fn send_note_end_events_to_host(&self, out_events: &clap_output_events) {
        if let Some(try_push_fn) = out_events.try_push {
            for note_ended in self.audio.lock().clap_ended_notes.pop_iter() {
                unsafe {
                    let event = clap_event_note {
                        header: clap_event_header {
                            size: size_of::<clap_event_note>() as u32,
                            time: note_ended.sample_index,
                            space_id: CLAP_CORE_EVENT_SPACE_ID,
                            type_: CLAP_EVENT_NOTE_END,
                            flags: 0,
                        },
                        note_id: note_ended.clap_note_id,
                        port_index: 0,
                        channel: -1,
                        key: note_ended.key.into(),
                        velocity: 0.0,
                    };

                    try_push_fn(out_events, &event as *const _ as *const _);
                }
            }
        }
    }

    unsafe fn tell_host_to_rescan_values(&self) {
        let host = &*(self.host);

        let get_extension = host.get_extension.unwrap();

        let ext = get_extension(self.host, CLAP_EXT_PARAMS.as_ptr()) as *const clap_host_params;

        if ext.is_null() {
            ::log::error!("host doesn't implement params extension");
        } else {
            // FIXME: should maybe clear automations etc too
            (&*(ext)).rescan.unwrap()(self.host, CLAP_PARAM_RESCAN_VALUES);
        }
    }

    unsafe fn tell_host_state_is_dirty(&self) {
        let host = &*(self.host);

        let get_extension = host.get_extension.unwrap();

        let ext = get_extension(self.host, CLAP_EXT_STATE.as_ptr()) as *const clap_host_state;

        if ext.is_null() {
            ::log::error!("host doesn't implement state extension");
        } else {
            (&*(ext)).mark_dirty.unwrap()(self.host);
        }
    }
}
