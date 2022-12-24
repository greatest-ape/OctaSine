use std::{
    ffi::{CStr, CString},
    ptr::null,
};

use clap_sys::{
    plugin::clap_plugin_descriptor,
    plugin_features::{
        CLAP_PLUGIN_FEATURE_INSTRUMENT, CLAP_PLUGIN_FEATURE_STEREO, CLAP_PLUGIN_FEATURE_SYNTHESIZER,
    },
    version::CLAP_VERSION,
};
use once_cell::sync::Lazy;

pub const ID: *const i8 = unsafe { CStr::from_bytes_with_nul_unchecked(b"OctaSine\0").as_ptr() };
const NAME: *const i8 = unsafe { CStr::from_bytes_with_nul_unchecked(b"OctaSine\0").as_ptr() };
const VENDOR: *const i8 =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"Joakim Frostegard\0").as_ptr() };
const URL: *const i8 =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"https://octasine.com\0").as_ptr() };

const FEATURES: &[*const i8] = &[
    CLAP_PLUGIN_FEATURE_INSTRUMENT.as_ptr(),
    CLAP_PLUGIN_FEATURE_SYNTHESIZER.as_ptr(),
    CLAP_PLUGIN_FEATURE_STEREO.as_ptr(),
    null(),
];

static VERSION: Lazy<CString> = Lazy::new(|| CString::new(crate::crate_version!()).unwrap());

pub static DESCRIPTOR: Lazy<clap_plugin_descriptor> = Lazy::new(|| clap_plugin_descriptor {
    clap_version: CLAP_VERSION,
    id: ID,
    name: NAME,
    vendor: VENDOR,
    url: URL,
    manual_url: null(),
    support_url: null(),
    version: Lazy::force(&VERSION).as_ptr(),
    description: null(),
    features: FEATURES.as_ptr(),
});
