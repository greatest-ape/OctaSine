use iced_baseview::{
    widget::button::{Appearance, StyleSheet},
    core::Color,
};

use super::Theme;

#[derive(Default)]
pub enum ButtonStyle {
    #[default]
    Regular,
    Value,
}

impl StyleSheet for Theme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Self::Style::Regular => match self {
                Self::Light => {
                    use super::colors::light::*;

                    Appearance {
                        background: Some(SURFACE.into()),
                        border_radius: 3.0.into(),
                        border_width: 1.0,
                        border_color: BORDER,
                        text_color: TEXT,
                        ..Default::default()
                    }
                }
                Self::Dark => {
                    use super::colors::dark::*;

                    Appearance {
                        background: Some(SURFACE.into()),
                        border_radius: 3.0.into(),
                        border_width: 0.0,
                        border_color: TEXT,
                        text_color: TEXT,
                        ..Default::default()
                    }
                }
            },
            Self::Style::Value => match self {
                Self::Light => {
                    use super::colors::light::*;

                    Appearance {
                        background: None,
                        border_radius: 3.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                        text_color: TEXT,
                        ..Default::default()
                    }
                }
                Self::Dark => {
                    use super::colors::dark::*;

                    Appearance {
                        background: None,
                        border_radius: 3.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                        text_color: TEXT,
                        ..Default::default()
                    }
                }
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            Self::Style::Regular => match self {
                Self::Light => {
                    use super::colors::light::*;

                    Appearance {
                        background: Some(SURFACE_HOVER.into()),
                        ..self.active(style)
                    }
                }
                Self::Dark => {
                    use super::colors::dark::*;

                    Appearance {
                        background: Some(SURFACE_HOVER.into()),
                        text_color: HOVERED,
                        ..self.active(style)
                    }
                }
            },
            Self::Style::Value => match self {
                Self::Light => {
                    use super::colors::light::*;

                    Appearance {
                        background: Some(SURFACE_HOVER.into()),
                        ..self.active(style)
                    }
                }
                Self::Dark => {
                    use super::colors::dark::*;

                    Appearance {
                        background: Some(SURFACE_HOVER.into()),
                        text_color: HOVERED,
                        ..self.active(style)
                    }
                }
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> Appearance {
        self.hovered(style)
    }
}
