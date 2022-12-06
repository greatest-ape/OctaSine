use iced_baseview::{
    widget::container::{Appearance, StyleSheet},
    Color,
};

use super::{dark, light, Theme};

#[derive(Default)]
pub enum ContainerStyle {
    #[default]
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
                use dark::colors::*;

                match style {
                    Self::Style::L0 => Appearance {
                        background: BACKGROUND.into(),
                        text_color: TEXT.into(),
                        ..Default::default()
                    },
                    Self::Style::L1 => Appearance {
                        background: Some(GRAY_100.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::L2 => Appearance {
                        background: Some(GRAY_200.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::L3 => Appearance {
                        background: Some(GRAY_200.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::Tooltip => Appearance {
                        background: GRAY_200.into(),
                        text_color: TEXT.into(),
                        border_width: 3.0,
                        border_radius: 3.0,
                        border_color: GRAY_200,
                    },
                }
            }
            Self::Light => {
                use light::colors::*;

                match style {
                    Self::Style::L0 => Appearance {
                        background: BACKGROUND.into(),
                        text_color: TEXT.into(),
                        ..Default::default()
                    },
                    Self::Style::L1 => Appearance {
                        background: Some(GRAY_900.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::L2 => Appearance {
                        background: Some(Color::WHITE.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::L3 => Appearance {
                        background: Some(Color::WHITE.into()),
                        border_radius: 4.0,
                        ..Default::default()
                    },
                    Self::Style::Tooltip => Appearance {
                        background: BLUE.into(),
                        text_color: Color::WHITE.into(),
                        border_width: 3.0,
                        border_radius: 3.0,
                        border_color: BLUE,
                    },
                }
            }
        }
    }
}
