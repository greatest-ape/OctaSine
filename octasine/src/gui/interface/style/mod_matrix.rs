use iced_baseview::Color;

use crate::gui::interface::mod_matrix::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    fn appearance(&self) -> Appearance {
        match self {
            Self::Light => {
                use super::light::colors::*;

                Appearance {
                    background_color: Color::WHITE,
                    border_color: Color::TRANSPARENT,
                    text_color: TEXT,
                    box_border_color: BORDER,
                    operator_box_color_active: SURFACE,
                    operator_box_border_color: Some(BORDER),
                    operator_box_color_hover: SURFACE_HOVER,
                    operator_box_color_dragging: SURFACE_PRESS,
                    modulation_box_color_active: SURFACE,
                    modulation_box_color_inactive: Color::TRANSPARENT,
                    modulation_box_color_hover: SURFACE_HOVER,
                    line_max_color: Color::BLACK,
                    mod_out_line_color: BLUE,
                    mix_out_line_color: GREEN,
                }
            }
            Self::Dark => {
                use super::dark::colors::*;

                Appearance {
                    background_color: GRAY_200,
                    border_color: Color::TRANSPARENT,
                    text_color: TEXT,
                    box_border_color: GRAY_500,
                    operator_box_border_color: None,
                    operator_box_color_active: SURFACE,
                    operator_box_color_hover: SURFACE_HOVER,
                    operator_box_color_dragging: GRAY_600,
                    modulation_box_color_active: TEXT,
                    modulation_box_color_inactive: Color::TRANSPARENT,
                    modulation_box_color_hover: HOVERED,
                    line_max_color: Color::WHITE,
                    mod_out_line_color: BLUE,
                    mix_out_line_color: GREEN,
                }
            }
        }
    }
}
