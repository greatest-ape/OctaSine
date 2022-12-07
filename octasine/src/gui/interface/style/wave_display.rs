use iced_baseview::Color;

use crate::gui::interface::wave_display::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    fn appearance(&self) -> Appearance {
        match self {
            Self::Light => {
                use super::light::colors::*;
                Appearance {
                    background_color: SURFACE,
                    border_color: BORDER,
                    middle_line_color: GRAY_600,
                    wave_line_color: BLUE,
                }
            }
            Self::Dark => {
                use super::dark::colors::*;
                Appearance {
                    background_color: Color::TRANSPARENT.into(),
                    border_color: BORDER_DARK,
                    middle_line_color: GRAY_400,
                    wave_line_color: BLUE,
                }
            }
        }
    }
}
