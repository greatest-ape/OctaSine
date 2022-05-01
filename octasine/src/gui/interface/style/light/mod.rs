pub mod colors;
pub mod knob;

use iced_baseview::{button, container, pick_list, radio, Color};
use iced_style::checkbox;

use crate::{gui::interface::mute_button, hex_gray};

use super::{envelope, mod_matrix, wave_picker};

use colors::*;

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
            border_color: BORDER,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: SURFACE_DARK.into(),
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
            border_color: BORDER,
            border_radius: 3.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: SURFACE_DARK.into(),
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
            border_color: BORDER,
            text_color: TEXT_FG,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: CONTRAST_DARK.into(),
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
            background: hex_gray!(0xE0).into(),
            text_color: TEXT_FG,
            selected_background: CONTRAST_DARK.into(),
            selected_text_color: TEXT_FG,
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: SURFACE.into(),
            text_color: TEXT_FG,
            border_color: BORDER,
            border_radius: 0.0,
            ..Default::default()
        }
    }
    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: CONTRAST_DARK.into(),
            ..self.active()
        }
    }
}

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            background_color: hex_gray!(0xF0).into(),
            border_color: BORDER,
            text_color: TEXT_FG,
            time_marker_minor_color: Color::from_rgb(0.9, 0.9, 0.9),
            time_marker_color_major: Color::from_rgb(0.7, 0.7, 0.7),
            path_color: TEXT_FG,
            dragger_fill_color_active: SURFACE,
            dragger_fill_color_hover: TEXT_FG,
            dragger_border_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
}

pub struct ModulationMatrix;

impl mod_matrix::StyleSheet for ModulationMatrix {
    fn active(&self) -> mod_matrix::Style {
        mod_matrix::Style {
            background_color: Color::WHITE,
            border_color: BORDER,
            text_color: TEXT_FG,
            box_border_color: BORDER,
            operator_box_color_active: SURFACE,
            operator_box_color_hover: SURFACE_DARK,
            operator_box_color_dragging: hex_gray!(0xC0),
            modulation_box_color_active: SURFACE,
            modulation_box_color_inactive: Color::TRANSPARENT,
            modulation_box_color_hover: SURFACE_DARK,
            line_max_color: Color::BLACK,
            mod_out_line_color: BLUE,
            mix_out_line_color: GREEN,
        }
    }
}

pub struct LfoShapePicker;

impl wave_picker::StyleSheet for LfoShapePicker {
    fn active(&self) -> wave_picker::Style {
        wave_picker::Style {
            background_color: SURFACE,
            border_color_active: BORDER,
            border_color_hovered: BORDER,
            middle_line_color: Color::from_rgb(0.7, 0.7, 0.7),
            shape_line_color_active: TEXT_FG,
            shape_line_color_hovered: TEXT_FG,
        }
    }
}

pub struct MuteButton;

impl mute_button::StyleSheet for MuteButton {
    fn volume_on(&self) -> mute_button::Style {
        mute_button::Style {
            background_color: SURFACE,
            border_color: BORDER,
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
            border_color: BORDER,
            text_color: TEXT_FG,
        }
    }
}

pub struct ContainerL1;

impl iced_baseview::container::StyleSheet for ContainerL1 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(hex_gray!(0xEA).into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL2;

impl iced_baseview::container::StyleSheet for ContainerL2 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(hex_gray!(0xFF).into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL3;

impl iced_baseview::container::StyleSheet for ContainerL3 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: None, //Some(hex_gray!(0x20).into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}
