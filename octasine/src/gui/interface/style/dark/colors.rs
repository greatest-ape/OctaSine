use iced_baseview::Color;


#[macro_export]
macro_rules! hex_gray {
    ($hex:literal) => {
        ::iced_baseview::Color::from_rgb(
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
        )
    };
}

#[macro_export]
macro_rules! hex {
    ($r:literal, $g:literal, $b:literal) => {
        ::iced_baseview::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

pub const GRAY_100: Color = hex_gray!(0x20);
pub const GRAY_200: Color = hex_gray!(0x2A);
pub const GRAY_600: Color = hex_gray!(0x70);
pub const GRAY_700: Color = hex_gray!(0x90);
pub const GRAY_800: Color = hex_gray!(0xB0);
pub const GRAY_900: Color = hex_gray!(0xD0);


pub const BACKGROUND: Color = hex_gray!(0x00);
pub const SURFACE: Color = hex_gray!(0x20);
pub const TEXT_BG: Color = GRAY_900;
pub const TEXT_FG: Color = GRAY_900;
pub const HOVERED: Color = hex_gray!(0xDD);
pub const CONTRAST: Color = hex_gray!(0x30);

pub const RED: Color = hex!(0xEF, 0x53, 0x50);
pub const BLUE: Color = hex!(0x50, 0x9D, 0xEF);
pub const GREEN: Color = hex!(0x50, 0xEF, 0x2a);