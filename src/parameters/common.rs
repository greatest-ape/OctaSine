/// Convert plugin host float values in the range 0.0 - 1.0 to and from
/// the internal representation
pub trait ParameterValueConversion {
    type ProcessingValue;

    fn to_processing(value: f32) -> Self::ProcessingValue;
    fn to_sync(value: Self::ProcessingValue) -> f32;

    /// Parse a string value coming from the host
    fn parse_string_value(value: String) -> Option<Self::ProcessingValue>;

    fn format_processing(internal_value: Self::ProcessingValue) -> String;

    fn format_value(value: f32) -> String {
        Self::format_processing(Self::to_processing(value))
    }
}