use semver::Version;

use super::serde::{SerdePatch, SerdePatchParameterValue};

const CHANGES: &[(Version, fn(&mut SerdePatch))] = &[
    // (Version::new(0, 8, 5), compat_0_8_5),
];

pub fn run_patch_compatibility_changes(patch: &mut SerdePatch) -> anyhow::Result<()> {
    let patch_version = patch.get_semver_version()?;

    for (version, f) in CHANGES {
        if patch_version < *version {
            f(patch);
        } else {
            break;
        }
    }

    Ok(())
}

/// WIP: Version 0.8.5 introduces new operator wave forms
///
/// Prior versions only had sine and white noise variants
#[allow(dead_code)]
fn compat_0_8_5(patch: &mut SerdePatch) {
    // Operator wave type parameter indices
    for parameter_index in [6, 20, 36, 52] {
        let p = patch.parameters.get_mut(parameter_index).unwrap();

        let value_text = p.value_text.to_lowercase();

        if value_text.contains("sin") {
            // Set values valid for v0.8.5 sine waveform
            p.value_float = SerdePatchParameterValue::from_f32(0.0);
            p.value_text = "SINE".to_string();
        } else {
            // Set values valid for v0.8.5 noise waveform
            p.value_float = SerdePatchParameterValue::from_f32(1.0);
            p.value_text = "NOISE".to_string();
        };
    }
}
