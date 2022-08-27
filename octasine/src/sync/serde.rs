use std::{io::Read, iter::repeat, path::Path};

use byteorder::{BigEndian, WriteBytesExt};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{crate_version, crate_version_to_vst_format, PLUGIN_UNIQUE_ID};

use super::patch_bank::{Patch, PatchBank};

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
            if let Some(parameter) = preset.parameters.get(i) {
                let value = parameter.get_value();

                let value_float = SerdePatchParameterValue::from_f32(value);

                parameters.push(SerdePatchParameter {
                    name: parameter.name.clone(),
                    value_float,
                    value_text: (parameter.format)(value),
                });
            }
        }

        Self {
            octasine_version: crate::get_version_info(),
            name: preset.get_name(),
            parameters,
        }
    }

    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        to_bytes(self)
    }

    pub fn to_fxp_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let patch_bytes = to_bytes(self)?;

        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"CcnK"); // fxp/fxp identifier
        bytes.write_i32::<BigEndian>((5 * 4 + 28 + 4 + patch_bytes.len()).try_into()?)?;

        bytes.extend_from_slice(b"FPCh"); // fxp opaque chunk
        bytes.write_i32::<BigEndian>(1)?; // fxp version
        bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_ID)?;
        bytes.write_i32::<BigEndian>(crate_version_to_vst_format(crate_version!()))?;

        bytes.write_i32::<BigEndian>(self.parameters.len().try_into()?)?;

        let name_buf = {
            let mut name_buf = [0u8; 28];
            let mut chars_written = 0;

            for (b, c) in name_buf.iter_mut().zip(self.name.chars()) {
                if c.is_ascii() {
                    *b = c as u8;
                } else {
                    *b = b' ';
                }

                chars_written += 1;
            }

            name_buf[chars_written.min(name_buf.len() - 1)] = 0;

            name_buf
        };

        bytes.extend_from_slice(&name_buf);

        bytes.write_i32::<BigEndian>(patch_bytes.len().try_into()?)?;
        bytes.extend_from_slice(&patch_bytes);

        Ok(bytes)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: String,
    pub patches: Vec<SerdePatch>,
}

impl SerdePatchBank {
    pub fn new(patch_bank: &PatchBank) -> Self {
        Self {
            octasine_version: crate::get_version_info(),
            patches: patch_bank
                .patches
                .iter()
                .map(Patch::export_serde_preset)
                .collect(),
        }
    }

    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        to_bytes(self)
    }

    pub fn to_fxb_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let bank_bytes = to_bytes(self)?;

        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"CcnK"); // fxp/fxp identifier
        bytes.write_i32::<BigEndian>((5 * 4 + 128 + 4 + bank_bytes.len()).try_into()?)?;

        bytes.extend_from_slice(b"FBCh"); // fxb opaque chunk
        bytes.write_i32::<BigEndian>(1)?; // fxb version (1 or 2)
        bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_ID)?;
        bytes.write_i32::<BigEndian>(crate_version_to_vst_format(crate_version!()))?;

        bytes.write_i32::<BigEndian>(self.patches.len().try_into()?)?;
        bytes.extend(repeat(0).take(128)); // reserved padding for fxb version 1

        bytes.write_i32::<BigEndian>(bank_bytes.len().try_into()?)?;
        bytes.extend_from_slice(&bank_bytes);

        Ok(bytes)
    }
}

fn to_bytes<T: Serialize>(t: &T) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::from(PREFIX);

    let mut encoder = GzEncoder::new(&mut bytes, Compression::best());

    serde_json::to_writer(&mut encoder, t)?;

    encoder.finish()?;

    bytes.extend_from_slice(&SUFFIX);

    Ok(bytes)
}

pub fn from_bytes<'a, T: DeserializeOwned>(
    mut bytes: &'a [u8],
) -> Result<T, impl ::std::error::Error> {
    bytes = split_off_slice_prefix(bytes, PREFIX);
    bytes = split_off_slice_suffix(bytes, SUFFIX);

    let mut decoder = GzDecoder::new(bytes);

    serde_json::from_reader(&mut decoder)
}

pub fn from_path<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let mut file = ::std::fs::File::open(path)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    match from_bytes(&bytes) {
        Ok(t) => Ok(t),
        Err(err) => Err(anyhow::anyhow!("deserialize patch/preset: {}", err)),
    }
}

fn split_off_slice_prefix<'a>(mut bytes: &'a [u8], prefix: &[u8]) -> &'a [u8] {
    if let Some(index) = find_in_slice(bytes, prefix) {
        bytes = &bytes[index + prefix.len()..];
    }

    bytes
}

fn split_off_slice_suffix<'a>(mut bytes: &'a [u8], suffix: &[u8]) -> &'a [u8] {
    if let Some(index) = find_in_slice(bytes, suffix) {
        bytes = &bytes[..index];
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
