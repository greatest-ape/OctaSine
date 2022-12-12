use iced_baseview::overlay::menu::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Appearance {
                    background: SURFACE.into(),
                    text_color: TEXT,
                    selected_background: SURFACE_HOVER.into(),
                    selected_text_color: TEXT,
                    border_width: 1.0,
                    border_color: SURFACE,
                    border_radius: 3.0,
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    background: GRAY_300.into(),
                    selected_background: SURFACE_HOVER.into(),
                    text_color: TEXT,
                    selected_text_color: HOVERED,
                    border_width: 1.0,
                    border_color: GRAY_300,
                    border_radius: 3.0,
                }
            }
        }
    }
}
