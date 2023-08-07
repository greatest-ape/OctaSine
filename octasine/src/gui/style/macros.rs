#[macro_export]
macro_rules! hex_gray {
    ($hex:literal) => {
        ::iced_baseview::core::Color::from_rgb(
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
        )
    };
}

#[macro_export]
macro_rules! hex {
    ($r:literal, $g:literal, $b:literal) => {
        ::iced_baseview::core::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}
