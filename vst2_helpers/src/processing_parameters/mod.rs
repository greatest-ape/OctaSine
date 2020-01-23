//! Processing parameters

pub mod interpolatable_value;
pub mod utils;


/// Convert plugin host float values in the range 0.0 - 1.0 to and from
/// the internal representation
pub trait ParameterValueConversion {
    type ProcessingParameterValue;

    fn to_processing(value: f64) -> Self::ProcessingParameterValue;
    fn to_preset(value: Self::ProcessingParameterValue) -> f64;

    /// Parse a string value coming from the host
    fn parse_string_value(value: String) -> Option<Self::ProcessingParameterValue>;

    fn format_processing(internal_value: Self::ProcessingParameterValue) -> String;

    fn format_value(value: f64) -> String {
        Self::format_processing(Self::to_processing(value))
    }
}


pub trait ProcessingParameter {
    type Value;
    type ExtraData;

    fn get_value(&mut self, extra_data: Self::ExtraData) -> Self::Value;
    fn get_target_value(&self) -> Self::Value;
    fn set_value(&mut self, value: Self::Value);
}

pub trait ProcessingParameterPresetValueAccess {
    fn set_from_preset_value(&mut self, value: f64);
    fn get_preset_target_value(&self) -> f64;
}

impl<P, T, D> ProcessingParameterPresetValueAccess for P
    where P:
        ProcessingParameter<Value = T, ExtraData = D> +
        ParameterValueConversion<ProcessingParameterValue = T>
{
    fn set_from_preset_value(&mut self, value: f64){
        self.set_value(Self::to_processing(value));
    }
    fn get_preset_target_value(&self) -> f64 {
        Self::to_preset(self.get_target_value())
    }
}