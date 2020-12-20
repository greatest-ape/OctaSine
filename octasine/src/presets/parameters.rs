use vst2_helpers::utils::atomic_double::AtomicPositiveDouble;

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
}