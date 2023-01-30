use std::sync::Arc;

use clap_sys::host::clap_host;
use parking_lot::Mutex;

use crate::{
    common::EventToHost,
    parameters::WrappedParameter,
    settings::Settings,
    sync::{
        change_info::MAX_NUM_PARAMETERS,
        serde::{SerdePatch, SerdePatchBank},
        GuiSyncHandle, SyncState,
    },
};

pub struct ClapGuiSyncHandle {
    pub producer: Mutex<rtrb::Producer<EventToHost>>,
    // SAFETY: calling request_process is thread-safe according to clap spec
    pub host: *const clap_host,
}

unsafe impl Send for ClapGuiSyncHandle {}
unsafe impl Sync for ClapGuiSyncHandle {}

impl ClapGuiSyncHandle {
    fn send_event(&self, event: EventToHost) {
        if let Err(err) = self.producer.lock().push(event) {
            ::log::error!("ClapGuiHandle can't send event: {:#}", err);
        }

        unsafe {
            let host = &*(self.host);

            if let Some(request_process) = host.request_process.as_ref() {
                request_process(self.host);
            }
        }
    }

    fn send_events<I: IntoIterator<Item = EventToHost>>(&self, events: I) {
        {
            let mut producer = self.producer.lock();

            for event in events {
                if let Err(err) = producer.push(event) {
                    ::log::error!("ClapGuiHandle can't send event: {:#}", err);
                }
            }
        }

        unsafe {
            let host = &*(self.host);

            if let Some(request_process) = host.request_process.as_ref() {
                request_process(self.host);
            }
        }
    }
}

impl GuiSyncHandle for Arc<SyncState<ClapGuiSyncHandle>> {
    fn begin_edit(&self, parameter: WrappedParameter) {
        if let Some(handle) = &self.host {
            handle.send_event(EventToHost::StartAutomating(parameter.key()))
        }
    }
    fn end_edit(&self, parameter: WrappedParameter) {
        if let Some(handle) = &self.host {
            handle.send_event(EventToHost::EndAutomating(parameter.key()))
        }
    }
    fn set_parameter(&self, parameter: WrappedParameter, value: f32) {
        if let Some(host) = &self.host {
            host.send_event(EventToHost::Automate(parameter.key(), value));
        }

        self.patches
            .set_parameter_from_gui(parameter.index() as usize, value);
    }
    fn set_parameter_from_text(&self, parameter: WrappedParameter, text: &str) -> Option<f32> {
        let index = parameter.index() as usize;

        if self.patches.set_parameter_text_from_gui(index, text) {
            let value = self.patches.get_parameter_value(index).unwrap();

            if let Some(host) = &self.host {
                let key = parameter.key();

                host.send_events([
                    EventToHost::StartAutomating(key),
                    EventToHost::Automate(key, value),
                    EventToHost::EndAutomating(key),
                ]);
            }

            Some(value)
        } else {
            None
        }
    }
    fn set_parameter_audio_only(&self, parameter: WrappedParameter, value: f32) {
        self.patches
            .set_parameter_from_gui(parameter.index() as usize, value);
    }
    fn get_parameter(&self, parameter: WrappedParameter) -> f32 {
        self.patches
            .get_parameter_value(parameter.index() as usize)
            .unwrap() // FIXME: unwrap
    }
    fn format_parameter_value(&self, parameter: WrappedParameter, value: f32) -> String {
        self.patches
            .format_parameter_value(parameter.index() as usize, value)
            .unwrap() // FIXME: unwrap
    }
    fn get_patches(&self) -> (usize, Vec<String>) {
        let index = self.patches.get_patch_index();
        let names = self.patches.get_patch_names();

        (index, names)
    }
    fn set_patch_index(&self, index: usize) {
        self.patches.set_patch_index(index);

        if let Some(host) = &self.host {
            host.send_event(EventToHost::RescanValues);
        }
    }
    fn get_current_patch_name(&self) -> String {
        self.patches.get_current_patch_name()
    }
    fn set_current_patch_name(&self, name: String) {
        self.patches.set_patch_name(name);

        // FIXME
        // if let Some(host) = self.host {
        //     host.update_display();
        // }
    }
    fn get_changed_parameters(&self) -> Option<[Option<f32>; MAX_NUM_PARAMETERS]> {
        self.patches.get_changed_parameters_from_gui()
    }
    fn have_patches_changed(&self) -> bool {
        self.patches.have_patches_changed()
    }
    fn get_gui_settings(&self) -> crate::gui::GuiSettings {
        Settings::load_or_default().gui
    }
    fn export_patch(&self) -> (String, Vec<u8>) {
        let name = self.patches.get_current_patch_filename_for_export();
        let data = self.patches.export_current_patch_fxp_bytes();

        (name, data)
    }
    fn export_bank(&self) -> Vec<u8> {
        self.patches.export_bank_as_fxb_bytes()
    }
    fn import_bank_from_serde(&self, serde_bank: SerdePatchBank) {
        self.patches.import_bank_from_serde(serde_bank);

        if let Some(host) = &self.host {
            host.send_event(EventToHost::RescanValues);
        }
    }
    fn import_patches_from_serde(&self, serde_patches: Vec<SerdePatch>) {
        self.patches.import_patches_from_serde(serde_patches);

        if let Some(host) = &self.host {
            host.send_event(EventToHost::RescanValues);
        }
    }
    fn clear_patch(&self) {
        self.patches.clear_current_patch();

        if let Some(host) = &self.host {
            host.send_event(EventToHost::RescanValues);
        }
    }
    fn clear_bank(&self) {
        self.patches.clear_bank();

        if let Some(host) = &self.host {
            host.send_event(EventToHost::RescanValues);
        }
    }
}
