use serde::{Serialize, Deserialize};

use super::{Preset, PresetBank};


#[derive(Serialize, Debug)]
pub(super) struct SerdePresetParameterValue(
    String
);


impl SerdePresetParameterValue {
    pub fn from_f64(value: f64) -> Self {
        Self(format!("{:.}", value))
    }

    pub fn as_f64(&self) -> f64 {
        self.0.parse().expect("deserialize SerdePresetParameterValue")
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
        where D: ::serde::de::Deserializer<'de>,
    {
        struct V;

        impl<'de> ::serde::de::Visitor<'de> for V {
            type Value = SerdePresetParameterValue;

            fn expecting(
                &self,
                formatter: &mut ::std::fmt::Formatter
            ) -> ::std::fmt::Result {
                formatter.write_str("f64 or string")
            }

            fn visit_str<E>(
                self,
                value: &str
            ) -> Result<Self::Value, E> where E: ::serde::de::Error {
                Ok(SerdePresetParameterValue(value.to_owned()))
            }

            // Backwards compatibility with f64
            fn visit_f64<E>(
                self,
                value: f64
            ) -> Result<Self::Value, E> where E: ::serde::de::Error {
                Ok(SerdePresetParameterValue::from_f64(value))
            }
        }

        deserializer.deserialize_any(V)
    }

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::ser::Serializer
    {
        serializer.serialize_str(&self.0)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct SerdePresetParameter {
    pub(super) name: String,
    #[serde(
        deserialize_with = "SerdePresetParameterValue::deserialize",
        serialize_with = "SerdePresetParameterValue::serialize",
    )]
    pub(super) value_float: SerdePresetParameterValue,
    pub(super) value_text: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub(super) struct SerdePreset {
    pub(super) name: String,
    pub(super) parameters: Vec<SerdePresetParameter>,
}


impl SerdePreset {
    pub(super) fn new(preset: &Preset) -> Self {
        let mut parameters = Vec::new();

        for i in 0..preset.parameters.len(){
            if let Some(parameter) = preset.parameters.get(i){
                let value = parameter.value.get();

                let value_float = SerdePresetParameterValue::from_f64(
                    value
                );

                parameters.push(SerdePresetParameter {
                    name: parameter.name.clone(),
                    value_float,
                    value_text: (parameter.format_sync)(value),
                });
            }
        }

        Self {
            name: preset.get_name(),
            parameters,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub(super) struct SerdePresetBank {
    pub(super) presets: Vec<SerdePreset>,
}


impl SerdePresetBank {
    pub(super) fn new(preset_bank: &PresetBank) -> Self {
        Self {
            presets: preset_bank.presets.iter()
                .map(Preset::export_serde_preset)
                .collect()
        }
    }
}