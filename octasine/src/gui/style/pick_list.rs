use iced_baseview::widget::pick_list::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Appearance {
                    background: SURFACE.into(),
                    text_color: TEXT,
                    border_color: BORDER,
                    border_width: 1.0,
                    border_radius: 3.0,
                    placeholder_color: TEXT,
                    handle_color: TEXT,
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    background: SURFACE.into(),
                    text_color: TEXT,
                    border_color: TEXT,
                    border_width: 0.0,
                    border_radius: 3.0,
                    placeholder_color: TEXT,
                    handle_color: TEXT,
                }
            }
        }
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Appearance {
                    background: SURFACE_HOVER.into(),
                    ..self.active(style)
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    background: SURFACE_HOVER.into(),
                    text_color: HOVERED,
                    ..self.active(style)
                }
            }
        }
    }
}
