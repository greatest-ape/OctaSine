use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::sync::patch_bank::{Patch, PatchBank};

use super::common::{find_in_slice, make_fxb, make_fxp, split_off_slice_prefix};

const BANK_PREFIX: &[u8] = b"\n\nOCTASINE-GZ-DATA-V2-BANK-BEGIN\n\n";
const PATCH_PREFIX: &[u8] = b"\n\nOCTASINE-GZ-DATA-V2-PATCH-BEGIN\n\n";

const COMPATIBILITY_CHANGES: &[(Version, fn(&mut SerdePatch))] = &[
    // (Version::new(0, 8, 5), compat_0_8_5),
];

#[derive(Serialize, Deserialize)]
pub struct SerdePatchBank {
    octasine_version: [u64; 3],
    patches: Vec<SerdePatch>,
}

impl SerdePatchBank {
    fn new(bank: &PatchBank) -> Self {
        let patches = bank.patches.iter().map(SerdePatch::new).collect();

        Self {
            octasine_version: get_octasine_version(),
            patches,
        }
    }
}

impl SerdePatchBank {
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let bytes = split_off_slice_prefix(bytes, BANK_PREFIX);

        let mut decoder = GzDecoder::new(bytes);

        let mut bank: Self = bincode::deserialize_from(&mut decoder)?;

        for patch in bank.patches.iter_mut() {
            patch.run_compatibility_changes();
        }

        Ok(bank)
    }

    fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut bytes = Vec::from(BANK_PREFIX);

        let mut encoder = GzEncoder::new(&mut bytes, Compression::best());

        bincode::serialize_into(&mut encoder, self)?;

        encoder.finish()?;

        Ok(bytes)
    }

    fn to_fxb_bytes(&self) -> anyhow::Result<Vec<u8>> {
        make_fxb(&self.to_bytes()?, self.patches.len())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatch {
    octasine_version: [u64; 3],
    name: String,
    parameters: Vec<SerdePatchParameter>,
}

impl SerdePatch {
    fn new(patch: &Patch) -> Self {
        let parameters = patch
            .parameters
            .iter()
            .enumerate()
            .map(|(i, (k, p))| SerdePatchParameter {
                name: p.name,
                index: i,
                key: k.0,
                value_f32: p.get_value(),
                value_string: p.get_value_text(),
            })
            .collect();

        Self {
            octasine_version: get_octasine_version(),
            name: patch.get_name(),
            parameters,
        }
    }

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let bytes = split_off_slice_prefix(bytes, PATCH_PREFIX);

        let mut decoder = GzDecoder::new(bytes);

        let mut patch: Self = bincode::deserialize_from(&mut decoder)?;

        patch.run_compatibility_changes();

        Ok(patch)
    }

    fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut bytes = Vec::from(PATCH_PREFIX);

        let mut encoder = GzEncoder::new(&mut bytes, Compression::best());

        bincode::serialize_into(&mut encoder, self)?;

        encoder.finish()?;

        Ok(bytes)
    }

    fn to_fxp_bytes(&self) -> anyhow::Result<Vec<u8>> {
        make_fxp(&self.to_bytes()?, &self.name, self.parameters.len())
    }

    fn run_compatibility_changes(&mut self) {
        let patch_version = {
            let [major, minor, patch] = self.octasine_version;

            Version::new(major, minor, patch)
        };

        for (change_version, f) in COMPATIBILITY_CHANGES {
            if patch_version < *change_version {
                f(self);
            } else {
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdePatchParameter {
    name: String,
    index: usize,
    key: u32,
    value_f32: f32,
    value_string: String,
}

fn get_octasine_version() -> [u64; 3] {
    let version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();

    [version.major, version.minor, version.patch]
}

pub fn bytes_are_v2(bytes: &[u8]) -> bool {
    find_in_slice(bytes, PATCH_PREFIX).is_some() || find_in_slice(bytes, BANK_PREFIX).is_some()
}

/// WIP: Version 0.8.5 introduces new operator wave forms
///
/// Prior versions only had sine and white noise variants
#[allow(dead_code)]
fn compat_0_8_5(patch: &mut SerdePatch) {
    // Operator wave type parameter indices
    for parameter_index in [6, 20, 36, 52] {
        let p = patch.parameters.get_mut(parameter_index).unwrap();

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
