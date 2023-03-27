mod common;
mod v1;
mod v2;

use super::patch_bank::{Patch, PatchBank};

pub fn update_bank_from_bytes(bank: &PatchBank, bytes: &[u8]) -> anyhow::Result<()> {
    todo!()
}

pub fn update_patch_from_bytes(patch: &Patch, bytes: &[u8]) -> anyhow::Result<()> {
    todo!()
}

pub fn get_bank_bytes(bank: &PatchBank) -> anyhow::Result<Vec<u8>> {
    todo!()
}

pub fn get_patch_bytes(patch: &Patch) -> anyhow::Result<Vec<u8>> {
    todo!()
}
