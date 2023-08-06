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
                    arc_empty_color: GRAY_600,
                    arc_filled_color: BLUE,
                    notch_color: TEXT,
                    anchor_dot_color: GRAY_300,
                    end_dot_color: GRAY_600,
                }
            }
            Theme::Dark => {
                use super::colors::dark::*;

                Appearance {
                    arc_empty_color: GRAY_500,
                    arc_filled_color: BLUE,
                    notch_color: GRAY_900,
                    anchor_dot_color: GRAY_300,
                    end_dot_color: GRAY_600,
                }
            }
        }
    }
}
