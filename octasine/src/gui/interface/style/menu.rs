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
                    border_color: BORDER,
                    border_radius: 0.0, // FIXME
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Appearance {
                    background: GRAY_300.into(),
                    selected_background: SURFACE_HOVER.into(),
                    text_color: TEXT,
                    selected_text_color: HOVERED,
                    border_width: 0.0,
                    border_color: TEXT,
                    border_radius: 0.0, // FIXME
                }
            }
        }
    }
}
