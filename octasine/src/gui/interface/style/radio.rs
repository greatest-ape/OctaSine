use iced_baseview::widget::radio::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Appearance {
                    background: SURFACE.into(),
                    dot_color: TEXT,
                    text_color: Some(TEXT),
                    border_width: 1.0,
                    border_color: BORDER,
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    background: SURFACE.into(),
                    dot_color: TEXT,
                    text_color: Some(TEXT),
                    border_width: 1.0,
                    border_color: TEXT,
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Appearance {
                    background: SURFACE_HOVER.into(),
                    ..self.active(style, is_selected)
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    border_color: HOVERED,
                    ..self.active(style, is_selected)
                }
            }
        }
    }
}
