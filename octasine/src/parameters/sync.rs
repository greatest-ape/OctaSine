use crate::preset_bank::SyncParameter;

use super::values::*;


pub fn create_parameters() -> Vec<SyncParameter> {
    vec![
        SyncParameter::master_volume(),
        SyncParameter::master_frequency(),

        // Operator 1
        SyncParameter::operator_volume(0),
        SyncParameter::operator_panning(0),
        SyncParameter::operator_wave_type(0),
        SyncParameter::operator_modulation_index(0),
        SyncParameter::operator_feedback(0),
        SyncParameter::operator_frequency_ratio(0),
        SyncParameter::operator_frequency_free(0),
        SyncParameter::operator_frequency_fine(0),
        SyncParameter::operator_attack_duration(0),
        SyncParameter::operator_attack_volume(0),
        SyncParameter::operator_decay_duration(0),
        SyncParameter::operator_decay_volume(0),
        SyncParameter::operator_release_duration(0),

        // Operator 2
        SyncParameter::operator_volume(1),
        SyncParameter::operator_panning(1),
        SyncParameter::operator_wave_type(1),
        SyncParameter::operator_additive(1),
        SyncParameter::operator_modulation_index(1),
        SyncParameter::operator_feedback(1),
        SyncParameter::operator_frequency_ratio(1),
        SyncParameter::operator_frequency_free(1),
        SyncParameter::operator_frequency_fine(1),
        SyncParameter::operator_attack_duration(1),
        SyncParameter::operator_attack_volume(1),
        SyncParameter::operator_decay_duration(1),
        SyncParameter::operator_decay_volume(1),
        SyncParameter::operator_release_duration(1),

        // Operator 3
        SyncParameter::operator_volume(2),
        SyncParameter::operator_panning(2),
        SyncParameter::operator_wave_type(2),
        SyncParameter::operator_additive(2),
        SyncParameter::operator_modulation_target_2(),
        SyncParameter::operator_modulation_index(2),
        SyncParameter::operator_feedback(2),
        SyncParameter::operator_frequency_ratio(2),
        SyncParameter::operator_frequency_free(2),
        SyncParameter::operator_frequency_fine(2),
        SyncParameter::operator_attack_duration(2),
        SyncParameter::operator_attack_volume(2),
        SyncParameter::operator_decay_duration(2),
        SyncParameter::operator_decay_volume(2),
        SyncParameter::operator_release_duration(2),

        // Operator 4
        SyncParameter::operator_volume(3),
        SyncParameter::operator_panning(3),
        SyncParameter::operator_wave_type(3),
        SyncParameter::operator_additive(3),
        SyncParameter::operator_modulation_target_3(),
        SyncParameter::operator_modulation_index(3),
        SyncParameter::operator_feedback(3),
        SyncParameter::operator_frequency_ratio(3),
        SyncParameter::operator_frequency_free(3),
        SyncParameter::operator_frequency_fine(3),
        SyncParameter::operator_attack_duration(3),
        SyncParameter::operator_attack_volume(3),
        SyncParameter::operator_decay_duration(3),
        SyncParameter::operator_decay_volume(3),
        SyncParameter::operator_release_duration(3),
    ]
}


impl SyncParameter {
    pub fn master_volume() -> Self {
        Self::new(
            "Master volume",
            MasterVolume::default()
        )
    }

    pub fn master_frequency() -> Self {
        Self::new(
            "Master frequency",
            MasterFrequency::default()
        )
    }

    pub fn operator_volume(index: usize) -> Self {
        Self::new(
            &format!("Op. {} volume", index + 1),
            OperatorVolume::new(index)
        )
    }

    pub fn operator_panning(index: usize) -> Self {
        Self::new(
            &format!("Op. {} pan", index + 1),
            OperatorPanning::default()
        )
    }

    pub fn operator_additive(index: usize) -> Self {
        Self::new(
            &format!("Op. {} additive", index + 1),
            OperatorAdditive::default()
        )
    }

    pub fn operator_frequency_ratio(index: usize) -> Self {
        Self::new(
            &format!("Op. {} freq ratio", index + 1),
            OperatorFrequencyRatio::default()
        )
    }

    pub fn operator_frequency_free(index: usize) -> Self {
        Self::new(
            &format!("Op. {} freq free", index + 1),
            OperatorFrequencyFree::default()
        )
    }

    pub fn operator_frequency_fine(index: usize) -> Self {
        Self::new(
            &format!("Op. {} freq fine", index + 1),
            OperatorFrequencyFine::default()
        )
    }

    pub fn operator_feedback(index: usize) -> Self {
        Self::new(
            &format!("Op. {} feedback", index + 1),
            OperatorFeedback::default()
        )
    }

    pub fn operator_modulation_index(index: usize) -> Self {
        Self::new(
            &format!("Op. {} mod index", index + 1),
            OperatorModulationIndex::default()
        )
    }

    pub fn operator_wave_type(index: usize) -> Self {
        Self::new(
            &format!("Op. {} wave", index + 1),
            OperatorWaveType::default()
        )
    }

    pub fn operator_attack_duration(index: usize) -> Self {
        Self::new(
            &format!("Op. {} attack time", index + 1),
            OperatorAttackDuration::default()
        )
    }

    pub fn operator_attack_volume(index: usize) -> Self {
        Self::new(
            &format!("Op. {} attack vol", index + 1),
            OperatorAttackVolume::default()
        )
    }

    pub fn operator_decay_duration(index: usize) -> Self {
        Self::new(
            &format!("Op. {} decay time", index + 1),
            OperatorDecayDuration::default()
        )
    }

    pub fn operator_decay_volume(index: usize) -> Self {
        Self::new(
            &format!("Op. {} decay vol", index + 1),
            OperatorDecayVolume::default()
        )
    }

    pub fn operator_release_duration(index: usize) -> Self {
        Self::new(
            &format!("Op. {} release time", index + 1),
            OperatorReleaseDuration::default()
        )
    }

    pub fn operator_modulation_target_2() -> Self {
        Self::new(
            "Op. 2 mod out",
            OperatorModulationTarget2::default()
        )
    }

    pub fn operator_modulation_target_3() -> Self {
        Self::new(
            "Op. 3 mod out",
            OperatorModulationTarget3::default()
        )
    }
}


// FIXME: these should be rewritten and moved to values.rs
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    // use assert_approx_eq::assert_approx_eq;

    use crate::common::*;
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_preset_parameters_len(){
        // Required for ChangedParametersInfo
        assert!(create_parameters().len() <= 64);
    }

    #[test]
    fn test_set_volume_text(){
        let p = SyncParameter::operator_volume(3);

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
        let p = SyncParameter::operator_modulation_target_3();

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
        let p = SyncParameter::operator_frequency_ratio(3);

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
            let s = format!("{:.02}", step);
            assert!(p.set_from_text(s.clone()));
            assert_eq!(p.get_value_text(), s.clone());
        }
    }

    /*
    #[test]
    fn test_set_frequency_free_text(){
        let p = SyncParameter::operator_frequency_free(3);

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
            let s = format!("{:.02}", step);
            assert!(p.set_from_text(s.clone()));
            assert_eq!(p.get_value_text(), s.clone());
        }
    }
    */

    #[test]
    fn test_set_wave_type_text(){
        let p = SyncParameter::operator_wave_type(3);

        assert!(p.set_from_text("sine".to_string()));
        assert_eq!(OperatorWaveType::from_sync(p.get_value()).get(), WaveType::Sine);

        assert!(p.set_from_text("noise".to_string()));
        assert_eq!(OperatorWaveType::from_sync(p.get_value()).get(), WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let p = SyncParameter::operator_attack_duration(3);

        assert!(p.set_from_text("0.0".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(), ENVELOPE_MIN_DURATION);

        assert!(p.set_from_text("1.0".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(), 1.0);

        assert!(p.set_from_text("10".to_string()));
        assert_eq!(OperatorAttackDuration::from_sync(p.get_value()).get(),
            ENVELOPE_MAX_DURATION);
    }
}