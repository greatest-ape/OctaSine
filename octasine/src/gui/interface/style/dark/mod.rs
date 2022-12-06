pub mod colors;
pub mod knob;

use iced_baseview::{widget::button, widget::checkbox, widget::container, overlay::menu, widget::pick_list, widget::radio, Color};

use crate::gui::interface::wave_display;

use super::super::{boolean_button, envelope, mod_matrix, wave_picker};

use colors::*;



pub struct Envelope;

impl envelope::canvas::StyleSheet for Envelope {
    fn appearance(&self) -> envelope::canvas::Appearance {
        envelope::canvas::Appearance {
            background_color: GRAY_200,
            border_color: BORDER_DARK,
            drag_border_color: GRAY_400,
            text_color: TEXT,
            time_marker_minor_color: GRAY_300,
            time_marker_color_major: GRAY_500,
            path_color: BLUE,
            dragger_fill_color_active: TEXT,
            dragger_fill_color_hover: HOVERED,
            dragger_fill_color_dragging: PRESSED,
            dragger_border_color: SURFACE,
            viewport_indicator_border: GRAY_600,
            viewport_indicator_border_active: BLUE,
        }
    }
}

pub struct ModulationMatrix;

impl mod_matrix::StyleSheet for ModulationMatrix {
    fn appearance(&self) -> mod_matrix::Appearance {
        mod_matrix::Appearance {
            background_color: GRAY_200,
            border_color: Color::TRANSPARENT,
            text_color: TEXT,
            box_border_color: GRAY_500,
            operator_box_border_color: None,
            operator_box_color_active: SURFACE,
            operator_box_color_hover: SURFACE_HOVER,
            operator_box_color_dragging: GRAY_600,
            modulation_box_color_active: TEXT,
            modulation_box_color_inactive: Color::TRANSPARENT,
            modulation_box_color_hover: HOVERED,
            line_max_color: Color::WHITE,
            mod_out_line_color: BLUE,
            mix_out_line_color: GREEN,
        }
    }
}

pub struct WavePicker;

impl wave_picker::StyleSheet for WavePicker {
    fn active(&self) -> wave_picker::Style {
        wave_picker::Style {
            background_color: Color::TRANSPARENT.into(),
            border_color_active: BORDER,
            border_color_hovered: BORDER_HOVERED,
            middle_line_color: GRAY_400,
            shape_line_color_active: BLUE,
            shape_line_color_hovered: BLUE,
        }
    }
}

pub struct WaveDisplay;

impl wave_display::StyleSheet for WaveDisplay {
    fn active(&self) -> wave_display::Style {
        wave_display::Style {
            background_color: Color::TRANSPARENT.into(),
            border_color: BORDER_DARK,
            middle_line_color: GRAY_400,
            wave_line_color: BLUE,
        }
    }
}

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
