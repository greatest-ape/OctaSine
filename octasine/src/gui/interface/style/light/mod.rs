pub mod colors;
pub mod knob;

use iced_baseview::{button, checkbox, container, pick_list, radio, Color};

use crate::gui::interface::wave_display;

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
            background: Some(GRAY_900.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL2;

impl iced_baseview::container::StyleSheet for ContainerL2 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(Color::WHITE.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL3;

impl iced_baseview::container::StyleSheet for ContainerL3 {
    fn style(&self) -> iced_baseview::container::Style {
        iced_baseview::container::Style {
            background: Some(Color::WHITE.into()),
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
            border_color: BORDER,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: SURFACE_HOVER.into(),
            ..self.active()
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: SURFACE.into(),
            checkmark_color: BLUE,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: BORDER,
            border_radius: 3.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: SURFACE_HOVER.into(),
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
            border_width: 1.0,
            border_color: BORDER,
            text_color: TEXT_FG,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: SURFACE_HOVER.into(),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        self.hovered()
    }
}

pub struct ValueButton;

impl button::StyleSheet for ValueButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::TRANSPARENT.into(),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_FG,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: SURFACE_HOVER.into(),
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
            background: SURFACE.into(),
            text_color: TEXT_FG,
            selected_background: SURFACE_HOVER.into(),
            selected_text_color: TEXT_FG,
            border_width: 1.0,
            border_color: BORDER,
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: SURFACE.into(),
            text_color: TEXT_FG,
            border_color: BORDER,
            border_width: 1.0,
            border_radius: 3.0,
            placeholder_color: TEXT_FG,
            icon_size: 0.6,
        }
    }
    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: SURFACE_HOVER.into(),
            ..self.active()
        }
    }
}

pub struct Envelope;

impl envelope::widget::StyleSheet for Envelope {
    fn active(&self) -> envelope::widget::Style {
        envelope::widget::Style {
            background_color: Color::WHITE,
            border_color: BORDER,
            drag_border_color: GRAY_700,
            text_color: TEXT_FG,
            time_marker_minor_color: GRAY_900,
            time_marker_color_major: GRAY_700,
            path_color: BLUE,
            dragger_fill_color_active: SURFACE,
            dragger_fill_color_hover: SURFACE_HOVER,
            dragger_fill_color_dragging: SURFACE_PRESS,
            dragger_border_color: BORDER,
            viewport_indicator_border: GRAY_300,
            viewport_indicator_border_active: BLUE,
        }
    }
}

pub struct ModulationMatrix;

impl mod_matrix::StyleSheet for ModulationMatrix {
    fn active(&self) -> mod_matrix::Style {
        mod_matrix::Style {
            background_color: Color::WHITE,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_FG,
            box_border_color: BORDER,
            operator_box_color_active: SURFACE,
            operator_box_border_color: Some(BORDER),
            operator_box_color_hover: SURFACE_HOVER,
            operator_box_color_dragging: SURFACE_PRESS,
            modulation_box_color_active: SURFACE,
            modulation_box_color_inactive: Color::TRANSPARENT,
            modulation_box_color_hover: SURFACE_HOVER,
            line_max_color: Color::BLACK,
            mod_out_line_color: BLUE,
            mix_out_line_color: GREEN,
        }
    }
}

pub struct WavePicker;

impl wave_picker::StyleSheet for WavePicker {
    fn active(&self) -> wave_picker::Style {
        wave_picker::Style {
            background_color: SURFACE,
            border_color_active: BORDER,
            border_color_hovered: BORDER,
            middle_line_color: GRAY_600,
            shape_line_color_active: BLUE,
            shape_line_color_hovered: BLUE,
        }
    }
}

pub struct WaveDisplay;

impl wave_display::StyleSheet for WaveDisplay {
    fn active(&self) -> wave_display::Style {
        wave_display::Style {
            background_color: SURFACE,
            border_color: BORDER,
            middle_line_color: GRAY_600,
            wave_line_color: BLUE,
        }
    }
}

pub struct MuteButton;

impl boolean_button::StyleSheet for MuteButton {
    fn active(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE,
            border_color: RED,
            text_color: RED,
        }
    }
    fn active_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE_HOVER,
            border_color: RED,
            text_color: RED,
        }
    }
    fn inactive(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE,
            border_color: BORDER,
            text_color: TEXT_FG,
        }
    }
    fn inactive_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE_HOVER,
            border_color: BORDER,
            text_color: TEXT_FG,
        }
    }
}

pub struct Tooltip;

impl container::StyleSheet for Tooltip {
    fn style(&self) -> container::Style {
        container::Style {
            background: BLUE.into(),
            text_color: Color::WHITE.into(),
            border_width: 3.0,
            border_radius: 3.0,
            border_color: BLUE,
        }
    }
}

pub struct BooleanButton;

impl boolean_button::StyleSheet for BooleanButton {
    fn active(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE,
            border_color: BLUE,
            text_color: BLUE,
        }
    }
    fn active_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE_HOVER,
            border_color: BLUE,
            text_color: BLUE,
        }
    }
    fn inactive(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE,
            border_color: BORDER,
            text_color: TEXT_FG,
        }
    }
    fn inactive_hover(&self) -> boolean_button::Style {
        boolean_button::Style {
            background_color: SURFACE_HOVER,
            border_color: BORDER,
            text_color: TEXT_FG,
        }
    }
}
