mod compat;

use std::io::{BufReader, Write};

use compact_str::CompactString;
use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    common::IndexMap,
    parameters::{Parameter, ParameterKey, SerializableRepresentation},
    sync::patch_bank::{Patch, PatchBank},
};

use self::compat::COMPATIBILITY_CHANGES;

use super::common::{make_fxb, make_fxp};

const PREFIX_PLAIN: &[u8] = b"\n\nOCTASINE-DATA-V2-PLAIN\n\n";
const PREFIX_GZ: &[u8] = b"\n\nOCTASINE-DATA-V2-GZ\n\n";

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: Version,
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

    pub fn from_v1(v1: super::v1::SerdePatchBank) -> anyhow::Result<Self> {
        let octasine_version = super::v1::parse_version(&v1.octasine_version)?;
        let mut v2_patches = Vec::with_capacity(v1.patches.len());

        for v1_patch in v1.patches.into_iter() {
            v2_patches.push(SerdePatch::from_v1(v1_patch)?);
        }

        Ok(Self {
            octasine_version,
            patches: v2_patches,
        })
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

    pub fn serialize_fxb_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        serialize_bytes_gz(&mut buffer, self)?;

        make_fxb(&buffer, self.patches.len())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatch {
    octasine_version: Version,
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
                    index: i,
                    value_patch: p.get_value(),
                    value_serializable: p.get_serializable(),
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

    pub fn from_v1(v1: super::v1::SerdePatch) -> anyhow::Result<Self> {
        let octasine_version = super::v1::parse_version(&v1.octasine_version)?;

        let mut v2_parameters = Self::new(&Patch::default()).parameters;

        for (index, v1_parameter) in v1.parameters.into_iter().enumerate() {
            let parameter = Parameter::from_index(index).ok_or_else(|| anyhow::anyhow!(""))?;

            let v2_parameter = v2_parameters
                .get_mut(&parameter.key())
                .ok_or_else(|| anyhow::anyhow!("no v2 parameter {:?}", parameter))?;

            *v2_parameter = SerdePatchParameter {
                index,
                value_patch: v1_parameter.value_float.as_f32(),
                value_serializable: SerializableRepresentation::Other(
                    v1_parameter.value_text.into(),
                ),
            };
        }

        let mut patch = Self {
            octasine_version,
            name: v1.name.into(),
            parameters: v2_parameters,
        };

        patch.run_compatibility_changes();

        Ok(patch)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut patch: Self = deserialize_bytes(bytes)?;

        patch.run_compatibility_changes();

        Ok(patch)
    }

    pub fn serialize_fxp_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        serialize_bytes_gz(&mut buffer, self)?;

        make_fxp(&buffer, &self.name, self.parameters.len())
    }

    fn run_compatibility_changes(&mut self) {
        for (changed_in_version, f) in COMPATIBILITY_CHANGES {
            if self.octasine_version < *changed_in_version {
                f(self);
            } else {
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchParameter {
    index: usize,
    pub value_patch: f32,
    value_serializable: SerializableRepresentation,
}

pub fn bytes_are_v2(bytes: &[u8]) -> bool {
    memchr::memmem::find(bytes, PREFIX_PLAIN).is_some()
        || memchr::memmem::find(bytes, PREFIX_GZ).is_some()
}

fn get_octasine_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

fn deserialize_bytes<T>(bytes: &[u8]) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
{
    if let Some(offset) = memchr::memmem::find(bytes, PREFIX_PLAIN) {
        let bytes = &bytes[offset + PREFIX_PLAIN.len()..];

        Ok(cbor4ii::serde::from_slice(bytes)?)
    } else if let Some(offset) = memchr::memmem::find(bytes, PREFIX_GZ) {
        let bytes = &bytes[offset + PREFIX_GZ.len()..];

        let mut decoder = BufReader::new(GzDecoder::new(bytes));

        Ok(cbor4ii::serde::from_reader(&mut decoder)?)
    } else {
        Err(anyhow::anyhow!("bank/patch data does not have v2 header"))
    }
}

fn serialize_bytes_plain<W: Write, T: Serialize>(writer: &mut W, value: &T) -> anyhow::Result<()> {
    writer.write_all(PREFIX_PLAIN)?;

    Ok(cbor4ii::serde::to_writer(writer, value)?)
}

fn serialize_bytes_gz<W: Write, T: Serialize>(writer: &mut W, value: &T) -> anyhow::Result<()> {
    writer.write_all(PREFIX_GZ)?;

    let mut encoder = GzEncoder::new(writer, Compression::best());

    cbor4ii::serde::to_writer(&mut encoder, value)?;

    Ok(())
}
