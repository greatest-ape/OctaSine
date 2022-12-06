use iced_baseview::{widget::checkbox::{Appearance, StyleSheet}, Color};

use super::Theme;


impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style, _is_checked: bool) -> Appearance {
        match self {
            Self::Light => {
                use super::light::colors::*;

                Appearance {
                    background: SURFACE.into(),
                    checkmark_color: BLUE,
                    text_color: Some(TEXT),
                    border_width: 1.0,
                    border_color: BORDER,
                    border_radius: 3.0,
                }
            }
            Self::Dark => {
                use super::dark::colors::*;

                Appearance {
                    background: Color::TRANSPARENT.into(),
                    checkmark_color: BLUE,
                    text_color: Some(TEXT),
                    border_width: 1.0,
                    border_color: BORDER,
                    border_radius: 3.0,
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> Appearance {
        match self {
            Self::Light => {
                use super::light::colors::*;

                Appearance {
                    background: SURFACE_HOVER.into(),
                    ..self.active(style, is_checked)
                }
            }
            Self::Dark => {
                use super::dark::colors::*;

                Appearance {
                    border_color: BORDER_HOVERED,
                    ..self.active(style, is_checked)
                }
            }
        }
    }
}
