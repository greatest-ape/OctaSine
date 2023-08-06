use crate::gui::knob2::{Appearance, StyleSheet};
use crate::gui::Theme;

#[derive(Default, Clone, Copy)]
pub struct KnobStyle;

impl StyleSheet for Theme {
    type Style = KnobStyle;

    fn active(&self, style: Self::Style) -> Appearance {
        match self {
            Theme::Light => {
                use super::colors::light::*;

                Appearance {
                    arc_empty: GRAY_600,
                    arc_filled: BLUE,
                    notch: TEXT,
                    primary_marker: GRAY_300,
                    secondary_marker: GRAY_600,
                }
            }
            Theme::Dark => {
                use super::colors::dark::*;

                Appearance {
                    arc_empty: GRAY_500,
                    arc_filled: BLUE,
                    notch: GRAY_900,
                    primary_marker: GRAY_300,
                    secondary_marker: GRAY_600,
                }
            }
        }
    }
}
