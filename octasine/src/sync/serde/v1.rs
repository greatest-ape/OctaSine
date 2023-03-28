use flate2::read::GzDecoder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::utils::get_version_info;

use super::super::patch_bank::Patch;

const PREFIX: &[u8] = b"\n\nOCTASINE-GZ-DATA-V1-BEGIN\n\n";
const SUFFIX: &[u8] = b"\n\nOCTASINE-GZ-DATA-V1-END\n\n";

#[derive(Serialize, Debug)]
pub struct SerdePatchParameterValue(String);

impl SerdePatchParameterValue {
    pub fn from_f32(value: f32) -> Self {
        Self(format!("{:.}", value))
    }

    pub fn as_f32(&self) -> f32 {
        self.0
            .parse()
            .expect("deserialize SerdePresetParameterValue")
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        struct V;

        impl<'de> ::serde::de::Visitor<'de> for V {
            type Value = SerdePatchParameterValue;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("f32 or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(SerdePatchParameterValue(value.to_owned()))
            }
        }

        deserializer.deserialize_any(V)
    }

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdePatchParameter {
    pub name: String,
    #[serde(
        deserialize_with = "SerdePatchParameterValue::deserialize",
        serialize_with = "SerdePatchParameterValue::serialize"
    )]
    pub value_float: SerdePatchParameterValue,
    pub value_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdePatch {
    octasine_version: String,
    pub name: String,
    pub parameters: Vec<SerdePatchParameter>,
}

impl SerdePatch {
    pub fn new(preset: &Patch) -> Self {
        let mut parameters = Vec::new();

        for i in 0..preset.parameters.len() {
            if let Some((_, parameter)) = preset.parameters.get_index(i) {
                let value = parameter.get_value();

                let value_float = SerdePatchParameterValue::from_f32(value);

                parameters.push(SerdePatchParameter {
                    name: parameter.name.to_string(),
                    value_float,
                    value_text: (parameter.format)(value).into(),
                });
            }
        }

        Self {
            octasine_version: get_version_info(),
            name: preset.get_name(),
            parameters,
        }
    }

    pub fn from_bytes<'a>(bytes: &'a [u8]) -> anyhow::Result<Self> {
        Ok(generic_from_bytes(bytes)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: String,
    pub patches: Vec<SerdePatch>,
}

impl SerdePatchBank {
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> anyhow::Result<Self> {
        Ok(generic_from_bytes(bytes)?)
    }
}

pub fn generic_from_bytes<'a, T: DeserializeOwned>(
    mut bytes: &'a [u8],
) -> Result<T, impl ::std::error::Error> {
    bytes = split_off_slice_prefix(bytes, PREFIX);
    bytes = split_off_slice_suffix(bytes, SUFFIX);

    let mut decoder = GzDecoder::new(bytes);

    serde_json::from_reader(&mut decoder)
}

fn split_off_slice_suffix<'a>(mut bytes: &'a [u8], suffix: &[u8]) -> &'a [u8] {
    if let Some(index) = find_in_slice(bytes, suffix) {
        bytes = &bytes[..index];
    }

    bytes
}

fn split_off_slice_prefix<'a>(mut bytes: &'a [u8], prefix: &[u8]) -> &'a [u8] {
    if let Some(index) = find_in_slice(bytes, prefix) {
        bytes = &bytes[index + prefix.len()..];
    }

    bytes
}

fn find_in_slice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }

    for (i, window) in haystack.windows(needle.len()).enumerate() {
        if window == needle {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_off_slice_prefix() {
        assert_eq!(split_off_slice_prefix(b"abcdef", b"abc"), b"def");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"bcd"), b"ef");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"def"), b"");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"abcdef"), b"");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"abcdefg"), b"abcdef");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"z"), b"abcdef");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"zzzzzz"), b"abcdef");
        assert_eq!(split_off_slice_prefix(b"abcdef", b"zzzzzzz"), b"abcdef");
        assert_eq!(split_off_slice_prefix(b"abcdef", b""), b"abcdef");
        assert_eq!(split_off_slice_prefix(b"", b""), b"");
    }
}
