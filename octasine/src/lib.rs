pub mod audio;
pub mod common;
pub mod math;
pub mod parameters;
pub mod plugin;
pub mod settings;
pub mod simd;
pub mod sync;
pub mod utils;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "clap")]
#[no_mangle]
pub static clap_entry: ::clap_sys::entry::clap_plugin_entry = plugin::clap::CLAP_ENTRY;

#[cfg(feature = "vst2")]
::vst::plugin_main!(plugin::vst2::OctaSine);

#[cfg(test)]
mod tests {
    use crate::{
        audio::AudioState, common::SampleRate, parameters::PARAMETERS, sync::SyncState,
        utils::update_audio_parameters,
    };

    #[test]
    fn test_parameter_interaction() {
        let mut audio = AudioState::default();
        let sync = SyncState::<()>::new(None);

        let mut patch_values = Vec::new();

        for i in 0..PARAMETERS.len() {
            let patch_value = fastrand::f32();

            sync.patches.set_parameter_from_host(i, patch_value);

            patch_values.push(patch_value)
        }

        update_audio_parameters(&mut audio, &sync);

        let sample_rate = SampleRate(44100.0);

        {
            for _ in 0..44100 {
                audio.advance_one_sample(sample_rate);
            }
        }

        for (i, parameter) in PARAMETERS.iter().copied().enumerate() {
            assert_eq!(i, parameter.to_index() as usize);

            let values_approx_eq = audio.compare_parameter_patch_value(parameter, patch_values[i]);

            if !values_approx_eq {
                println!("Parameter: {:?}", parameter);
                println!("Set patch value: {}", patch_values[i]);
            }

            assert!(values_approx_eq)
        }
    }
}
