//! Vst crate helpers, mainly preset and parameter handling.

#![allow(clippy::len_without_is_empty)]

pub mod approximations;
pub mod processing_parameters;
pub mod presets;
pub mod utils;


#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION").to_string()
    };
}


pub fn crate_version_to_vst_format(crate_version: String) -> i32 {
    format!("{:0<4}", crate_version.replace(".", ""))
        .parse()
        .expect("convert crate version to i32")
}


/// Implement VST PluginParameters on a struct by giving the field name to
/// its PresetBank
/// 
/// Currently seems to work as long as vst crate is included in the crate
/// using this macro.
#[macro_export]
macro_rules! impl_plugin_parameters {
    ($struct:ident, $preset_bank_field:ident) => {
        impl vst::plugin::PluginParameters for $struct {
            /// Get parameter label for parameter at `index` (e.g. "db", "sec", "ms", "%").
            fn get_parameter_label(&self, index: i32) -> String {
                self.$preset_bank_field.get_parameter_unit(index as usize)
            }

            /// Get the parameter value for parameter at `index` (e.g. "1.0", "150", "Plate", "Off").
            fn get_parameter_text(&self, index: i32) -> String {
                self.$preset_bank_field.get_parameter_value_text(index as usize)
            }

            /// Get the name of parameter at `index`.
            fn get_parameter_name(&self, index: i32) -> String {
                self.$preset_bank_field.get_parameter_name(index as usize)
            }

            /// Get the value of paramater at `index`. Should be value between 0.0 and 1.0.
            fn get_parameter(&self, index: i32) -> f32 {
                self.$preset_bank_field.get_parameter_value_float(index as usize) as f32
            }

            /// Set the value of parameter at `index`. `value` is between 0.0 and 1.0.
            fn set_parameter(&self, index: i32, value: f32) {
                self.$preset_bank_field.set_parameter_value_float_from_host(index as usize, f64::from(value));
            }

            /// Use String as input for parameter value. Used by host to provide an editable field to
            /// adjust a parameter value. E.g. "100" may be interpreted as 100hz for parameter. Returns if
            /// the input string was used.
            fn string_to_parameter(&self, index: i32, text: String) -> bool {
                self.$preset_bank_field.set_parameter_value_text_from_host(index as usize, text)
            }

            /// Return whether parameter at `index` can be automated.
            fn can_be_automated(&self, index: i32) -> bool {
                self.$preset_bank_field.can_parameter_be_automated(index as usize)
            }

            /// Set the current preset to the index specified by `preset`.
            ///
            /// This method can be called on the processing thread for automation.
            fn change_preset(&self, index: i32) {
                self.$preset_bank_field.set_preset_index(index as usize);
            }

            /// Get the current preset index.
            fn get_preset_num(&self) -> i32 {
                self.$preset_bank_field.get_preset_index() as i32
            }

            /// Set the current preset name.
            fn set_preset_name(&self, name: String) {
                self.$preset_bank_field.set_current_preset_name(name)
            }

            /// Get the name of the preset at the index specified by `preset`.
            fn get_preset_name(&self, index: i32) -> String {
                self.$preset_bank_field.get_preset_name_by_index(index as usize)
            }

            /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
            /// the current preset.
            fn get_preset_data(&self) -> Vec<u8> {
                self.$preset_bank_field.export_current_preset_bytes()
            }

            /// If `preset_chunks` is set to true in plugin info, this should return the raw chunk data for
            /// the current plugin bank.
            fn get_bank_data(&self) -> Vec<u8> {
                self.$preset_bank_field.export_bank_as_bytes()
            }

            /// If `preset_chunks` is set to true in plugin info, this should load a preset from the given
            /// chunk data.
            fn load_preset_data(&self, data: &[u8]) {
                self.$preset_bank_field.import_bytes_into_current_preset(data);
            }

            /// If `preset_chunks` is set to true in plugin info, this should load a preset bank from the
            /// given chunk data.
            fn load_bank_data(&self, data: &[u8]) {
                if let Err(err) = self.$preset_bank_field.import_bank_from_bytes(data){
                    // FIXME: how do I enable this conditionally based on a feature in
                    // a different crate, in a sensible way..
                    // ::log::error!("couldn't load preset bank from bytes: {}", err);
                }
            }
        }
    };
}


// Parameter macros


/// Implement simple preset parameter value access for when "value" field
/// contains an atomic positive double
#[macro_export]
macro_rules! impl_preset_parameter_value_access {
    ($name:ident) => {
        impl PresetParameterValueAccess for $name {
            fn set_value(&self, value: f64) {
                self.value.set(value);
            }
            fn get_value(&self) -> f64 {
                self.value.get()
            }
            fn get_value_if_changed(&self) -> Option<f64> {
                self.value.get_if_changed()
            }
        }
    };
}


/// Implement parameter value conversion from processing parameter already
/// implementing it
#[macro_export]
macro_rules! impl_value_conversion_from_processing {
    ($name:ident, $other:ident) => {
        impl ParameterValueConversion for $name {
            type ProcessingParameterValue = <$other as ProcessingParameter>::Value;

            fn to_processing(value: f64) -> Self::ProcessingParameterValue {
                $other::to_processing(value)
            }
            fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
                $other::to_preset(value)
            }

            /// Parse a string value coming from the host
            fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
                $other::parse_string_value(value)
            }

            fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
                $other::format_processing(internal_value)
            }
        }
    };
}


/// Helper for trivial string parsing for value conversion
#[macro_export]
macro_rules! simple_parameter_string_parsing {
    ($struct_name:ident, $value:ident, $internal_type:ty) => {
        $value.parse::<$internal_type>().ok().map(|value| {
            let max = $struct_name::to_processing(1.0);
            let min = $struct_name::to_processing(0.0);

            value.max(min).min(max)
        })
    };
}


/// Implement ParameterValueConversion with 1-to-1 conversion
#[macro_export]
macro_rules! impl_parameter_value_conversion_identity {
    ($struct_name:ident) => {
        impl ParameterValueConversion for $struct_name {
            type ProcessingParameterValue = f64;

            fn to_processing(value: f64) -> Self::ProcessingParameterValue {
                value
            }
            fn to_preset(value: Self::ProcessingParameterValue) -> f64 {
                value
            }
            fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue> {
                simple_parameter_string_parsing!(Self, value, Self::ProcessingParameterValue)
            }

            fn format_processing(internal_value: Self::ProcessingParameterValue) -> String {
                format!("{:.02}", internal_value)
            }
        }
    };
}


#[macro_export]
macro_rules! create_interpolatable_processing_parameter {
    ($name:ident, $default:ident, $extra_data:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            value: InterpolatableProcessingValue,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: InterpolatableProcessingValue::new($default)
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = f64;
            type ExtraData = $extra_data;

            fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value {
                self.value.get_value(extra_data, &mut |_| ())
            }
            fn get_target_value(&self) -> Self::Value {
                self.value.target_value
            }
            fn set_value(&mut self, value: Self::Value) {
                self.value.set_value(value)
            }
        }
    }
}


#[macro_export]
macro_rules! create_simple_processing_parameter {
    ($name:ident, $type:ty, $default:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub value: $type
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    value: $default
                }
            }
        }

        impl ProcessingParameter for $name {
            type Value = $type;
            type ExtraData = ();

            fn get_value(&mut self, _: Self::ExtraData) -> Self::Value {
                self.value
            }
            fn get_target_value(&self) -> Self::Value {
                self.value
            }
            fn set_value(&mut self, value: Self::Value){
                self.value = value;
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format(){
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}