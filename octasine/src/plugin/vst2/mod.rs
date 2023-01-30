#[cfg(feature = "gui")]
pub mod editor;
mod sync;

use std::sync::Arc;

use vst::api::{Events, Supported};
use vst::event::Event;
use vst::host::Host;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};

use crate::audio::gen::process_f32_runtime_select;
use crate::audio::AudioState;
use crate::sync::SyncState;
use crate::utils::{init_logging, update_audio_parameters};
use crate::{common::*, crate_version};

pub const PLUGIN_NAME: &str = "OctaSine v0.8";
pub const PLUGIN_UNIQUE_ID: i32 = 1_438_048_625;

pub struct OctaSine {
    pub audio: Box<AudioState>,
    pub sync: Arc<SyncState<vst::plugin::HostCallback>>,
    #[cfg(feature = "gui")]
    editor: Option<editor::Editor<Arc<SyncState<vst::plugin::HostCallback>>>>,
}

impl Default for OctaSine {
    fn default() -> Self {
        Self::create(None)
    }
}

impl OctaSine {
    fn create(host: Option<HostCallback>) -> Self {
        // If initialization of logging fails, we can't do much about it, but
        // we shouldn't panic
        let _ = init_logging("vst2");

        let sync = Arc::new(SyncState::new(host));

        #[cfg(feature = "gui")]
        let editor = editor::Editor::new(sync.clone());

        Self {
            audio: Default::default(),
            sync,
            #[cfg(feature = "gui")]
            editor: Some(editor),
        }
    }

    fn get_bpm_from_host(&self) -> Option<BeatsPerMinute> {
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        let mask = 1 << 10;

        let time_info = self.sync.host?.get_time_info(mask)?;

        if (time_info.flags & mask) != 0 {
            Some(BeatsPerMinute(time_info.tempo as f64))
        } else {
            None
        }
    }
}

impl Plugin for OctaSine {
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (l, r) = &mut buffer.split().1.split_at_mut(1);

        let lefts = l.get_mut(0);
        let rights = r.get_mut(0);

        update_audio_parameters(&mut self.audio, &self.sync);

        // VST2 spec does not guarantee that events are sent in order
        self.audio.sort_note_events();

        if let Some(bpm) = self.get_bpm_from_host() {
            self.audio.set_bpm(bpm);
        }

        process_f32_runtime_select(&mut self.audio, lefts, rights, 0);
    }

    fn new(host: HostCallback) -> Self {
        Self::create(Some(host))
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: "Joakim Frostegard".to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: PLUGIN_UNIQUE_ID,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            presets: self.sync.patches.num_patches() as i32,
            parameters: self.sync.patches.num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            f64_precision: false,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        self.audio
            .enqueue_note_events(events.events().filter_map(|event| {
                if let Event::Midi(event) = event {
                    let event = NoteEvent {
                        delta_frames: event.delta_frames.try_into().unwrap_or(0),
                        event: NoteEventInner::Midi { data: event.data },
                    };

                    Some(event)
                } else {
                    None
                }
            }))
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.audio.set_sample_rate(SampleRate(f64::from(rate)));
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent
            | CanDo::ReceiveTimeInfo
            | CanDo::SendEvents
            | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.sync) as Arc<dyn PluginParameters>
    }

    #[cfg(feature = "gui")]
    fn get_editor(&mut self) -> Option<Box<dyn ::vst::editor::Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn ::vst::editor::Editor>)
        } else {
            None
        }
    }
}

pub fn crate_version_to_vst_format(crate_version: &str) -> i32 {
    format!("{:0<4}", crate_version.replace('.', ""))
        .parse()
        .expect("convert crate version to i32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format() {
        assert_eq!(crate_version_to_vst_format("1"), 1000);
        assert_eq!(crate_version_to_vst_format("0.1"), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2"), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2"), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1"), 1010);
    }
}
