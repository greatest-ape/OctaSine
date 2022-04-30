pub mod knob;

use iced_baseview::{button, container, pick_list, radio, Color};
use iced_style::checkbox;

use crate::gui::interface::mute_button;

use super::{envelope, mod_matrix, wave_picker};

#[macro_export]
macro_rules! hex_gray {
    ($hex:literal) => {
        ::iced_baseview::Color::from_rgb(
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
        )
    };
}

#[macro_export]
macro_rules! hex {
    ($r:literal, $g:literal, $b:literal) => {
        ::iced_baseview::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

pub const BACKGROUND: Color = hex_gray!(0x00);
pub const SURFACE: Color = hex_gray!(0x20);
pub const TEXT_BG: Color = hex_gray!(0x90);
pub const TEXT_FG: Color = hex_gray!(0xBB);
pub const HOVERED: Color = hex_gray!(0xDD);
pub const CONTRAST: Color = hex_gray!(0x30);

pub const RED: Color = hex!(0xEF, 0x53, 0x50);
pub const BLUE: Color = hex!(0x50, 0x9D, 0xEF);
pub const GREEN: Color = hex!(0x50, 0xEF, 0x2a);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: BACKGROUND.into(),
            text_color: TEXT_BG.into(),
            ..container::Style::default()
        }
    }
}

pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: SURFACE.into(),
            dot_color: TEXT_FG,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: TEXT_FG,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            border_color: HOVERED,
            ..self.active()
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: SURFACE.into(),
            checkmark_color: TEXT_FG,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: TEXT_FG,
            border_radius: 5.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            border_color: HOVERED,
            ..self.active(is_checked)
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: CONTRAST.into(),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: TEXT_BG,
            text_color: TEXT_FG,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            border_color: HOVERED,
            text_color: HOVERED,
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        self.hovered()
    }
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            background: hex_gray!(0x20).into(),
            selected_background: CONTRAST.into(),
            text_color: TEXT_FG,
            selected_text_color: HOVERED,
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: CONTRAST.into(),
            text_color: TEXT_FG,
            border_color: TEXT_BG,
            border_radius: 0.0,
            ..Default::default()
        }
    }
    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: HOVERED,
            border_color: HOVERED,
            ..self.active()
        }
    }
}

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            background_color: Color::BLACK,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: TEXT_BG,
            time_marker_minor_color: Color::from_rgb(0.3, 0.3, 0.3),
            time_marker_color_major: Color::from_rgb(0.5, 0.5, 0.5),
            path_color: TEXT_FG,
            dragger_fill_color_active: SURFACE,
            dragger_fill_color_hover: TEXT_FG,
            dragger_border_color: TEXT_FG,
        }
    }
}

pub struct ModulationMatrix;

impl mod_matrix::StyleSheet for ModulationMatrix {
    fn active(&self) -> mod_matrix::Style {
        mod_matrix::Style {
            background_color: Color::BLACK,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: Color::BLACK,
            box_border_color: Color::from_rgb(0.5, 0.5, 0.5),
            operator_box_color_active: TEXT_FG,
            operator_box_color_hover: HOVERED,
            operator_box_color_dragging: HOVERED,
            modulation_box_color_active: TEXT_FG,
            modulation_box_color_inactive: Color::TRANSPARENT,
            modulation_box_color_hover: HOVERED,
            line_max_color: Color::WHITE,
            mod_out_line_color: Color::new(0.25, 0.5, 1.0, 1.0),
        }
    }
}

pub struct LfoShapePicker;

impl wave_picker::StyleSheet for LfoShapePicker {
    fn active(&self) -> wave_picker::Style {
        wave_picker::Style {
            background_color: SURFACE,
            border_color_active: TEXT_FG,
            border_color_hovered: HOVERED,
            middle_line_color: Color::from_rgb(0.3, 0.3, 0.3),
            shape_line_color_active: TEXT_FG,
            shape_line_color_hovered: HOVERED,
        }
    }
}

pub struct MuteButton;

impl mute_button::StyleSheet for MuteButton {
    fn volume_on(&self) -> mute_button::Style {
        mute_button::Style {
            background_color: SURFACE,
            border_color: TEXT_BG,
            text_color: TEXT_FG,
        }
    }
    fn volume_off(&self) -> mute_button::Style {
        mute_button::Style {
            background_color: SURFACE,
            border_color: RED,
            text_color: RED,
        }
    }
    fn hovered(&self) -> mute_button::Style {
        mute_button::Style {
            background_color: SURFACE,
            border_color: HOVERED,
            text_color: HOVERED,
        }
    }
}
