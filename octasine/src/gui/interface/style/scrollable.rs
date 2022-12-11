use iced_baseview::{
    widget::scrollable::{Scrollbar, Scroller, StyleSheet},
    Color,
};

use super::Theme;

// FIXME
impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> Scrollbar {
        match self {
            Self::Light => {
                use super::colors::light::*;

                Scrollbar {
                    background: SURFACE.into(),
                    border_radius: 3.0,
                    border_width: 1.0,
                    border_color: BORDER,
                    scroller: Scroller {
                        color: GRAY_600,
                        border_radius: 3.0,
                        border_width: 1.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }
            Self::Dark => Scrollbar {
                background: Color::WHITE.into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::WHITE,
                scroller: Scroller {
                    color: Color::WHITE,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::WHITE,
                },
            },
        }
    }

    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        let mut appearance = self.hovered(style);

        match self {
            Self::Light => (),
            Self::Dark => {
                use super::colors::dark::*;

                appearance.scroller.color = SURFACE_HOVER;
            }
        }

        appearance
    }

    fn hovered(&self, style: &Self::Style) -> Scrollbar {
        let mut appearance = self.active(style);

        match self {
            Self::Light => {
                use super::colors::light::*;

                appearance.scroller.color = GRAY_400;
            }
            Self::Dark => {
                use super::colors::dark::*;

                appearance.scroller.color = SURFACE_HOVER;
            }
        }

        appearance
    }
}
