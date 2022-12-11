use iced_baseview::Color;

use crate::{hex, hex_gray};

pub const RED: Color = hex!(0xEF, 0x00, 0x00);
pub const BLUE: Color = hex!(0x00, 0x78, 0xEF);
pub const GREEN: Color = hex!(0x00, 0xEF, 0x78);

pub const GRAY_300: Color = hex_gray!(0x60);
pub const GRAY_400: Color = hex_gray!(0x77);
pub const GRAY_450: Color = hex_gray!(0x87);
pub const GRAY_500: Color = hex_gray!(0xA0);
pub const GRAY_600: Color = hex_gray!(0xB0);
pub const GRAY_700: Color = hex_gray!(0xD0);
pub const GRAY_800: Color = hex_gray!(0xE0);
pub const GRAY_900: Color = hex_gray!(0xEA);

pub const BACKGROUND: Color = GRAY_700;
pub const SURFACE: Color = Color::WHITE;
pub const SURFACE_HOVER: Color = GRAY_800;
pub const SURFACE_PRESS: Color = GRAY_700;
pub const TEXT: Color = Color::BLACK;
pub const BORDER: Color = GRAY_500;
