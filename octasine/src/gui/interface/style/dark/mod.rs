pub mod colors;

use iced_baseview::Color;

use crate::gui::interface::wave_display;

use super::super::boolean_button;

use colors::*;
pub struct MuteButton;

impl boolean_button::StyleSheet for MuteButton {
    fn active(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: RED,
            text_color: RED,
        }
    }
    fn active_hover(&self) -> boolean_button::Style {
        self.active()
    }
    fn inactive(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: BORDER_DARK,
            text_color: GRAY_700,
        }
    }
    fn inactive_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: GRAY_800,
            text_color: GRAY_900,
        }
    }
}

pub struct BooleanButton;

impl boolean_button::StyleSheet for BooleanButton {
    fn active(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: BLUE,
            text_color: BLUE,
        }
    }
    fn active_hover(&self) -> boolean_button::Style {
        self.active()
    }
    fn inactive(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: BORDER_DARK,
            text_color: GRAY_700,
        }
    }
    fn inactive_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: Color::TRANSPARENT,
            border_color: GRAY_800,
            text_color: GRAY_900,
        }
    }
}
