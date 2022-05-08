pub mod colors;
pub mod knob;

use iced_baseview::{button, checkbox, container, pick_list, radio, Color};

use super::super::{boolean_button, envelope, mod_matrix, wave_picker};

use colors::*;

pub struct ContainerL0;

impl container::StyleSheet for ContainerL0 {
    fn style(&self) -> container::Style {
        container::Style {
            background: BACKGROUND.into(),
            text_color: TEXT_BG.into(),
            ..container::Style::default()
        }
    }
}

pub struct ContainerL1;

impl iced_baseview::container::StyleSheet for ContainerL1 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(GRAY_100.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL2;

impl iced_baseview::container::StyleSheet for ContainerL2 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(GRAY_200.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL3;

impl iced_baseview::container::StyleSheet for ContainerL3 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(GRAY_200.into()),
            border_radius: 4.0,
            ..Default::default()
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
            background: Color::TRANSPARENT.into(),
            checkmark_color: BLUE,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: BORDER,
            border_radius: 3.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            border_color: BORDER_HOVERED,
            ..self.active(is_checked)
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: SURFACE.into(),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: TEXT_BG,
            text_color: TEXT_FG,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: SURFACE_HOVER.into(),
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
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            background: GRAY_300.into(),
            selected_background: SURFACE_HOVER.into(),
            text_color: TEXT_FG,
            selected_text_color: HOVERED,
            border_width: 0.0,
            border_color: TEXT_BG,
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: SURFACE.into(),
            text_color: TEXT_FG,
            border_color: TEXT_BG,
            border_width: 0.0,
            border_radius: 3.0,
            icon_size: 0.6,
            placeholder_color: SURFACE,
        }
    }
    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: SURFACE_HOVER.into(),
            text_color: HOVERED,
            ..self.active()
        }
    }
}

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            background_color: Color::TRANSPARENT,
            border_color: BORDER_DARK,
            drag_border_color: GRAY_400,
            text_color: TEXT_BG,
            time_marker_minor_color: GRAY_300,
            time_marker_color_major: GRAY_500,
            path_color: BLUE,
            dragger_fill_color_active: TEXT_FG,
            dragger_fill_color_hover: HOVERED,
            dragger_fill_color_dragging: PRESSED,
            dragger_border_color: SURFACE,
        }
    }
}

pub struct ModulationMatrix;

impl mod_matrix::StyleSheet for ModulationMatrix {
    fn active(&self) -> mod_matrix::Style {
        mod_matrix::Style {
            background_color: GRAY_200,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_FG,
            box_border_color: GRAY_500,
            operator_box_border_color: None,
            operator_box_color_active: SURFACE,
            operator_box_color_hover: SURFACE_HOVER,
            operator_box_color_dragging: GRAY_600,
            modulation_box_color_active: TEXT_FG,
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

pub struct Tooltip;

impl container::StyleSheet for Tooltip {
    fn style(&self) -> container::Style {
        container::Style {
            background: GRAY_200.into(),
            text_color: TEXT_FG.into(),
            border_width: 3.0,
            border_radius: 3.0,
            border_color: GRAY_200,
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
