use byteorder::{BigEndian, WriteBytesExt};

use crate::{
    crate_version,
    plugin::common::{crate_version_to_vst2_format, PLUGIN_UNIQUE_VST2_ID},
};

pub fn make_fxp(
    patch_bytes: &[u8],
    patch_name: &str,
    num_parameters: usize,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(b"CcnK"); // fxp/fxp identifier
    bytes.write_i32::<BigEndian>((5 * 4 + 28 + 4 + patch_bytes.len()).try_into()?)?;

    bytes.extend_from_slice(b"FPCh"); // fxp opaque chunk
    bytes.write_i32::<BigEndian>(1)?; // fxp version
    bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_VST2_ID)?;
    bytes.write_i32::<BigEndian>(crate_version_to_vst2_format(crate_version!()))?;

    bytes.write_i32::<BigEndian>(num_parameters.try_into()?)?;

    let name_buf = {
        let mut buf = [0u8; 28];

        // Iterate through all buffer items except last, where a null
        // terminator must be left in place. If there are less than 27
        // chars, the last one will automatically be followed by a null
        // byte.
        for (b, c) in buf[..27].iter_mut().zip(
            patch_name
                .chars()
                .filter_map(|c| c.is_ascii().then_some(c as u8)),
        ) {
            *b = c;
        }

        buf
    };

    bytes.extend_from_slice(&name_buf);

    bytes.write_i32::<BigEndian>(patch_bytes.len().try_into()?)?;
    bytes.extend_from_slice(patch_bytes);

    Ok(bytes)
}

pub fn make_fxb(bank_bytes: &[u8], num_patches: usize) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(b"CcnK"); // fxp/fxp identifier
    bytes.write_i32::<BigEndian>((5 * 4 + 128 + 4 + bank_bytes.len()).try_into()?)?;

    bytes.extend_from_slice(b"FBCh"); // fxb opaque chunk
    bytes.write_i32::<BigEndian>(1)?; // fxb version (1 or 2)
    bytes.write_i32::<BigEndian>(PLUGIN_UNIQUE_VST2_ID)?;
    bytes.write_i32::<BigEndian>(crate_version_to_vst2_format(crate_version!()))?;

    bytes.write_i32::<BigEndian>(num_patches.try_into()?)?;
    bytes.extend(::std::iter::repeat(0).take(128)); // reserved padding for fxb version 1

    bytes.write_i32::<BigEndian>(bank_bytes.len().try_into()?)?;
    bytes.extend_from_slice(bank_bytes);

    Ok(bytes)
}
