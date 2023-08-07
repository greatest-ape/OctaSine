use iced_baseview::{
    widget::container::{Appearance, StyleSheet},
    core::Color,
};

use super::{colors, Theme};

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
    Transparent,
    L0,
    L1,
    L2,
    L3,
    Tooltip,
}

impl StyleSheet for Theme {
    type Style = ContainerStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match self {
            Self::Dark => {
                use colors::dark::*;

                match style {
                    Self::Style::Transparent => Appearance {
                        text_color: None,
                        background: None,
                        border_radius: 0.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    Self::Style::L0 => Appearance {
                        background: Some(BACKGROUND.into()),
                        text_color: TEXT.into(),
                        ..Default::default()
                    },
                    Self::Style::L1 => Appearance {
                        background: Some(GRAY_100.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::L2 => Appearance {
                        background: Some(GRAY_200.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::L3 => Appearance {
                        background: Some(GRAY_200.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::Tooltip => Appearance {
                        background: Some(GRAY_200.into()),
                        text_color: TEXT.into(),
                        border_width: 3.0,
                        border_radius: 3.0.into(),
                        border_color: GRAY_200,
                    },
                }
            }
            Self::Light => {
                use colors::light::*;

                match style {
                    Self::Style::Transparent => Appearance {
                        text_color: None,
                        background: None,
                        border_radius: 0.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    Self::Style::L0 => Appearance {
                        background: Some(BACKGROUND.into()),
                        text_color: TEXT.into(),
                        ..Default::default()
                    },
                    Self::Style::L1 => Appearance {
                        background: Some(GRAY_900.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::L2 => Appearance {
                        background: Some(Color::WHITE.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::L3 => Appearance {
                        background: Some(Color::WHITE.into()),
                        border_radius: 4.0.into(),
                        ..Default::default()
                    },
                    Self::Style::Tooltip => Appearance {
                        background: Some(BLUE.into()),
                        text_color: Color::WHITE.into(),
                        border_width: 3.0,
                        border_radius: 3.0.into(),
                        border_color: BLUE,
                    },
                }
            }
        }
    }
}
