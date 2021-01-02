use crate::preset_bank::SyncParameter;

use super::values::*;


pub fn create_parameters() -> Vec<SyncParameter> {
    vec![
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

        // LFO 1
        lfo_target_parameter(0),
        lfo_shape(0),
        lfo_mode(0),
        lfo_bpm_sync(0),
        lfo_speed(0),
        lfo_magnitude(0),
    ]
}


fn master_volume() -> SyncParameter {
    SyncParameter::new(
        "Master volume",
        MasterVolume::default()
    )
}

fn master_frequency() -> SyncParameter {
    SyncParameter::new(
        "Master frequency",
        MasterFrequency::default()
    )
}

fn operator_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} volume", index + 1),
        OperatorVolume::new(index)
    )
}

fn operator_panning(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} pan", index + 1),
        OperatorPanning::default()
    )
}

fn operator_additive(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} additive", index + 1),
        OperatorAdditive::default()
    )
}

fn operator_frequency_ratio(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq ratio", index + 1),
        OperatorFrequencyRatio::default()
    )
}

fn operator_frequency_free(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq free", index + 1),
        OperatorFrequencyFree::default()
    )
}

fn operator_frequency_fine(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} freq fine", index + 1),
        OperatorFrequencyFine::default()
    )
}

fn operator_feedback(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} feedback", index + 1),
        OperatorFeedback::default()
    )
}

fn operator_modulation_index(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} mod index", index + 1),
        OperatorModulationIndex::default()
    )
}

fn operator_wave_type(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} wave", index + 1),
        OperatorWaveType::default()
    )
}

fn operator_attack_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} attack time", index + 1),
        OperatorAttackDuration::default()
    )
}

fn operator_attack_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} attack vol", index + 1),
        OperatorAttackVolume::default()
    )
}

fn operator_decay_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} decay time", index + 1),
        OperatorDecayDuration::default()
    )
}

fn operator_decay_volume(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} decay vol", index + 1),
        OperatorDecayVolume::default()
    )
}

fn operator_release_duration(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("Op. {} release time", index + 1),
        OperatorReleaseDuration::default()
    )
}

fn operator_modulation_target_2() -> SyncParameter {
    SyncParameter::new(
        "Op. 2 mod out",
        OperatorModulationTarget2::default()
    )
}

fn operator_modulation_target_3() -> SyncParameter {
    SyncParameter::new(
        "Op. 3 mod out",
        OperatorModulationTarget3::default()
    )
}

fn lfo_target_parameter(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} target", index + 1),
        LfoTargetParameterValue::default()
    )
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

fn lfo_speed(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} speed", index + 1),
        LfoSpeedValue::default()
    )
}

fn lfo_magnitude(index: usize) -> SyncParameter {
    SyncParameter::new(
        &format!("LFO {} magnitude", index + 1),
        LfoMagnitudeValue::default()
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
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("0".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 0.0);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("1.2".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 1.2);

        assert!(p.set_from_text("2.0".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 2.0);

        assert!(p.set_from_text("3.0".to_string()));
        assert_eq!(OperatorVolume::from_sync(p.get_value()).get(), 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let p = operator_modulation_target_3();

        assert!(!p.set_from_text("abc".to_string()));
        assert!(!p.set_from_text("0".to_string()));
        assert!(!p.set_from_text("0.5".to_string()));
        assert!(!p.set_from_text("4".to_string()));

        assert!(p.set_from_text("1".to_string()));
        assert_eq!(OperatorModulationTarget3::from_sync(p.get_value()).get(), 0);

        assert!(p.set_from_text("2".to_string()));
        assert_eq!(OperatorModulationTarget3::from_sync(p.get_value()).get(), 1);

        assert!(p.set_from_text("3".to_string()));
        assert_eq!(OperatorModulationTarget3::from_sync(p.get_value()).get(), 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let p = operator_frequency_ratio(3);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), OPERATOR_RATIO_STEPS[0]);

        assert!(p.set_from_text("10000000.0".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("0.99".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("0.5".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), 0.5);

        assert!(p.set_from_text("0.51".to_string()));
        assert_eq!(OperatorFrequencyRatio::from_sync(p.get_value()).get(), 0.5);

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
        assert_eq!(OperatorWaveType::from_sync(p.get_value()).get(), WaveType::Sine);

        assert!(p.set_from_text("noise".to_string()));
        assert_eq!(OperatorWaveType::from_sync(p.get_value()).get(), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = operator_attack_duration(3);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(), ENVELOPE_MIN_DURATION);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("10".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(),
            ENVELOPE_MAX_DURATION);
    }
}