use semver::Version;

use crate::parameters::{OperatorParameter, Parameter, SerializableRepresentation};

use super::SerdePatch;

pub const COMPATIBILITY_CHANGES: &[(Version, fn(&mut SerdePatch))] =
    &[(Version::new(0, 8, 5), compat_0_8_5)];

/// New operator wave forms
///
/// Prior versions only had sine and white noise variants
#[allow(dead_code)]
pub fn compat_0_8_5(patch: &mut SerdePatch) {
    let parameter_keys = [
        Parameter::Operator(0, OperatorParameter::WaveType).key(),
        Parameter::Operator(1, OperatorParameter::WaveType).key(),
        Parameter::Operator(2, OperatorParameter::WaveType).key(),
        Parameter::Operator(3, OperatorParameter::WaveType).key(),
    ];

    for key in parameter_keys {
        let p = patch.parameters.get_mut(&key).unwrap();

        match &p.value_serializable {
            SerializableRepresentation::Other(s) => {
                // These values will in most (but not all) cases already be set
                match s.as_str() {
                    "SINE" => {
                        p.value_patch = 0.0;
                    }
                    "NOISE" => {
                        p.value_patch = 1.0;
                    }
                    v => {
                        ::log::error!(
                            "converting patch for 0.8.5 compatibility: unrecognized operator wave type: {}",
                            v
                        );
                    }
                }
            }
            SerializableRepresentation::Float(v) => {
                ::log::error!(
                    "converting patch for 0.8.5 compatibility: incorrect serializable representation for operator wave type: {}",
                    v
                );
            }
        }
    }
}
