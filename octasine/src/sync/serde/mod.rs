mod common;
mod v1;
mod v2;

use std::io::Write;

use super::patch_bank::{Patch, PatchBank};

/// Remember to update relevant metadata if changes were indeed made
pub fn update_bank_from_bytes(bank: &PatchBank, bytes: &[u8]) -> anyhow::Result<()> {
    let serde_bank = if v2::bytes_are_v2(bytes) {
        v2::SerdePatchBank::from_bytes(bytes)?
    } else {
        v2::SerdePatchBank::from_v1(v1::SerdePatchBank::from_bytes(bytes)?)?
    };

    let default_serde_patch = v2::SerdePatch::new(&Patch::default());

    for (index, patch) in bank.patches.iter().enumerate() {
        let serde_patch = if let Some(serde_patch) = serde_bank.patches.get(index) {
            patch.set_name(serde_patch.name.as_str().into());

            serde_patch
        } else {
            patch.set_name("".into());

            &default_serde_patch
        };

        for (key, parameter) in patch.parameters.iter() {
            if let Some(serde_parameter) = serde_patch.parameters.get(key) {
                parameter.set_value(serde_parameter.value_patch);
            }
        }
    }

    Ok(())
}

/// Remember to update relevant metadata if changes were indeed made
pub fn update_patch_from_bytes(patch: &Patch, bytes: &[u8]) -> anyhow::Result<()> {
    let serde_patch = if v2::bytes_are_v2(bytes) {
        v2::SerdePatch::from_bytes(bytes)?
    } else {
        v2::SerdePatch::from_v1(v1::SerdePatch::from_bytes(bytes)?)?
    };

    patch.set_name(serde_patch.name.as_str());

    for (key, parameter) in patch.parameters.iter() {
        if let Some(serde_parameter) = serde_patch.parameters.get(key) {
            parameter.set_value(serde_parameter.value_patch);
        }
    }

    Ok(())
}

pub fn serialize_bank_plain_bytes<W: Write>(
    writer: &mut W,
    bank: &PatchBank,
) -> anyhow::Result<()> {
    v2::SerdePatchBank::new(bank).serialize_plain(writer)
}

pub fn serialize_bank_fxb_bytes(bank: &PatchBank) -> anyhow::Result<Vec<u8>> {
    v2::SerdePatchBank::new(bank).serialize_fxb_bytes()
}

pub fn serialize_patch_fxp_bytes(patch: &Patch) -> anyhow::Result<Vec<u8>> {
    v2::SerdePatch::new(patch).serialize_fxp_bytes()
}
