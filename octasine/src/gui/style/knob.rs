use iced_audio::style::knob::{Appearance, StyleSheet, TickMarksAppearance};

use super::Theme;

#[derive(Default, Clone, Copy)]
pub enum KnobStyle {
    #[default]
    Regular,
    Bipolar,
}

impl StyleSheet for Theme {
    type Style = KnobStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        use iced_audio::knob::{
            ArcAppearance, ArcBipolarAppearance, LineCap, LineNotch, NotchShape, StyleLength,
        };

        let (filled_color, empty_color, notch_color) = match self {
            Self::Dark => {
                use super::colors::dark::*;

                (BLUE, GRAY_500, GRAY_900)
            }
            Self::Light => {
                use super::colors::light::*;

                (BLUE, GRAY_600, TEXT)
            }
        };

        let notch = NotchShape::Line(LineNotch {
            color: notch_color,
            width: StyleLength::Units(2.0),
            length: StyleLength::Units(6.0),
            cap: LineCap::Round,
            offset: StyleLength::Units(3.0),
        });

        let arc_width = StyleLength::Units(2.0);
        let arc_cap = LineCap::Square;

        match style {
            Self::Style::Regular => Appearance::Arc(ArcAppearance {
                width: arc_width,
                empty_color,
                filled_color,
                cap: arc_cap,
                notch,
            }),
            Self::Style::Bipolar => Appearance::ArcBipolar(ArcBipolarAppearance {
                width: arc_width,
                empty_color,
                left_filled_color: filled_color,
                right_filled_color: filled_color,
                cap: arc_cap,
                notch_center: notch,
                notch_left_right: None,
            }),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn tick_marks_appearance(&self, _style: &Self::Style) -> Option<TickMarksAppearance> {
        use iced_audio::style::tick_marks::{Appearance, Shape};

        let (tier_1, tier_2) = match self {
            Self::Dark => {
                use super::colors::dark::*;

                (GRAY_600, GRAY_800)
            }
            Self::Light => {
                use super::colors::light::*;

                (GRAY_600, GRAY_300)
            }
        };

        Some(TickMarksAppearance {
            style: Appearance {
                tier_1: Shape::Circle {
                    diameter: 3.0,
                    color: tier_1,
                },
                tier_2: Shape::Circle {
                    diameter: 3.0,
                    color: tier_2,
                },
                tier_3: Shape::Line {
                    length: 3.0,
                    width: 2.0,
                    color: tier_2,
                },
            },
            offset: 3.0,
        })
    }
}
