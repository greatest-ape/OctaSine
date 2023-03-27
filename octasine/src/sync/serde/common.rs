use byteorder::{BigEndian, WriteBytesExt};

use crate::{
    crate_version,
    plugin::common::{crate_version_to_vst2_format, PLUGIN_UNIQUE_VST2_ID},
};

pub fn split_off_slice_prefix<'a>(mut bytes: &'a [u8], prefix: &[u8]) -> &'a [u8] {
    if let Some(index) = find_in_slice(bytes, prefix) {
        bytes = &bytes[index + prefix.len()..];
    }

    bytes
}

pub fn find_in_slice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
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
    bytes.extend_from_slice(&bank_bytes);

    Ok(bytes)
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
