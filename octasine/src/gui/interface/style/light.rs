use iced_baseview::{button, container, pick_list, radio, Color};
use iced_style::checkbox;

use crate::gui::interface::mute_button;

use super::{envelope, mod_matrix, wave_picker};

macro_rules! hex_gray {
    ($hex:literal) => {
        Color::from_rgb(
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
        )
    };
}

pub const BACKGROUND: Color = hex_gray!(0xFF);
pub const SURFACE: Color = hex_gray!(0xF0);
pub const SURFACE_DARK: Color = hex_gray!(0xD9);
pub const TEXT_BG: Color = hex_gray!(0x40);
pub const TEXT_FG: Color = hex_gray!(0x00);
pub const BORDER: Color = hex_gray!(0xA0);
pub const CONTRAST: Color = hex_gray!(0xD9);
pub const CONTRAST_DARK: Color = hex_gray!(0xC0);

pub const RED: Color = Color::from_rgb(0.765, 0.067, 0.176);

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
            border_radius: 5.0,
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
            background: SURFACE.into(),
            text_color: TEXT_FG,
            selected_background: hex_gray!(0x60).into(),
            selected_text_color: Color::WHITE,
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: CONTRAST.into(),
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

pub(super) mod knob {
    use super::*;
    use iced_audio::knob::*;

    pub const TICK_TIER_1: Color = Color {
        r: 0.56,
        g: 0.56,
        b: 0.56,
        a: 0.93,
    };
    pub const TICK_TIER_2: Color = Color {
        r: 0.56,
        g: 0.56,
        b: 0.56,
        a: 0.83,
    };
    pub const TICK_TIER_3: Color = Color {
        r: 0.56,
        g: 0.56,
        b: 0.56,
        a: 0.65,
    };

    const ACTIVE_CIRCLE_NOTCH_STYLE: CircleNotch = CircleNotch {
        color: TEXT_BG,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        diameter: StyleLength::Scaled(0.17),
        offset: StyleLength::Scaled(0.15),
    };

    const ACTIVE_CIRCLE_STYLE: CircleStyle = CircleStyle {
        color: SURFACE,
        border_width: 1.0,
        border_color: TEXT_BG,
        notch: NotchShape::Circle(ACTIVE_CIRCLE_NOTCH_STYLE),
    };

    pub struct Knob;

    impl iced_audio::knob::StyleSheet for Knob {
        fn active(&self) -> iced_audio::knob::Style {
            Style::Circle(ACTIVE_CIRCLE_STYLE)
        }
        fn hovered(&self) -> iced_audio::knob::Style {
            self.active()
        }
        fn dragging(&self) -> iced_audio::knob::Style {
            self.hovered()
        }
        fn tick_marks_style(&self) -> Option<TickMarksStyle> {
            Some(TickMarksStyle {
                style: iced_audio::tick_marks::Style {
                    tier_1: iced_audio::tick_marks::Shape::Circle {
                        diameter: 4.0,
                        color: TICK_TIER_1,
                    },
                    tier_2: iced_audio::tick_marks::Shape::Circle {
                        diameter: 2.0,
                        color: TICK_TIER_2,
                    },
                    tier_3: iced_audio::tick_marks::Shape::Circle {
                        diameter: 2.0,
                        color: TICK_TIER_3,
                    },
                },
                offset: 3.5,
            })
        }
    }
}

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            background_color: Color::WHITE,
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
            mod_out_line_color: Color::new(0.25, 0.5, 1.0, 1.0),
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
            border_color: BORDER,
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
