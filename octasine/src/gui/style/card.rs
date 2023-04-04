use iced_aw::native::card::StyleSheet;
use iced_aw::style::card::Appearance;
use iced_baseview::Color;

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> Appearance {
        match self {
            Self::Dark => {
                use super::colors::dark::{BACKGROUND, GRAY_100, GRAY_200, TEXT};

                Appearance {
                    background: BACKGROUND.into(),
                    border_radius: 3.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    head_background: GRAY_200.into(),
                    head_text_color: TEXT,
                    body_background: GRAY_100.into(),
                    body_text_color: TEXT,
                    foot_background: GRAY_100.into(),
                    foot_text_color: TEXT,
                    close_color: TEXT,
                }
            }
            Self::Light => {
                use super::colors::light::{BACKGROUND, BLUE, GRAY_900, TEXT};

                Appearance {
                    background: BACKGROUND.into(),
                    border_radius: 3.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    head_background: BLUE.into(),
                    head_text_color: Color::WHITE,
                    body_background: Color::WHITE.into(),
                    body_text_color: TEXT,
                    foot_background: GRAY_900.into(),
                    foot_text_color: TEXT,
                    close_color: TEXT,
                }
            }
        }
    }
}
