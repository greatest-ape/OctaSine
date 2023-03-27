use std::{io::Read, iter::repeat, path::Path};

use anyhow::Context;
use byteorder::{BigEndian, WriteBytesExt};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    crate_version,
    plugin::common::{crate_version_to_vst2_format, PLUGIN_UNIQUE_VST2_ID},
    utils::get_version_info,
};

use super::{
    compat::run_patch_compatibility_changes,
    patch_bank::{Patch, PatchBank},
};

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
    /// Optional because added in v0.8.4 (FIXME)
    octasine_full_semver_version: Option<String>,
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
                    name: parameter.name.clone(),
                    value_float,
                    value_text: (parameter.format)(value),
                });
            }
        }

        Self {
            octasine_version: get_version_info(),
            octasine_full_semver_version: Some(env!("CARGO_PKG_VERSION").into()),
            name: preset.get_name(),
            parameters,
        }
    }

    pub fn get_semver_version(&self) -> Result<Version, semver::Error> {
        if let Some(v) = self.octasine_full_semver_version.as_ref() {
            Version::parse(v)
        } else {
            let mut v = self.octasine_version.chars();

            // Drop leading "v"
            let _ = v.next();

            let v: String = v.take_while(|c| !c.is_whitespace()).collect();

            Version::parse(&v)
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
        bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_VST2_ID)?;
        bytes.write_i32::<BigEndian>(crate_version_to_vst2_format(crate_version!()))?;

        bytes.write_i32::<BigEndian>(self.parameters.len().try_into()?)?;

        let name_buf = {
            let mut buf = [0u8; 28];

            // Iterate through all buffer items except last, where a null
            // terminator must be left in place. If there are less than 27
            // chars, the last one will automatically be followed by a null
            // byte.
            for (b, c) in buf[..27].iter_mut().zip(
                self.name
                    .chars()
                    .into_iter()
                    .filter_map(|c| c.is_ascii().then_some(c as u8)),
            ) {
                *b = c;
            }

            buf
        };

        bytes.extend_from_slice(&name_buf);

        bytes.write_i32::<BigEndian>(patch_bytes.len().try_into()?)?;
        bytes.extend_from_slice(&patch_bytes);

        Ok(bytes)
    }

    pub fn from_bytes<'a>(bytes: &'a [u8]) -> anyhow::Result<Self> {
        let mut patch = generic_from_bytes(bytes)?;

        run_patch_compatibility_changes(&mut patch)?;

        Ok(patch)
    }

    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let mut file = ::std::fs::File::open(path)?;
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)?;

        Self::from_bytes(&bytes).with_context(|| "deserialize patch")
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: String,
    /// Optional because added in v0.8.4 (FIXME)
    octasine_full_semver_version: Option<String>,
    pub patches: Vec<SerdePatch>,
}

impl SerdePatchBank {
    pub fn new(patch_bank: &PatchBank) -> Self {
        Self {
            octasine_version: get_version_info(),
            octasine_full_semver_version: Some(env!("CARGO_PKG_VERSION").into()),
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
        bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_VST2_ID)?;
        bytes.write_i32::<BigEndian>(crate_version_to_vst2_format(crate_version!()))?;

        bytes.write_i32::<BigEndian>(self.patches.len().try_into()?)?;
        bytes.extend(repeat(0).take(128)); // reserved padding for fxb version 1

        bytes.write_i32::<BigEndian>(bank_bytes.len().try_into()?)?;
        bytes.extend_from_slice(&bank_bytes);

        Ok(bytes)
    }

    pub fn from_bytes<'a>(bytes: &'a [u8]) -> anyhow::Result<Self> {
        let mut bank: Self = generic_from_bytes(bytes)?;

        bank.run_compatibility_changes()?;

        Ok(bank)
    }

    pub fn run_compatibility_changes(&mut self) -> anyhow::Result<()> {
        for patch in self.patches.iter_mut() {
            run_patch_compatibility_changes(patch)?;
        }

        Ok(())
    }

    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let mut file = ::std::fs::File::open(path)?;
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)?;

        Self::from_bytes(&bytes).with_context(|| "deserialize patch")
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

/// Does NOT perform compatibility changes
pub fn generic_from_bytes<'a, T: DeserializeOwned>(
    mut bytes: &'a [u8],
) -> Result<T, impl ::std::error::Error> {
    bytes = split_off_slice_prefix(bytes, PREFIX);
    bytes = split_off_slice_suffix(bytes, SUFFIX);

    let mut decoder = GzDecoder::new(bytes);

    serde_json::from_reader(&mut decoder)
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
