use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;
use vst2_helpers::processing_parameters::utils::*;

use crate::common::*;
use crate::constants::*;

use crate::preset_bank::PresetParameter;
use crate::parameters::processing::values::*;


impl PresetParameter {
    pub fn master_volume() -> Self {
        let value = MasterVolume::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: "Master volume".to_string(),
            unit_from_value: |_| "dB".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::MasterVolume(
                MasterVolume::from_sync(v)
            ),
            format: |v| MasterVolume::from_sync(v).format(),
        }
    }

    pub fn master_frequency() -> Self {
        let value = MasterFrequency::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: "Master frequency".to_string(),
            unit_from_value: |_| "Hz".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::MasterFrequency(
                MasterFrequency::from_sync(v)
            ),
            format: |v| MasterFrequency::from_sync(v).format(),
        }
    }

    pub fn operator_volume(index: usize) -> Self {
        let value = OperatorVolume::new(index).to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} volume", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::OperatorVolume(
                OperatorVolume::from_sync(v)
            ),
            format: |v| OperatorVolume::from_sync(v).format(),
        }
    }

    pub fn operator_panning(index: usize) -> Self {
        let value = OperatorPanning::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} pan", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorPanning(
                OperatorPanning::from_sync(v)
            ),
            format: |v| OperatorPanning::from_sync(v).format(),
        }
    }

    pub fn operator_additive(index: usize) -> Self {
        let value = OperatorAdditive::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} additive", index + 1),
            unit_from_value: |_| "%".to_string(),
            value_from_text: |v| None,
            to_processing: |v| ProcessingValue::OperatorAdditive(
                OperatorAdditive::from_sync(v)
            ),
            format: |v| OperatorAdditive::from_sync(v).format(),
        }
    }

    pub fn operator_frequency_ratio(index: usize) -> Self {
        let value = OperatorFrequencyRatio::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} freq. ratio", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| {
                v.parse::<f64>().ok().map(|value|
                    round_to_step(&OPERATOR_RATIO_STEPS[..], value)
                ).map(|v| OperatorFrequencyRatio(v).to_sync())
            },
            to_processing: |v| ProcessingValue::OperatorFrequencyRatio(
                OperatorFrequencyRatio::from_sync(v)
            ),
            format: |v| OperatorFrequencyRatio::from_sync(v).format(),
        }
    }

    pub fn operator_frequency_free(index: usize) -> Self {
        let value = OperatorFrequencyFree::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} freq. free", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None, // FIXME: simple parameter parsing
            to_processing: |v| ProcessingValue::OperatorFrequencyFree(
                OperatorFrequencyFree::from_sync(v)
            ),
            format: |v| OperatorFrequencyFree::from_sync(v).format(),
        }
    }

    pub fn operator_frequency_fine(index: usize) -> Self {
        let value = OperatorFrequencyFine::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} freq. fine", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None, // FIXME: simple parameter parsing
            to_processing: |v| ProcessingValue::OperatorFrequencyFine(
                OperatorFrequencyFine::from_sync(v)
            ),
            format: |v| OperatorFrequencyFine::from_sync(v).format(),
        }
    }

    pub fn operator_feedback(index: usize) -> Self {
        let value = OperatorFeedback::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} feedback", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None, // FIXME: simple parameter parsing
            to_processing: |v| ProcessingValue::OperatorFeedback(
                OperatorFeedback::from_sync(v)
            ),
            format: |v| OperatorFeedback::from_sync(v).format(),
        }
    }

    pub fn operator_modulation_index(index: usize) -> Self {
        let value = OperatorModulationIndex::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} mod index", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |v| None, // FIXME: simple parameter parsing
            to_processing: |v| ProcessingValue::OperatorModulationIndex(
                OperatorModulationIndex::from_sync(v)
            ),
            format: |v| OperatorModulationIndex::from_sync(v).format(),
        }
    }

    pub fn operator_wave_type(index: usize) -> Self {
        let value = OperatorWaveType::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} wave type", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| {
                let value = value.to_lowercase();

                if value.contains("sin"){
                    Some(OperatorWaveType(WaveType::Sine).to_sync())
                } else if value.contains("noise") {
                    Some(OperatorWaveType(WaveType::WhiteNoise).to_sync())
                } else {
                    None
                }
            },
            to_processing: |v| ProcessingValue::OperatorWaveType(
                OperatorWaveType::from_sync(v)
            ),
            format: |v| OperatorWaveType::from_sync(v).format(),
        }
    }

    pub fn operator_attack_duration(index: usize) -> Self {
        let value = OperatorAttackDuration::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} attack time", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorAttackDuration(
                OperatorAttackDuration::from_sync(v)
            ),
            format: |v| OperatorAttackDuration::from_sync(v).format(),
        }
    }

    pub fn operator_attack_volume(index: usize) -> Self {
        let value = OperatorAttackVolume::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} attack volume", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorAttackVolume(
                OperatorAttackVolume::from_sync(v)
            ),
            format: |v| OperatorAttackVolume::from_sync(v).format(),
        }
    }

    pub fn operator_decay_duration(index: usize) -> Self {
        let value = OperatorDecayDuration::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} decay time", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorDecayDuration(
                OperatorDecayDuration::from_sync(v)
            ),
            format: |v| OperatorDecayDuration::from_sync(v).format(),
        }
    }

    pub fn operator_decay_volume(index: usize) -> Self {
        let value = OperatorDecayVolume::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} decay volume", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorDecayVolume(
                OperatorDecayVolume::from_sync(v)
            ),
            format: |v| OperatorDecayVolume::from_sync(v).format(),
        }
    }

    pub fn operator_release_duration(index: usize) -> Self {
        let value = OperatorReleaseDuration::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} release time", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| None, // FIXME
            to_processing: |v| ProcessingValue::OperatorReleaseDuration(
                OperatorReleaseDuration::from_sync(v)
            ),
            format: |v| OperatorReleaseDuration::from_sync(v).format(),
        }
    }

    pub fn operator_modulation_target_2(index: usize) -> Self {
        let value = OperatorModulationTarget2::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} mod out", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| {
                if let Ok(value) = value.parse::<usize>(){
                    if value == 1 || value == 2 {
                        return Some(OperatorModulationTarget2(value - 1).to_sync());
                    }
                }

                None
            },
            to_processing: |v| ProcessingValue::OperatorModulationTarget2(
                OperatorModulationTarget2::from_sync(v)
            ),
            format: |v| OperatorModulationTarget2::from_sync(v).format(),
        }
    }

    pub fn operator_modulation_target_3(index: usize) -> Self {
        let value = OperatorModulationTarget3::default().to_sync();

        Self {
            value: AtomicPositiveDouble::new(value),
            name: format!("Op. {} mod out", index + 1),
            unit_from_value: |_| "".to_string(),
            value_from_text: |value| {
                if let Ok(value) = value.parse::<usize>(){
                    if value == 1 || value == 2 || value == 3 {
                        return Some(OperatorModulationTarget3(value - 1).to_sync());
                    }
                }

                None
            },
            to_processing: |v| ProcessingValue::OperatorModulationTarget3(
                OperatorModulationTarget3::from_sync(v)
            ),
            format: |v| OperatorModulationTarget3::from_sync(v).format(),
        }
    }
}


/*
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::common::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_set_volume_text(){
        let p = PresetParameterOperatorVolume::new(3);

        assert!(p.set_parameter_value_text("-1.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 0.0);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1.2".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 1.2);

        assert!(p.set_parameter_value_text("2.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 2.0);

        assert!(p.set_parameter_value_text("3.0".to_string()));
        assert_eq!(PresetParameterOperatorVolume::to_processing(p.get_value()), 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let p = PresetParameterOperatorModulationTarget3::new(3);

        assert!(!p.set_parameter_value_text("abc".to_string()));
        assert!(!p.set_parameter_value_text("0".to_string()));
        assert!(!p.set_parameter_value_text("0.5".to_string()));
        assert!(!p.set_parameter_value_text("4".to_string()));

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 0);

        assert!(p.set_parameter_value_text("2".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 1);

        assert!(p.set_parameter_value_text("3".to_string()));
        assert_eq!(PresetParameterOperatorModulationTarget3::to_processing(p.get_value()), 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let p = PresetParameterOperatorFrequencyRatio::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), OPERATOR_RATIO_STEPS[0]);

        assert!(p.set_parameter_value_text("10000000.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.99".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.5".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        assert!(p.set_parameter_value_text("0.51".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyRatio::to_processing(p.get_value()), 0.5);

        for step in OPERATOR_RATIO_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_frequency_free_text(){
        let p = PresetParameterOperatorFrequencyFree::new(3);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("1".to_string()));
        assert_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS[0]);

        assert!(p.set_parameter_value_text("4.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), 4.0);

        assert!(p.set_parameter_value_text("256.0".to_string()));
        assert_approx_eq!(PresetParameterOperatorFrequencyFree::to_processing(p.get_value()), OPERATOR_FREE_STEPS.last().unwrap());

        for step in OPERATOR_FREE_STEPS.iter() {
            let s = format!("{:.02}", step);
            assert!(p.set_parameter_value_text(s.clone()));
            assert_eq!(p.get_parameter_value_text(), s.clone());
        }
    }

    #[test]
    fn test_set_wave_type_text(){
        let p = PresetParameterOperatorWaveType::new(3);

        assert!(p.set_parameter_value_text("sine".to_string()));
        assert_eq!(PresetParameterOperatorWaveType::to_processing(p.get_value()), WaveType::Sine);

        assert!(p.set_parameter_value_text("noise".to_string()));
        assert_eq!(PresetParameterOperatorWaveType::to_processing(p.get_value()), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = PresetParameterOperatorAttackDuration::new(3);

        assert!(p.set_parameter_value_text("0.0".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()), ENVELOPE_MIN_DURATION);

        assert!(p.set_parameter_value_text("1.0".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()), 1.0);

        assert!(p.set_parameter_value_text("10".to_string()));
        assert_eq!(PresetParameterOperatorAttackDuration::to_processing(p.get_value()),
            ENVELOPE_MAX_DURATION);
    }
}
*/



/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_parameters_len(){
        // Required for ChangedParametersInfo
        assert!(OctaSinePresetParameters::default().len() <= 64);
    }
 
    #[test]
    fn test_load_built_in_presets(){
        use vst2_helpers::presets::PresetBank;

        let preset_bank: PresetBank<OctaSinePresetParameters> =
            crate::built_in_preset_bank();

        // Hopefully prevent compiler from optimizing away code above (if it
        // actually ever did.)
        println!("Dummy info: {}", preset_bank.get_parameter_value_float(0));
    }

    /// Previous format used plain floats for value_float, so we need to check
    /// that (almost) the same values are deserialized no matter the format
    #[test]
    fn test_compare_preset_format_versions(){
        use assert_approx_eq::assert_approx_eq;
        use vst2_helpers::presets::PresetBank;

        let bank_1: PresetBank<OctaSinePresetParameters> = PresetBank::new_from_bytes(
            include_bytes!("../../presets/test-preset-bank-format-1.json")
        );
        let bank_2: PresetBank<OctaSinePresetParameters> = PresetBank::new_from_bytes(
            include_bytes!("../../presets/test-preset-bank-format-2.json")
        );

        assert_eq!(bank_1.len(), bank_2.len());

        for preset_index in 0..bank_1.len(){
            bank_1.set_preset_index(preset_index);
            bank_2.set_preset_index(preset_index);

            assert_eq!(
                bank_1.get_num_parameters(),
                bank_2.get_num_parameters()
            );

            for parameter_index in 0..bank_1.get_num_parameters(){
                assert_approx_eq!(
                    bank_1.get_parameter_value_float(parameter_index),
                    bank_2.get_parameter_value_float(parameter_index),
                    // Accept precision loss (probably due to
                    // JSON/javascript shenanigans)
                    0.0000000000000002
                );
            }
        }
    }

    #[test]
    fn test_export_import(){
        use vst2_helpers::presets::test_helpers::export_import;

        export_import::<OctaSinePresetParameters>();
    }
}
*/