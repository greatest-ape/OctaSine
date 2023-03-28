use std::io::Write;

use compact_str::CompactString;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    common::IndexMap,
    parameters::{OperatorParameter, Parameter, ParameterKey},
    sync::patch_bank::{Patch, PatchBank},
};

use super::common::{make_fxb, make_fxp};

const PREFIX_PLAIN: &[u8] = b"\n\nOCTASINE-DATA-V2-PLAIN\n\n";
const PREFIX_GZ: &[u8] = b"\n\nOCTASINE-DATA-V2-GZ\n\n";

const COMPATIBILITY_CHANGES: &[(Version, fn(&mut SerdePatch))] = &[
    // (Version::new(0, 8, 5), compat_0_8_5),
];

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: [u64; 3],
    pub patches: Vec<SerdePatch>,
}

impl SerdePatchBank {
    pub fn new(bank: &PatchBank) -> Self {
        let patches = bank.patches.iter().map(SerdePatch::new).collect();

        Self {
            octasine_version: get_octasine_version(),
            patches,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut bank: Self = deserialize_bytes(bytes)?;

        for patch in bank.patches.iter_mut() {
            patch.run_compatibility_changes();
        }

        Ok(bank)
    }

    pub fn serialize_plain<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        serialize_bytes_plain(writer, self)
    }

    pub fn to_fxb_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        serialize_bytes_gz(&mut buffer, self)?;

        make_fxb(&buffer, self.patches.len())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatch {
    octasine_version: [u64; 3],
    pub name: CompactString,
    pub parameters: IndexMap<ParameterKey, SerdePatchParameter>,
}

impl SerdePatch {
    pub fn new(patch: &Patch) -> Self {
        let parameters = patch
            .parameters
            .iter()
            .enumerate()
            .map(|(i, (k, p))| {
                let parameter = SerdePatchParameter {
                    name: p.name.as_str().into(),
                    index: i,
                    value_f32: p.get_value(),
                    value_string: p.get_value_text().into(),
                };

                (*k, parameter)
            })
            .collect();

        Self {
            octasine_version: get_octasine_version(),
            name: patch.get_name().into(),
            parameters,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut patch: Self = deserialize_bytes(bytes)?;

        patch.run_compatibility_changes();

        Ok(patch)
    }

    pub fn to_fxp_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        serialize_bytes_gz(&mut buffer, self)?;

        make_fxp(&buffer, &self.name, self.parameters.len())
    }

    fn run_compatibility_changes(&mut self) {
        let patch_version = {
            let [major, minor, patch] = self.octasine_version;

            Version::new(major, minor, patch)
        };

        for (changed_in_version, f) in COMPATIBILITY_CHANGES {
            if patch_version < *changed_in_version {
                f(self);
            } else {
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchParameter {
    name: CompactString,
    index: usize,
    pub value_f32: f32,
    value_string: CompactString,
}

pub fn bytes_are_v2(bytes: &[u8]) -> bool {
    memchr::memmem::find(bytes, PREFIX_PLAIN).is_some()
        || memchr::memmem::find(bytes, PREFIX_GZ).is_some()
}

fn get_octasine_version() -> [u64; 3] {
    let version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();

    [version.major, version.minor, version.patch]
}

fn deserialize_bytes<'a, T>(bytes: &'a [u8]) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
{
    if let Some(offset) = memchr::memmem::find(bytes, PREFIX_PLAIN) {
        let bytes = &bytes[offset + PREFIX_PLAIN.len()..];

        Ok(bincode::deserialize(bytes)?)
    } else if let Some(offset) = memchr::memmem::find(bytes, PREFIX_GZ) {
        let bytes = &bytes[offset + PREFIX_GZ.len()..];

        let mut decoder = GzDecoder::new(bytes);

        Ok(bincode::deserialize_from(&mut decoder)?)
    } else {
        Err(anyhow::anyhow!("bank/patch data does not have v2 header"))
    }
}

fn serialize_bytes_plain<W: Write, T: Serialize>(writer: &mut W, value: &T) -> anyhow::Result<()> {
    Ok(bincode::serialize_into(writer, value)?)
}

fn serialize_bytes_gz<W: Write, T: Serialize>(writer: &mut W, value: &T) -> anyhow::Result<()> {
    let encoder = GzEncoder::new(writer, Compression::best());

    bincode::serialize_into(encoder, value)?;

    Ok(())
}

/// WIP: Version 0.8.5 introduces new operator wave forms
///
/// Prior versions only had sine and white noise variants
#[allow(dead_code)]
fn compat_0_8_5(patch: &mut SerdePatch) {
    let parameter_keys = [
        Parameter::Operator(0, OperatorParameter::WaveType).key(),
        Parameter::Operator(1, OperatorParameter::WaveType).key(),
        Parameter::Operator(2, OperatorParameter::WaveType).key(),
        Parameter::Operator(3, OperatorParameter::WaveType).key(),
    ];
    // Operator wave type parameter indices
    for key in parameter_keys {
        let p = patch.parameters.get_mut(&key).unwrap();

        // FIXME: set values valid for v0.8.5
        match p.value_string.as_str() {
            "SINE" => {
                p.value_f32 = 0.0;
            }
            "NOISE" => {
                p.value_f32 = 1.0;
            }
            v => {
                ::log::error!("found invalid operator wave type {}", v);
            }
        }
    }
}
