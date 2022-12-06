use iced_baseview::{
    widget::scrollable::{Scrollbar, Scroller, StyleSheet},
    Color,
};

use super::Theme;

// FIXME
impl StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> Scrollbar {
        Scrollbar {
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
        }
    }

    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }
}
