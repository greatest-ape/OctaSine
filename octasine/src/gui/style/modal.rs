use iced_aw::native::modal::StyleSheet;
use iced_aw::style::modal::Appearance;
use iced_baseview::Color;

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> Appearance {
        match self {
            Self::Dark => {
                let mut color = Color::BLACK;

                color.a = 0.75;

                Appearance {
                    background: color.into(),
                }
            }
            Self::Light => {
                let mut color = Color::BLACK;

                color.a = 0.5;

                Appearance {
                    background: color.into(),
                }
            }
        }
    }
}
