extern crate vst;
extern crate smallvec;

use vst::api::{Supported, Events};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{Category, Plugin, Info, CanDo};
use vst::plugin_main;

pub mod constants;
pub mod parameters;
pub mod synth;
pub mod utils;
pub mod waves;

pub use synth::*;


plugin_main!(FmSynthPlugin);


pub struct FmSynthPlugin {
    synth: FmSynth,
}

impl Default for FmSynthPlugin {
    fn default() -> Self {
        Self {
            synth: FmSynth::default(),
        }
    }
}

impl Plugin for FmSynthPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "FM".to_string(),
            vendor: "Joakim Frostegård".to_string(),
            unique_id: 43789,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: self.synth.get_num_parameters() as i32,
            initial_delay: 0,
            ..Info::default()
        }
    }

    // Supresses warning about match statment only having one arm
    #[allow(unknown_lints)]
    #[allow(unused_variables)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.synth.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.synth.set_sample_rate(SampleRate(f64::from(rate)));
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.synth.generate_audio(buffer);
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    /// Parameter plumbing

    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, index: i32) -> String {
        self.synth.get_parameter_unit_of_measurement(index as usize)
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.synth.get_parameter_value_text(index as usize)
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.synth.get_parameter_name(index as usize)
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.synth.get_parameter_value_float(index as usize) as f32
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&mut self, index: i32, value: f32) {
        self.synth.set_parameter_value_float(index as usize, value as f64)
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&mut self, index: i32, text: String) -> bool {
        self.synth.set_parameter_value_text(index as usize, text)
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.synth.can_parameter_be_automated(index as usize)
    }
}