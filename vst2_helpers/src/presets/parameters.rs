use crate::processing_parameters::ParameterValueConversion;


pub trait PresetParameters: Default {
    fn get(&self, index: usize) -> Option<&dyn PresetParameter>;
    fn len(&self) -> usize;
}


pub trait PresetParameter {
    fn get_parameter_name(&self) -> String;
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }

    fn get_parameter_value_float(&self) -> f64;
    fn set_parameter_value_float(&self, value: f64);

    fn get_parameter_value_text(&self) -> String;
    fn set_parameter_value_text(&self, _value: String) -> bool;

    fn get_parameter_value_float_if_changed(&self) -> Option<f64>;
}


pub trait PresetParameterValueAccess {
    fn set_value(&self, value: f64);
    fn get_value(&self) -> f64;
    fn get_value_if_changed(&self) -> Option<f64>;
}


pub trait PresetParameterGetName {
    fn get_parameter_name(&self) -> String;
}


pub trait PresetParameterGetUnit {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }
}

/// Simple general implementation of PresetParameter for structs implementing
/// the other PresetParameter traits
impl<T> PresetParameter for T
    where T:
        PresetParameterValueAccess +
        PresetParameterGetName +
        PresetParameterGetUnit +
        ParameterValueConversion
{
    fn get_parameter_name(&self) -> String {
        PresetParameterGetName::get_parameter_name(self)
    }
    fn get_parameter_unit_of_measurement(&self) -> String {
        PresetParameterGetUnit::get_parameter_unit_of_measurement(self)
    }

    fn set_parameter_value_float(&self, value: f64){
        PresetParameterValueAccess::set_value(self, value);
    }
    fn set_parameter_value_text(&self, value: String) -> bool {
        if let Some(value) = Self::parse_string_value(value){
            let internal = Self::to_preset(value);
            PresetParameterValueAccess::set_value(self, internal);

            true
        } else {
            false
        }
    }
    fn get_parameter_value_float(&self) -> f64 {
        PresetParameterValueAccess::get_value(self)
    }
    fn get_parameter_value_text(&self) -> String {
        let value = PresetParameterValueAccess::get_value(self);

        Self::format_value(value)
    }
    fn get_parameter_value_float_if_changed(&self) -> Option<f64> {
        PresetParameterValueAccess::get_value_if_changed(self)
    }
}