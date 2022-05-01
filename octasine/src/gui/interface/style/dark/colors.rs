use iced_baseview::Color;

use crate::{hex, hex_gray};

pub const GRAY_100: Color = hex_gray!(0x20);
pub const GRAY_200: Color = hex_gray!(0x2A);
pub const GRAY_300: Color = hex_gray!(0x3A);
pub const GRAY_400: Color = hex_gray!(0x50);
pub const GRAY_500: Color = hex_gray!(0x60);
pub const GRAY_600: Color = hex_gray!(0x70);
pub const GRAY_700: Color = hex_gray!(0x90);
pub const GRAY_800: Color = hex_gray!(0xB0);
pub const GRAY_900: Color = hex_gray!(0xD0);

pub const RED: Color = hex!(0xEF, 0x53, 0x50);
pub const BLUE: Color = hex!(0x50, 0x9D, 0xEF);
pub const GREEN: Color = hex!(0x50, 0xEF, 0x2a);

pub const BACKGROUND: Color = hex_gray!(0x00);
pub const SURFACE: Color = GRAY_400;
pub const SURFACE_HOVER: Color = GRAY_500;
pub const TEXT_BG: Color = GRAY_900;
pub const TEXT_FG: Color = GRAY_900;
pub const HOVERED: Color = hex_gray!(0xFF);
