pub mod colors;
pub mod knob;

use iced_baseview::{button, checkbox, container, overlay::menu, pick_list, radio, Color, Theme};

use crate::gui::interface::wave_display;

use super::super::{boolean_button, envelope, mod_matrix, wave_picker};

use colors::*;

pub struct ContainerL0;

impl container::StyleSheet for ContainerL0 {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: BACKGROUND.into(),
            text_color: TEXT_BG.into(),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerL1;

impl iced_baseview::container::StyleSheet for ContainerL1 {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> iced_baseview::container::Appearance {
        iced_baseview::container::Appearance {
            background: Some(GRAY_100.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL2;

impl iced_baseview::container::StyleSheet for ContainerL2 {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> iced_baseview::container::Appearance {
        iced_baseview::container::Appearance {
            background: Some(GRAY_200.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct ContainerL3;

impl iced_baseview::container::StyleSheet for ContainerL3 {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> iced_baseview::container::Appearance {
        iced_baseview::container::Appearance {
            background: Some(GRAY_200.into()),
            border_radius: 4.0,
            ..Default::default()
        }
    }
}

pub struct Radio;

impl radio::StyleSheet for Radio {
    type Style = Theme;

    fn active(&self, style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: SURFACE.into(),
            dot_color: TEXT_FG,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: TEXT_FG,
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            border_color: HOVERED,
            ..self.active(style, is_selected)
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    type Style = Theme;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Color::TRANSPARENT.into(),
            checkmark_color: BLUE,
            text_color: Some(TEXT_FG),
            border_width: 1.0,
            border_color: BORDER,
            border_radius: 3.0,
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            border_color: BORDER_HOVERED,
            ..self.active(style, is_checked)
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: SURFACE.into(),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: TEXT_BG,
            text_color: TEXT_FG,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: SURFACE_HOVER.into(),
            text_color: HOVERED,
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

pub struct ValueButton;

impl button::StyleSheet for ValueButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: 3.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_FG,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: SURFACE_HOVER.into(),
            text_color: HOVERED,
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

pub struct Menu;

impl menu::StyleSheet for Menu {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            background: GRAY_300.into(),
            selected_background: SURFACE_HOVER.into(),
            text_color: TEXT_FG,
            selected_text_color: HOVERED,
            border_width: 0.0,
            border_color: TEXT_BG,
            border_radius: 0.0, // FIXME
        }
    }
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            background: SURFACE.into(),
            text_color: TEXT_FG,
            border_color: TEXT_BG,
            border_width: 0.0,
            border_radius: 3.0,
            icon_size: 0.6,
            placeholder_color: TEXT_FG,
        }
    }
    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            background: SURFACE_HOVER.into(),
            text_color: HOVERED,
            ..self.active(style)
        }
    }
}

pub struct Envelope;

impl envelope::widget::StyleSheet for Envelope {
    fn active(&self) -> envelope::widget::Style {
        envelope::widget::Style {
            background_color: GRAY_200,
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
            viewport_indicator_border: GRAY_600,
            viewport_indicator_border_active: BLUE,
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

pub struct Tooltip;

impl container::StyleSheet for Tooltip {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
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
