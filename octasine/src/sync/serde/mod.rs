mod common;
mod v1;
mod v2;

use std::io::Write;

use super::patch_bank::{Patch, PatchBank};

/// Remember to update relevant metadata if changes were indeed made
pub fn update_bank_from_bytes(bank: &PatchBank, bytes: &[u8]) -> anyhow::Result<()> {
    if v2::bytes_are_v2(bytes) {
        let serde_bank = v2::SerdePatchBank::from_bytes(bytes)?;
        let default_serde_patch = v2::SerdePatch::new(&Patch::default());

        for (index, patch) in bank.patches.iter().enumerate() {
            let serde_patch = if let Some(serde_patch) = serde_bank.patches.get(index) {
                patch.set_name(serde_patch.name.as_str().into());

                serde_patch
            } else {
                patch.set_name("".into());

                &default_serde_patch
            };

            for (a, b) in patch
                .envelope_viewports
                .iter()
                .zip(serde_patch.envelope_viewports.iter())
            {
                a.viewport_factor.set(b.viewport_factor);
                a.x_offset.set(b.x_offset);
            }

            for (key, parameter) in patch.parameters.iter() {
                if let Some(serde_parameter) = serde_patch.parameters.get(key) {
                    parameter.set_value(serde_parameter.value_f32);
                }
            }
        }
    } else {
        let serde_bank = v1::SerdePatchBank::from_bytes(bytes)?;
        let default_serde_patch = v1::SerdePatch::new(&Patch::default());

        for (index, patch) in bank.patches.iter().enumerate() {
            let serde_patch = if let Some(serde_patch) = serde_bank.patches.get(index) {
                patch.set_name(serde_patch.name.as_str().into());

                serde_patch
            } else {
                patch.set_name("".into());

                &default_serde_patch
            };

            for (index, parameter) in patch.parameters.values().enumerate() {
                if let Some(import_parameter) = serde_patch.parameters.get(index) {
                    parameter.set_value(import_parameter.value_float.as_f32())
                }
            }
        }
    }

    Ok(())
}

/// Remember to update relevant metadata if changes were indeed made
pub fn update_patch_from_bytes(patch: &Patch, bytes: &[u8]) -> anyhow::Result<()> {
    if v2::bytes_are_v2(bytes) {
        let serde_patch = v2::SerdePatch::from_bytes(bytes)?;

        patch.set_name(serde_patch.name.as_str());

        for (a, b) in patch
            .envelope_viewports
            .iter()
            .zip(serde_patch.envelope_viewports.iter())
        {
            a.viewport_factor.set(b.viewport_factor);
            a.x_offset.set(b.x_offset);
        }

        for (key, parameter) in patch.parameters.iter() {
            if let Some(serde_parameter) = serde_patch.parameters.get(key) {
                parameter.set_value(serde_parameter.value_f32);
            }
        }
    } else {
        let serde_patch = v1::SerdePatch::from_bytes(bytes)?;

        patch.set_name(serde_patch.name.as_str());

        for (index, parameter) in patch.parameters.values().enumerate() {
            if let Some(import_parameter) = serde_patch.parameters.get(index) {
                parameter.set_value(import_parameter.value_float.as_f32())
            }
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
    v2::SerdePatchBank::new(bank).to_fxb_bytes()
}

pub fn serialize_patch_fxp_bytes(patch: &Patch) -> anyhow::Result<Vec<u8>> {
    v2::SerdePatch::new(patch).to_fxp_bytes()
}
