use iced_baseview::Color;

use crate::gui::interface::envelope::canvas::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    fn appearance(&self) -> Appearance {
        match self {
            Self::Light => {
                use super::light::colors::*;

                Appearance {
                    background_color: Color::WHITE,
                    border_color: BORDER,
                    drag_border_color: GRAY_700,
                    text_color: TEXT,
                    time_marker_minor_color: GRAY_900,
                    time_marker_color_major: GRAY_700,
                    path_color: BLUE,
                    dragger_fill_color_active: SURFACE,
                    dragger_fill_color_hover: SURFACE_HOVER,
                    dragger_fill_color_dragging: SURFACE_PRESS,
                    dragger_border_color: BORDER,
                    viewport_indicator_border: GRAY_300,
                    viewport_indicator_border_active: BLUE,
                }
            }
            Self::Dark => {
                use super::dark::colors::*;

                Appearance {
                    background_color: GRAY_200,
                    border_color: BORDER_DARK,
                    drag_border_color: GRAY_400,
                    text_color: TEXT,
                    time_marker_minor_color: GRAY_300,
                    time_marker_color_major: GRAY_500,
                    path_color: BLUE,
                    dragger_fill_color_active: TEXT,
                    dragger_fill_color_hover: HOVERED,
                    dragger_fill_color_dragging: PRESSED,
                    dragger_border_color: SURFACE,
                    viewport_indicator_border: GRAY_600,
                    viewport_indicator_border_active: BLUE,
                }
            }
        }
    }
}
