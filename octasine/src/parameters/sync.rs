use crate::{constants::NUM_LFOS, preset_bank::SyncParameter};

use super::values::*;


pub fn create_parameters() -> Vec<SyncParameter> {
    let mut parameters = vec![
        master_volume(),
        master_frequency(),

        // Operator 1
        operator_volume(0),
        operator_panning(0),
        operator_wave_type(0),
        operator_modulation_index(0),
        operator_feedback(0),
        operator_frequency_ratio(0),
        operator_frequency_free(0),
        operator_frequency_fine(0),
        operator_attack_duration(0),
        operator_attack_volume(0),
        operator_decay_duration(0),
        operator_decay_volume(0),
        operator_release_duration(0),

        // Operator 2
        operator_volume(1),
        operator_panning(1),
        operator_wave_type(1),
        operator_additive(1),
        operator_modulation_index(1),
        operator_feedback(1),
        operator_frequency_ratio(1),
        operator_frequency_free(1),
        operator_frequency_fine(1),
        operator_attack_duration(1),
        operator_attack_volume(1),
        operator_decay_duration(1),
        operator_decay_volume(1),
        operator_release_duration(1),

        // Operator 3
        operator_volume(2),
        operator_panning(2),
        operator_wave_type(2),
        operator_additive(2),
        operator_modulation_target_2(),
        operator_modulation_index(2),
        operator_feedback(2),
        operator_frequency_ratio(2),
        operator_frequency_free(2),
        operator_frequency_fine(2),
        operator_attack_duration(2),
        operator_attack_volume(2),
        operator_decay_duration(2),
        operator_decay_volume(2),
        operator_release_duration(2),

        // Operator 4
        operator_volume(3),
        operator_panning(3),
        operator_wave_type(3),
        operator_additive(3),
        operator_modulation_target_3(),
        operator_modulation_index(3),
        operator_feedback(3),
        operator_frequency_ratio(3),
        operator_frequency_free(3),
        operator_frequency_fine(3),
        operator_attack_duration(3),
        operator_attack_volume(3),
        operator_decay_duration(3),
        operator_decay_volume(3),
        operator_release_duration(3),
    ];

    for lfo_index in 0..NUM_LFOS {
        parameters.extend(vec![
            lfo_target_parameter(lfo_index),
            lfo_bpm_sync(lfo_index),
            lfo_frequency_ratio(lfo_index),
            lfo_frequency_free(lfo_index),
            lfo_mode(lfo_index),
            lfo_shape(lfo_index),
            lfo_amount(lfo_index),
        ])
    }

    parameters
}


fn master_volume() -> SyncParameter {
    SyncParameter::new(
        "Master volume",
        MasterVolumeValue::default()
    )
}

fn master_frequency() -> SyncParameter {
    SyncParameter::new(
        "Master frequency",
        MasterFrequencyValue::default()
    )
}

fn operator_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} volume", index + 1),
        OperatorVolumeValue::new(index)
    )
}

fn operator_panning(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} pan", index + 1),
        OperatorPanningValue::default()
    )
}

fn operator_additive(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} additive", index + 1),
        OperatorAdditiveValue::default()
    )
}

fn operator_frequency_ratio(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq ratio", index + 1),
        OperatorFrequencyRatioValue::default()
    )
}

fn operator_frequency_free(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq free", index + 1),
        OperatorFrequencyFreeValue::default()
    )
}

fn operator_frequency_fine(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq fine", index + 1),
        OperatorFrequencyFineValue::default()
    )
}

fn operator_feedback(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} feedback", index + 1),
        OperatorFeedbackValue::default()
    )
}

fn operator_modulation_index(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} mod index", index + 1),
        OperatorModulationIndexValue::default()
    )
}

fn operator_wave_type(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} wave", index + 1),
        OperatorWaveTypeValue::default()
    )
}

fn operator_attack_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} attack time", index + 1),
        OperatorAttackDurationValue::default()
    )
}

fn operator_attack_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} attack vol", index + 1),
        OperatorAttackVolumeValue::default()
    )
}

fn operator_decay_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} decay time", index + 1),
        OperatorDecayDurationValue::default()
    )
}

fn operator_decay_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} decay vol", index + 1),
        OperatorDecayVolumeValue::default()
    )
}

fn operator_release_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} release time", index + 1),
        OperatorReleaseDurationValue::default()
    )
}

fn operator_modulation_target_2() -> SyncParameter {
    SyncParameter::new(
        "Op. 3 mod out",
        Operator3ModulationTargetValue::default()
    )
}

fn operator_modulation_target_3() -> SyncParameter {
    SyncParameter::new(
        "Op. 4 mod out",
        Operator4ModulationTargetValue::default()
    )
}

fn lfo_target_parameter(index: usize) -> SyncParameter {
    let title = format!("LFO {} target", index + 1);

    match index {
        0 => SyncParameter::new(&title, Lfo1TargetParameterValue::default()),
        1 => SyncParameter::new(&title, Lfo2TargetParameterValue::default()),
        2 => SyncParameter::new(&title, Lfo3TargetParameterValue::default()),
        3 => SyncParameter::new(&title, Lfo4TargetParameterValue::default()),
        _ => unreachable!(),
    }
}

fn lfo_shape(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} shape", index + 1),
        LfoShapeValue::default()
    )
}

fn lfo_mode(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} mode", index + 1),
        LfoModeValue::default()
    )
}

fn lfo_bpm_sync(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} bpm sync", index + 1),
        LfoBpmSyncValue::default()
    )
}

fn lfo_frequency_ratio(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} freq ratio", index + 1),
        LfoFrequencyRatioValue::default()
    )
}

fn lfo_frequency_free(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} freq free", index + 1),
        LfoFrequencyFreeValue::default()
    )
}

fn lfo_amount(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} amount", index + 1),
        LfoAmountValue::default()
    )
}


#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    // use assert_approx_eq::assert_approx_eq;

    use crate::common::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_preset_parameters_len(){
        assert!(create_parameters().len() <= crate::preset_bank::MAX_NUM_PARAMETERS);
    }

    #[test]
    fn test_set_volume_text(){
        let p = operator_volume(3);

        assert!(p.set_from_text("-1.0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("1.2".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 1.2);

        assert!(p.set_from_text("2.0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 2.0);

        assert!(p.set_from_text("3.0".to_string()));
        assert_eq!(OperatorVolumeValue::from_sync(p.get_value()).get(), 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let p = operator_modulation_target_3();

        assert!(!p.set_from_text("abc".to_string()));
        assert!(!p.set_from_text("0".to_string()));
        assert!(!p.set_from_text("0.5".to_string()));
        assert!(!p.set_from_text("4".to_string()));

        assert!(p.set_from_text("1".to_string()));
        assert_eq!(Operator4ModulationTargetValue::from_sync(p.get_value()).get(), 0);

        assert!(p.set_from_text("2".to_string()));
        assert_eq!(Operator4ModulationTargetValue::from_sync(p.get_value()).get(), 1);

        assert!(p.set_from_text("3".to_string()));
        assert_eq!(Operator4ModulationTargetValue::from_sync(p.get_value()).get(), 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let p = operator_frequency_ratio(3);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), OPERATOR_RATIO_STEPS[0]);

        assert!(p.set_from_text("10000000.0".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("0.99".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("0.5".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), 0.5);

        assert!(p.set_from_text("0.51".to_string()));
        assert_eq!(OperatorFrequencyRatioValue::from_sync(p.get_value()).get(), 0.5);

        for step in OPERATOR_RATIO_STEPS.iter() {
            let s = format!("{:.04}", step);
            assert!(p.set_from_text(s.clone()));
            assert_eq!(p.get_value_text(), s.clone());
        }
    }

    /*
    #[test]
    fn test_set_frequency_free_text(){
        let p = operator_frequency_free(3);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorFrequencyFree::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("1".to_string()));
        assert_eq!(OperatorFrequencyFree::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("0.0".to_string()));
        assert_approx_eq!(OperatorFrequencyFree::from_sync(p.get_value()).get(), OPERATOR_FREE_STEPS[0]);

        assert!(p.set_from_text("4.0".to_string()));
        assert_approx_eq!(OperatorFrequencyFree::from_sync(p.get_value()).get(), 4.0);

        assert!(p.set_from_text("256.0".to_string()));
        assert_approx_eq!(OperatorFrequencyFree::from_sync(p.get_value()).get(), OPERATOR_FREE_STEPS.last().unwrap());

        for step in OPERATOR_FREE_STEPS.iter() {
            let s = format!("{:.04}", step);
            assert!(p.set_from_text(s.clone()));
            assert_eq!(p.get_value_text(), s.clone());
        }
    }
    */

    #[test]
    fn test_set_wave_type_text(){
        let p = operator_wave_type(3);

        assert!(p.set_from_text("sine".to_string()));
        assert_eq!(OperatorWaveTypeValue::from_sync(p.get_value()).get(), WaveType::Sine);

        assert!(p.set_from_text("noise".to_string()));
        assert_eq!(OperatorWaveTypeValue::from_sync(p.get_value()).get(), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = operator_attack_duration(3);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorAttackDurationValue::from_sync(p.get_value()).get(), ENVELOPE_MIN_DURATION);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorAttackDurationValue::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("10".to_string()));
        assert_eq!(OperatorAttackDurationValue::from_sync(p.get_value()).get(),
            ENVELOPE_MAX_DURATION);
    }
}