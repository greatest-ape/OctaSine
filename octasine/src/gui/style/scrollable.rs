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
                    background: GRAY_700.into(),
                    border_radius: 5.0,
                    border_width: 1.0,
                    border_color: Color::TRANSPARENT,
                    scroller: Scroller {
                        color: GRAY_450,
                        border_radius: 5.0,
                        border_width: 1.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }
            Self::Dark => {
                use super::colors::dark::*;

                Scrollbar {
                    background: GRAY_400.into(),
                    border_radius: 5.0,
                    border_width: 1.0,
                    border_color: GRAY_300,
                    scroller: Scroller {
                        color: GRAY_600,
                        border_radius: 5.0,
                        border_width: 1.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }
        }
    }

    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style, true)
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        let mut appearance = self.active(style);

        if is_mouse_over_scrollbar {
            match self {
                Self::Light => {
                    use super::colors::light::*;

                    appearance.scroller.color = GRAY_400;
                }
                Self::Dark => {
                    use super::colors::dark::*;

                    appearance.scroller.color = GRAY_800;
                }
            }
        }

        appearance
    }
}
