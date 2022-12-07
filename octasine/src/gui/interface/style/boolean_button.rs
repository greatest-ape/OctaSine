use iced_baseview::Color;

use crate::gui::interface::boolean_button::{Appearance, StyleSheet};

use super::Theme;

#[derive(Default, Clone, Copy)]
pub enum BooleanButtonStyle {
    #[default]
    Regular,
    Mute,
}

impl StyleSheet for Theme {
    type Style = BooleanButtonStyle;

    fn active(&self, style: &Self::Style, hover: bool) -> Appearance {
        match self {
            Self::Dark => {
                use super::dark::colors::*;

                let color = match style {
                    Self::Style::Regular => BLUE,
                    Self::Style::Mute => RED,
                };

                Appearance {
                    background_color: Color::TRANSPARENT,
                    border_color: color,
                    text_color: color,
                }
            }
            Self::Light => {
                use super::light::colors::*;

                let color = match style {
                    Self::Style::Regular => BLUE,
                    Self::Style::Mute => RED,
                };

                Appearance {
                    background_color: if hover { SURFACE_HOVER } else { SURFACE },
                    border_color: color,
                    text_color: color,
                }
            }
        }
    }

    fn inactive(&self, _style: &Self::Style, hover: bool) -> Appearance {
        match self {
            Self::Dark => {
                use super::dark::colors::*;

                if hover {
                    Appearance {
                        background_color: Color::TRANSPARENT,
                        border_color: GRAY_800,
                        text_color: GRAY_900,
                    }
                } else {
                    Appearance {
                        background_color: Color::TRANSPARENT,
                        border_color: BORDER_DARK,
                        text_color: GRAY_700,
                    }
                }
            }
            Self::Light => {
                use super::light::colors::*;

                Appearance {
                    background_color: if hover { SURFACE_HOVER } else { SURFACE },
                    border_color: BORDER,
                    text_color: TEXT,
                }
            }
        }
    }
}
