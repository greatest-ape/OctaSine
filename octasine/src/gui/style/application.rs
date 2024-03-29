use iced_baseview::{
    widget::application::{Appearance, StyleSheet},
    Color,
};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        match self {
            Self::Light => Appearance {
                background_color: Color::WHITE,
                text_color: Color::BLACK,
            },
            Self::Dark => Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            },
        }
    }
}
