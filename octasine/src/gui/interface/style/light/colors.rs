use iced_baseview::Color;

use crate::{hex, hex_gray};

pub const RED: Color = hex!(0xEF, 0x53, 0x50);
pub const BLUE: Color = hex!(0x50, 0x9D, 0xEF);
pub const GREEN: Color = hex!(0x50, 0xEF, 0x2a);

pub const GRAY_300: Color = hex_gray!(0x60);
pub const GRAY_500: Color = hex_gray!(0xA0);
pub const GRAY_600: Color = hex_gray!(0xB0);
pub const GRAY_700: Color = hex_gray!(0xD0);
pub const GRAY_800: Color = hex_gray!(0xE0);
pub const GRAY_900: Color = hex_gray!(0xEA);

pub const BACKGROUND: Color = GRAY_700;
pub const SURFACE: Color = Color::WHITE;
pub const SURFACE_HOVER: Color = GRAY_800;
pub const TEXT_BG: Color = Color::BLACK;
pub const TEXT_FG: Color = Color::BLACK;
pub const BORDER: Color = GRAY_500;
