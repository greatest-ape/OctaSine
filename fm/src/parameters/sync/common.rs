use super::super::common::ParameterValueConversion;


pub trait SyncParameter {
    fn get_parameter_name(&self) -> String;
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }

    fn get_parameter_value_float(&self) -> f32;
    fn set_parameter_value_float(&self, value: f32);

    fn get_parameter_value_text(&self) -> String;
    fn set_parameter_value_text(&self, _value: String) -> bool;


    fn get_parameter_value_float_if_changed(&self) -> Option<f32>;
}


pub trait SyncParameterValueAccess {
    fn set_value(&self, value: f32);
    fn get_value(&self) -> f32;
    fn get_value_if_changed(&self) -> Option<f32>;
}


pub trait SyncParameterGetName {
    fn get_parameter_name(&self) -> String;
}


pub trait SyncParameterGetUnit {
    fn get_parameter_unit_of_measurement(&self) -> String {
        "".to_string()
    }
}

/// Simple general implementation of SyncParameter for structs implementing
/// the other SyncParameter traits
impl<T> SyncParameter for T
    where T:
        SyncParameterValueAccess +
        SyncParameterGetName +
        SyncParameterGetUnit +
        ParameterValueConversion
{
    fn get_parameter_name(&self) -> String {
        SyncParameterGetName::get_parameter_name(self)
    }
    fn get_parameter_unit_of_measurement(&self) -> String {
        SyncParameterGetUnit::get_parameter_unit_of_measurement(self)
    }

    fn set_parameter_value_float(&self, value: f32){
        SyncParameterValueAccess::set_value(self, value);
    }
    fn set_parameter_value_text(&self, value: String) -> bool {
        if let Some(value) = Self::parse_string_value(value){
            let internal = Self::to_sync(value);
            SyncParameterValueAccess::set_value(self, internal);

            true
        } else {
            false
        }
    }
    fn get_parameter_value_float(&self) -> f32 {
        SyncParameterValueAccess::get_value(self)
    }
    fn get_parameter_value_text(&self) -> String {
        let value = SyncParameterValueAccess::get_value(self);

        Self::format_value(value)
    }
    fn get_parameter_value_float_if_changed(&self) -> Option<f32> {
        SyncParameterValueAccess::get_value_if_changed(self)
    }
}