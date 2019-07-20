#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

use std::sync::Arc;

use array_init::array_init;
use rand::{FromEntropy, Rng};
use rand::rngs::SmallRng;

use vst::api::{Supported, Events};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::host::Host;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};

pub mod approximations;
pub mod common;
pub mod constants;
pub mod voices;
pub mod processing_parameters;
pub mod presets;

pub use crate::approximations::*;
pub use crate::common::*;
pub use crate::constants::*;
pub use crate::voices::*;
pub use crate::processing_parameters::*;
pub use crate::presets::*;


#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION").to_string()
    };
}


fn crate_version_to_vst_format(crate_version: String) -> i32 {
    format!("{:0<4}", crate_version.replace(".", ""))
        .parse()
        .expect("convert crate version to i32")
}


/// State used for processing
pub struct ProcessingState {
    pub global_time: TimeCounter,
    pub sample_rate: SampleRate,
    pub time_per_sample: TimePerSample,
    pub bpm: BeatsPerMinute,
    pub rng: SmallRng,
    pub envelope_curve_table: EnvelopeCurveTable,
    pub voices: [Voice; 128],
    pub parameters: ProcessingParameters,
}


/// Thread-safe state used for parameter and preset calls
pub struct SyncOnlyState {
    pub host: HostCallback,
    pub presets: PresetBank,
}


/// One for left channel, one for right
pub struct OutputChannel {
    pub additive: f32,
    pub operator_inputs: [f32; NUM_OPERATORS],
}

impl Default for OutputChannel {
    fn default() -> Self {
        Self {
            additive: 0.0,
            operator_inputs: [0.0; NUM_OPERATORS],
        }
    }
}


/// Main structure
pub struct FmSynth {
    processing: ProcessingState,
    pub sync_only: Arc<SyncOnlyState>,
}

impl Default for FmSynth {
    fn default() -> Self {
        Self::new(HostCallback::default())
    }
}


impl FmSynth {
    fn time_per_sample(sample_rate: SampleRate) -> TimePerSample {
        TimePerSample(1.0 / sample_rate.0)
    }

    fn hard_limit(value: f32) -> f32 {
        value.min(1.0).max(-1.0)
    }

    /// Generate stereo samples for a voice using SIMD
    #[cfg(feature = "simd")]
    pub fn generate_voice_samples_simd(
        envelope_curve_table: &EnvelopeCurveTable,
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut ProcessingParameters,
        voice: &mut Voice,
    ) -> (f32, f32) {
        use packed_simd::*;
        use simd_sleef_sin35::*;

        let operators = &mut parameters.operators;

        // Extract data

        let mut envelope_volume: [f32; 4] = [0.0; 4];
        let mut operator_volume: [f32; 4] = [0.0; 4];
        let mut operator_modulation_index = [0.0f32; 4];
        let mut operator_feedback: [f32; 4] = [0.0; 4];
        let mut operator_panning: [f32; 4] = [0.0; 4];
        let mut operator_additive: [f32; 4] = [0.0; 4];
        let mut operator_frequency_ratio: [f32; 4] = [0.0; 4];
        let mut operator_frequency_free: [f32; 4] = [0.0; 4];
        let mut operator_frequency_fine: [f32; 4] = [0.0; 4];
        let mut operator_modulation_targets = [0usize; 4];
        let mut operator_wave_type = [WaveType::Sine; 4];

        let mut operator_last_phase: [f32; 4] = [0.0; 4];

        for (index, operator) in operators.iter_mut().enumerate(){
            envelope_volume[index] = {
                voice.operators[index].volume_envelope.get_volume(
                    envelope_curve_table,
                    &operator.volume_envelope,
                    voice.key_pressed,
                    voice.duration
                )
            };

            operator_volume[index] = operator.volume.get_value(time);
            operator_modulation_index[index] = operator.modulation_index.get_value(time);
            operator_feedback[index] = operator.feedback.get_value(time);
            operator_panning[index] = operator.panning.get_value(time);
            operator_wave_type[index] = operator.wave_type.value;

            // Get additive factor; use 1.0 for operator 1
            operator_additive[index] = if index == 0 {
                1.0
            } else {
                operator.additive_factor.get_value(time)
            };

            operator_frequency_ratio[index] = operator.frequency_ratio.value;
            operator_frequency_free[index] = operator.frequency_free.value;
            operator_frequency_fine[index] = operator.frequency_fine.value;

            if let Some(p) = &mut operator.output_operator {
                use ProcessingParameterOperatorModulationTarget::*;

                let opt_value = match p {
                    OperatorIndex2(p) => Some(p.get_value(time)),
                    OperatorIndex3(p) => Some(p.get_value(time)),
                };

                if let Some(value) = opt_value {
                    operator_modulation_targets[index] = value;
                }
            }

            operator_last_phase[index] = voice.operators[index].last_phase.0;
        }

        // Put data into SIMD variables

        let envelope_volume_simd = f32x4::from(envelope_volume);
        let operator_volume_simd = f32x4::from(operator_volume);
        let operator_feedback_simd = f32x4::from(operator_feedback);
        let operator_modulation_index_simd = f32x4::from(operator_modulation_index);
        let operator_panning_simd = f32x4::from(operator_panning);
        let operator_additive_simd = f32x4::from(operator_additive);
        let operator_frequency_ratio_simd = f32x4::from(operator_frequency_ratio);
        let operator_frequency_free_simd = f32x4::from(operator_frequency_free);
        let operator_frequency_fine_simd = f32x4::from(operator_frequency_fine);
        let operator_last_phase_simd = f32x4::from(operator_last_phase);

        // Do calculations

        let zero_value_limit_simd = f32x4::splat(ZERO_VALUE_LIMIT);
        
        let operator_volume_product_simd = operator_volume_simd *
            envelope_volume_simd;

        let operator_volume_off_simd: m32x4 = operator_volume_product_simd.lt(
            zero_value_limit_simd
        );

        // Calculate, save and return new phase * TAU
        let operator_new_phase_simd: f32x4 = {
            let base_frequency = voice.midi_pitch.get_frequency(
                parameters.master_frequency.value
            );

            let operator_frequency_simd: f32x4 = base_frequency *
                operator_frequency_ratio_simd *
                operator_frequency_free_simd *
                operator_frequency_fine_simd;

            let mut operator_new_phase_simd = operator_last_phase_simd +
                operator_frequency_simd * time_per_sample.0;

            // Get fractional part of floats
            operator_new_phase_simd -= f32x4::from_cast(
                i32x4::from_cast(operator_new_phase_simd)
            );

            // Save new phase
            for index in 0..4 {
                voice.operators[index].last_phase.0 =
                    operator_new_phase_simd.extract(index);
            }

            operator_new_phase_simd * TAU
        };

        // Calculate feedback if it is on on any operator
        let feedback_simd: f32x4 = {
            let all_feedback_off = operator_feedback_simd.lt(
                zero_value_limit_simd
            ).all();

            if all_feedback_off {
                f32x4::splat(0.0)
            } else {
                operator_feedback_simd * SleefSin35::sin(operator_new_phase_simd)
            }
        };

        fn create_pairs(source: f32x4) -> [f32x2; 4] {
            let array: [f32; 4] = source.into();
            
            [
                f32x2::splat(array[0]),
                f32x2::splat(array[1]),
                f32x2::splat(array[2]),
                f32x2::splat(array[3]),
            ]
        }

        fn create_pairs_from_two(a: f32x4, b: f32x4) -> [f32x2; 4] {
            let array_a: [f32; 4] = a.into();
            let array_b: [f32; 4] = b.into();

            [
                f32x2::new(array_a[0], array_b[0]),
                f32x2::new(array_a[1], array_b[1]),
                f32x2::new(array_a[2], array_b[2]),
                f32x2::new(array_a[3], array_b[3]),
            ]
        }

        // Calculate panning tendency for weird modulation input panning
        let tendency_pairs = {
            let pan_transformed_simd = 2.0 * (operator_panning_simd - 0.5);
            
            let zero_splat_simd = f32x4::splat(0.0);
            
            let right_tendency_simd = pan_transformed_simd.max(zero_splat_simd);
            let left_tendency_simd = (pan_transformed_simd * -1.0)
                .max(zero_splat_simd);
            
            create_pairs_from_two(left_tendency_simd, right_tendency_simd)
        };

        let constant_power_panning_pairs = [
            f32x2::from(operators[0].panning.left_and_right),
            f32x2::from(operators[1].panning.left_and_right),
            f32x2::from(operators[2].panning.left_and_right),
            f32x2::from(operators[3].panning.left_and_right),
        ];

        // Extract data into pairs

        let phase_pairs = create_pairs(operator_new_phase_simd);
        let modulation_index_pairs = create_pairs(operator_modulation_index_simd);
        let feedback_pairs = create_pairs(feedback_simd);
        let operator_volume_product_pairs = create_pairs(operator_volume_product_simd);
        let additive_pairs = create_pairs(operator_additive_simd);

        // Generate samples

        let mut modulation_in_pairs = [f32x2::splat(0.0); 4];
        let mut additive_out_simd = f32x2::splat(0.0);

        for (index, target) in operator_modulation_targets.iter().enumerate().rev(){
            let target = *target;

            if operator_volume_off_simd.extract(index) {
                continue;
            }

            let mut out_simd = if operator_wave_type[index] == WaveType::Sine {
                let mono = modulation_in_pairs[index].sum();

                modulation_in_pairs[index] = tendency_pairs[index] * mono +
                    (1.0 - tendency_pairs[index]) * modulation_in_pairs[index];

                let sin_input_simd: f32x2 = modulation_index_pairs[index] *
                    (feedback_pairs[index] + modulation_in_pairs[index]) +
                    phase_pairs[index];

                SleefSin35::sin(sin_input_simd)
            } else {
                f32x2::splat((rng.gen::<f32>() - 0.5) * 2.0)
            };

            out_simd *= operator_volume_product_pairs[index] *
                constant_power_panning_pairs[index];
            
            let additive_out_increase_simd = additive_pairs[index] * out_simd;

            additive_out_simd += additive_out_increase_simd;
            modulation_in_pairs[target] += out_simd - additive_out_increase_simd;
        }

        let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 *
            parameters.master_volume.get_value(time);
        
        additive_out_simd *= volume_factor;

        (additive_out_simd.extract(0), additive_out_simd.extract(1))
    }

    /// Generate stereo samples for a voice
    /// 
    /// Doesn't take self parameter due to conflicting borrowing of Voices
    /// in calling function `process`
    #[allow(dead_code)]
    pub fn generate_voice_samples(
        envelope_curve_table: &EnvelopeCurveTable,
        rng: &mut impl Rng,
        time: TimeCounter,
        time_per_sample: TimePerSample,
        parameters: &mut ProcessingParameters,
        voice: &mut Voice,
    ) -> (f32, f32) {
        let operators = &mut parameters.operators;

        let base_frequency = voice.midi_pitch.get_frequency(
            parameters.master_frequency.value
        );

        let mut output_channels = [
            OutputChannel::default(),
            OutputChannel::default()
        ];

        for (operator_index, operator) in (operators.iter_mut().enumerate()).rev() {
            // Fetch all operator values here to make sure all interpolatable
            // ones are advanced even if calculations are skipped below.

            let operator_volume = operator.volume.get_value(time);
            let operator_feedback = operator.feedback.get_value(time);
            let operator_modulation_index = operator.modulation_index.get_value(time);
            let operator_panning = operator.panning.get_value(time);

            // Get additive factor; use 1.0 for operator 1
            let operator_additive = if operator_index == 0 {
                1.0
            } else {
                operator.additive_factor.get_value(time)
            };

            // Get modulation target; use operator 1 for operator 1 and 2.
            // (Since additive factor is 1.0 for operator 1, its target is
            // irrelevant.)
            let operator_mod_output = if let Some(ref p) = operator.output_operator {
                match p {
                    ProcessingParameterOperatorModulationTarget::OperatorIndex2(p) => p.value,
                    ProcessingParameterOperatorModulationTarget::OperatorIndex3(p) => p.value,
                }
            } else {
                0
            };

            let operator_frequency = base_frequency *
                operator.frequency_ratio.value *
                operator.frequency_free.value *
                operator.frequency_fine.value;

            // Always calculate envelope to make sure it advances
            let envelope_volume = {
                voice.operators[operator_index].volume_envelope.get_volume(
                    envelope_curve_table,
                    &operator.volume_envelope,
                    voice.key_pressed,
                    voice.duration
                )
            };

            // If volume is off, skip sound generation and panning
            if operator_volume < ZERO_VALUE_LIMIT ||
                envelope_volume < ZERO_VALUE_LIMIT {
                continue;
            }

            let mut operator_inputs = [
                output_channels[0].operator_inputs[operator_index],
                output_channels[1].operator_inputs[operator_index],
            ];

            // Mix modulator into current operator depending on panning of
            // current operator. If panned to the middle, just pass through
            // the stereo signals: if panned to any side, mix out the
            // original stereo signals and mix in mono.
            if operator_panning != 0.5 {
                let pan_transformed = 2.0 * (operator_panning - 0.5);

                let right_tendency = pan_transformed.max(0.0);
                let left_tendency = (-pan_transformed).max(0.0);

                let mono = operator_inputs[0] + operator_inputs[1];

                operator_inputs[0] = (1.0 - left_tendency) * operator_inputs[0] +
                    left_tendency * mono;
                operator_inputs[1] = (1.0 - right_tendency) * operator_inputs[1] +
                    right_tendency * mono;
            }

            // Calculate, save and return new phase
            let new_phase_times_tau = {
                // Calculate phase increment, add to last phase, get remainder
                // after division with 1.0 with .fract(), which seems to fix
                // an audio issue
                let new_phase = operator_frequency.mul_add(
                    time_per_sample.0,
                    voice.operators[operator_index].last_phase.0,
                ).fract();

                voice.operators[operator_index].last_phase.0 = new_phase;

                new_phase * TAU
            };

            let mut new_signals = [0.0, 0.0];

            // Generate FM sine / noise signals for each channel
            match operator.wave_type.value {
                WaveType::Sine => {
                    // Do feedback calculation only if feedback is on
                    let new_feedback = if operator_feedback > ZERO_VALUE_LIMIT {
                        operator_feedback * new_phase_times_tau.sin()
                    } else {
                        0.0
                    };

                    let inputs_identical = operator_inputs[0] == operator_inputs[1];

                    for channel in 0..2 {
                        // Skip generating right channel signal if inputs
                        // are identical - just use the left channel signal
                        if channel == 1 && inputs_identical {
                            new_signals[1] = new_signals[0];
                        } else {
                            let modulation = operator_modulation_index *
                                (operator_inputs[channel] + new_feedback);

                            let signal = (new_phase_times_tau + modulation).sin();

                            new_signals[channel] = envelope_volume * signal;
                        }
                    }
                },
                WaveType::WhiteNoise => {
                    let signal = envelope_volume *
                        (rng.gen::<f32>() - 0.5) * 2.0;

                    new_signals[0] = signal;
                    new_signals[1] = signal;
                }
            }

            // Pan signals and write to output_channels
            for channel in 0..2 {
                let pan_volume = operator.panning.left_and_right[channel];

                let out = pan_volume * operator_volume * new_signals[channel];

                let additive_out = operator_additive * out;
                let mod_out = out - additive_out;

                output_channels[channel].additive += additive_out;
                output_channels[channel]
                    .operator_inputs[operator_mod_output] += mod_out;
            }
        }

        let signal_left = output_channels[0].additive;
        let signal_right = output_channels[1].additive;

        let volume_factor = VOICE_VOLUME_FACTOR * voice.key_velocity.0 *
            parameters.master_volume.get_value(time);

        (signal_left * volume_factor, signal_right * volume_factor)
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.key_off(data[1]),
            144 => self.key_on(data[1], data[2]),
            _   => ()
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
        self.processing.voices[pitch as usize].press_key(velocity);
    }

    fn key_off(&mut self, pitch: u8) {
        self.processing.voices[pitch as usize].release_key();
    }

    /// Fetch BPM. Currently not used
    #[allow(dead_code)]
    fn fetch_bpm(&mut self){
        // Use TEMPO_VALID constant content as mask directly because
        // of problems with using TimeInfoFlags
        if let Some(time_info) = self.sync_only.host.get_time_info(1 << 10) {
            self.processing.bpm = BeatsPerMinute(time_info.tempo as f32);
        }
    }
}


impl Plugin for FmSynth {
    fn process(&mut self, audio_buffer: &mut AudioBuffer<f32>){
        let time_per_sample = self.processing.time_per_sample;

        let outputs = audio_buffer.split().1;
        let lefts = outputs.get_mut(0).iter_mut();
        let rights = outputs.get_mut(1).iter_mut();

        for (output_sample_left, output_sample_right) in lefts.zip(rights) {
            let changed_parameter_indeces = self.sync_only.presets
                .get_changed_parameters();

            if let Some(indeces) = changed_parameter_indeces {
                for (index, opt_new_value) in indeces.iter().enumerate(){
                    if let Some(new_value) = opt_new_value {
                        if let Some(p) = self.processing.parameters.get(index){
                            p.set_from_preset_value(*new_value);
                        }
                    }
                }
            }

            *output_sample_left = 0.0;
            *output_sample_right = 0.0;

            for voice in self.processing.voices.iter_mut(){
                if voice.active {
                    #[cfg(not(feature = "simd"))]
                    let (out_left, out_right) = Self::generate_voice_samples(
                        &self.processing.envelope_curve_table,
                        &mut self.processing.rng,
                        self.processing.global_time,
                        time_per_sample,
                        &mut self.processing.parameters,
                        voice,
                    );
                    #[cfg(feature = "simd")]
                    let (out_left, out_right) = Self::generate_voice_samples_simd(
                        &self.processing.envelope_curve_table,
                        &mut self.processing.rng,
                        self.processing.global_time,
                        time_per_sample,
                        &mut self.processing.parameters,
                        voice,
                    );

                    *output_sample_left += Self::hard_limit(out_left);
                    *output_sample_right += Self::hard_limit(out_right);

                    voice.duration.0 += time_per_sample.0;

                    voice.deactivate_if_envelopes_ended();
                }
            }

            self.processing.global_time.0 += time_per_sample.0;
        }
    }

    fn new(host: HostCallback) -> Self {
        let sample_rate = SampleRate(44100.0);

        let processing = ProcessingState {
            global_time: TimeCounter(0.0),
            sample_rate: sample_rate,
            time_per_sample: Self::time_per_sample(sample_rate),
            bpm: BeatsPerMinute(120.0),
            rng: SmallRng::from_entropy(),
            envelope_curve_table: EnvelopeCurveTable::new(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
            parameters: ProcessingParameters::new(),
        };

        let sync_only = Arc::new(SyncOnlyState {
            host: host,
            presets: PresetBank::new(),
        });

        Self {
            processing,
            sync_only
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: "Joakim Frostegård".to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: 43789,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            presets: self.sync_only.presets.len() as i32,
            parameters: self.sync_only.presets.get_num_parameters() as i32,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }

    #[cfg(feature = "logging")]
	fn init(&mut self) {
        let log_folder = dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

		let log_file = ::std::fs::File::create(
            log_folder.join(format!("{}.log", PLUGIN_NAME))
        ).unwrap();

		let _ = simplelog::CombinedLogger::init(vec![
            simplelog::WriteLogger::new(
                simplelog::LogLevelFilter::Info,
                simplelog::Config::default(),
                log_file
            )
        ]);

        log_panics::init();

		info!("init");
	}

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            if let Event::Midi(ev) = event {
                self.process_midi_event(ev.data);
            } 
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        let sample_rate = SampleRate(f32::from(rate));

        self.processing.sample_rate = sample_rate;
        self.processing.time_per_sample = Self::time_per_sample(sample_rate);
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent | CanDo::ReceiveTimeInfo
            | CanDo::SendEvents | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<PluginParameters> {
        Arc::clone(&self.sync_only) as Arc<PluginParameters>
    }
}


impl PluginParameters for SyncOnlyState {

    /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
    fn get_parameter_label(&self, index: i32) -> String {
        self.presets.get_parameter_unit(index as usize)
    }

    /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
    fn get_parameter_text(&self, index: i32) -> String {
        self.presets.get_parameter_value_text(index as usize)
    }

    /// Get the name of parameter at `index`.
    fn get_parameter_name(&self, index: i32) -> String {
        self.presets.get_parameter_name(index as usize)
    }

    /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
    fn get_parameter(&self, index: i32) -> f32 {
        self.presets.get_parameter_value_float(index as usize)
    }

    /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
    fn set_parameter(&self, index: i32, value: f32) {
        self.presets.set_parameter_value_float(index as usize, value);
    }

    /// Use String as input for parameter value. Used by host to provide an editable field to
    /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
    /// the input string was used.
    fn string_to_parameter(&self, index: i32, text: String) -> bool {
        self.presets.set_parameter_value_text(index as usize, text)
    }

    /// Return whether parameter at `index` can be automated.
    fn can_be_automated(&self, index: i32) -> bool {
        self.presets.can_parameter_be_automated(index as usize)
    }

    /// Set the current preset to the index specified by `preset`.
    ///
    /// This method can be called on the processing thread for automation.
    fn change_preset(&self, index: i32) {
        self.presets.set_preset_index(index as usize);
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.presets.get_preset_index() as i32
    }

    /// Set the current preset name.
    fn set_preset_name(&self, name: String) {
        self.presets.set_current_preset_name(name)
    }

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, index: i32) -> String {
        self.presets.get_preset_name_by_index(index as usize)
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current preset.
    fn get_preset_data(&self) -> Vec<u8> {
        self.presets.export_current_preset_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
    /// the current plugin bank.
    fn get_bank_data(&self) -> Vec<u8> {
        self.presets.export_bank_as_bytes()
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
    /// chunk data.
    fn load_preset_data(&self, data: &[u8]) {
        self.presets.import_bytes_into_current_preset(data);
    }

    /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
    /// given chunk data.
    fn load_bank_data(&self, data: &[u8]) {
        self.presets.import_bank_from_bytes(data);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version_to_vst_format(){
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}