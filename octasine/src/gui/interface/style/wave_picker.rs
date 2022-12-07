use iced_baseview::Color;

use crate::gui::interface::wave_picker::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    fn appearance(&self) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;
                Appearance {
                    background_color: SURFACE,
                    border_color_active: BORDER,
                    border_color_hovered: BORDER,
                    middle_line_color: GRAY_600,
                    shape_line_color_active: BLUE,
                    shape_line_color_hovered: BLUE,
                }
            }
            Self::Dark => {
                use super::colors::dark::*;
                Appearance {
                    background_color: Color::TRANSPARENT.into(),
                    border_color_active: BORDER,
                    border_color_hovered: BORDER_HOVERED,
                    middle_line_color: GRAY_400,
                    shape_line_color_active: BLUE,
                    shape_line_color_hovered: BLUE,
                }
            }
        }
    }
}
