use vst2_helpers::processing_parameters::utils::*;
use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;

use crate::common::*;
use crate::constants::*;

use super::values::*;


pub struct PresetParameter {
    pub value: AtomicPositiveDouble,
    pub name: String,
    pub unit_from_value: fn(f64) -> String,
    pub value_from_text: fn(String) -> Option<f64>,
    pub to_processing: fn(f64) -> ProcessingValue,
    pub format: fn(f64) -> String,
}


impl PresetParameter {
    pub fn set_from_text(&self, text: String) -> bool {
        if let Some(value) = (self.value_from_text)(text){
            self.value.set(value);

            true
        } else {
            false
        }
    }

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
}