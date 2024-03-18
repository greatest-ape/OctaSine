use std::os::raw::c_char;

use mts_esp_client_sys::{
    MTSClient, MTS_DeregisterClient, MTS_NoteToFrequency, MTS_RegisterClient,
};

pub const PLUGIN_UNIQUE_VST2_ID: i32 = 1_438_048_626;
pub const PLUGIN_SEMVER_NAME: &str = "OctaSine v0.9";

pub fn crate_version_to_vst2_format(crate_version: &str) -> i32 {
    format!("{:0<4}", crate_version.replace('.', ""))
        .parse()
        .expect("convert crate version to i32")
}

pub struct MtsClient(*mut MTSClient);

impl MtsClient {
    /// Safety: only call once per instance
    pub unsafe fn new() -> Self {
        Self(MTS_RegisterClient())
    }

    /// Safety: only call from one thread
    pub unsafe fn note_to_frequency(&self, midi_note: u8, midi_channel: Option<u8>) -> f64 {
        let midi_note = c_char::from_ne_bytes(midi_note.min(127).to_ne_bytes());
        let midi_channel = midi_channel
            .map(|midi_channel| c_char::from_ne_bytes(midi_channel.min(15).to_ne_bytes()))
            .unwrap_or_else(|| {
                // Try to account for the fact that ARM c_char is unsigned,
                // while the API wants to be passed -1 to indicate "no
                // particular channel"
                #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
                {
                    u8::from_ne_bytes((-1i8).to_ne_bytes())
                }
                #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
                {
                    -1i8
                }
            });

        MTS_NoteToFrequency(self.0, midi_note, midi_channel)
    }
}

impl Drop for MtsClient {
    fn drop(&mut self) {
        unsafe {
            MTS_DeregisterClient(self.0);
        }
    }
}

unsafe impl Send for MtsClient {}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format() {
        assert_eq!(crate_version_to_vst2_format("1"), 1000);
        assert_eq!(crate_version_to_vst2_format("0.1"), 0100);
        assert_eq!(crate_version_to_vst2_format("0.0.2"), 0020);
        assert_eq!(crate_version_to_vst2_format("0.5.2"), 0520);
        assert_eq!(crate_version_to_vst2_format("1.0.1"), 1010);
    }
}
