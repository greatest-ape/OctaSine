use semver::Version;

use crate::parameters::{
    operator_wave_type::WaveType, OperatorParameter, OperatorWaveTypeValue, Parameter,
    ParameterValue,
};

use super::serde::{SerdePatch, SerdePatchParameterValue};

const CHANGES: &[(Version, fn(&mut SerdePatch))] = &[
    // (Version::new(0, 8, 5), compat_0_8_5),
];

pub fn run_patch_compatibility_changes(patch: &mut SerdePatch) -> anyhow::Result<()> {
    let version = patch.get_semver_version()?;

    for (v, f) in CHANGES {
        if version < *v {
            f(patch);
        } else {
            break;
        }
    }

    Ok(())
}

/// Version 0.8.5 introduces new operator wave forms
/// 
/// Prior versions only had sine and white noise variants
#[allow(dead_code)]
fn compat_0_8_5(patch: &mut SerdePatch) {
    for i in 0..4 {
        let parameter_index =
            Parameter::Operator(i, OperatorParameter::WaveType).to_index() as usize;

        let p = patch.parameters.get_mut(parameter_index).unwrap();

        // FIXME: needs to be fixed values for compatibility with later changes
        let patch_value = if p.value_text.contains("sin") {
            OperatorWaveTypeValue(WaveType::Sine).to_patch()
        } else {
            OperatorWaveTypeValue(WaveType::WhiteNoise).to_patch()
        };

        p.value_float = SerdePatchParameterValue::from_f32(patch_value);
        // FIXME: needs to set value_text
    }
}
