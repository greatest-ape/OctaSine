pub const PLUGIN_UNIQUE_VST2_ID: i32 = 1_438_048_625;
pub const PLUGIN_SEMVER_NAME: &str = "OctaSine v0.8";

pub fn crate_version_to_vst2_format(crate_version: &str) -> i32 {
    format!("{:0<4}", crate_version.replace('.', ""))
        .parse()
        .expect("convert crate version to i32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format() {
        assert_eq!(crate_version_to_vst2_format("1"), 1000);
        assert_eq!(crate_version_to_vst2_format("0.1"), 0100);
        assert_eq!(crate_version_to_vst2_format("0.0.2"), 0020);
        assert_eq!(crate_version_to_vst2_format("0.5.2"), 0520);
        assert_eq!(crate_version_to_vst2_format("1.0.1"), 1010);
    }
}
