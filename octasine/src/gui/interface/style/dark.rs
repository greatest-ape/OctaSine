use iced_baseview::{button, container, pick_list, radio, Color};

use super::{divider, envelope, mod_matrix};

macro_rules! hex_gray {
    ($hex:literal) => {
        Color::from_rgb(
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
            $hex as f32 / 255.0,
        )
    };
}

pub const BACKGROUND: Color = Color::BLACK;

pub const ACTIVE: Color = Color::from_rgb(
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
);

pub const DARK_GRAY: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub const GRAY: Color = ACTIVE;

pub const LIGHT_GRAY: Color = Color::from_rgb(
    0xBB as f32 / 255.0,
    0xBB as f32 / 255.0,
    0xBB as f32 / 255.0,
);

pub const HOVERED: Color = Color::from_rgb(
    0xDD as f32 / 255.0,
    0xDD as f32 / 255.0,
    0xDD as f32 / 255.0,
);

pub const CONTRAST: Color = Color::from_rgb(
    0x19 as f32 / 255.0,
    0x2E as f32 / 255.0,
    0x4D as f32 / 255.0,
);

const DRAGGING: Color = Color::from_rgb(0.9, 0.9, 0.9);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: BACKGROUND.into(),
            text_color: ACTIVE.into(),
            ..container::Style::default()
        }
    }
}

pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: DARK_GRAY.into(),
            dot_color: LIGHT_GRAY,
            border_width: 1.0,
            border_color: LIGHT_GRAY,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            border_color: HOVERED,
            ..self.active()
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
            border_color: GRAY,
            text_color: LIGHT_GRAY,
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
        button::Style {
            border_color: DRAGGING,
            ..self.hovered()
        }
    }
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            background: hex_gray!(0x40).into(),
            selected_background: CONTRAST.into(),
            text_color: LIGHT_GRAY,
            selected_text_color: LIGHT_GRAY,
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: CONTRAST.into(),
            text_color: LIGHT_GRAY,
            border_color: GRAY,
            border_radius: 3.0,
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
        color: LIGHT_GRAY,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        diameter: StyleLength::Scaled(0.17),
        offset: StyleLength::Scaled(0.15),
    };

    const ACTIVE_CIRCLE_STYLE: CircleStyle = CircleStyle {
        color: DARK_GRAY,
        border_width: 1.0,
        border_color: LIGHT_GRAY,
        notch: NotchShape::Circle(ACTIVE_CIRCLE_NOTCH_STYLE),
    };

    pub struct Knob;

    impl iced_audio::knob::StyleSheet for Knob {
        fn active(&self) -> iced_audio::knob::Style {
            Style::Circle(ACTIVE_CIRCLE_STYLE)
        }
        fn hovered(&self) -> iced_audio::knob::Style {
            Style::Circle(CircleStyle {
                border_color: HOVERED,
                notch: NotchShape::Circle(CircleNotch {
                    color: HOVERED,
                    ..ACTIVE_CIRCLE_NOTCH_STYLE
                }),
                ..ACTIVE_CIRCLE_STYLE
            })
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
            background_color: Color::BLACK,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: ACTIVE,
            time_marker_minor_color: Color::from_rgb(0.3, 0.3, 0.3),
            time_marker_color_major: Color::from_rgb(0.5, 0.5, 0.5),
            path_color: LIGHT_GRAY,
            dragger_fill_color_active: DARK_GRAY,
            dragger_fill_color_hover: LIGHT_GRAY,
            dragger_border_color: LIGHT_GRAY,
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
            operator_box_color_active: LIGHT_GRAY,
            operator_box_color_hover: HOVERED,
            operator_box_color_dragging: HOVERED,
            modulation_box_color_active: LIGHT_GRAY,
            modulation_box_color_inactive: DARK_GRAY,
            line_max_color: Color::WHITE,
        }
    }
}

pub struct Divider;

impl divider::StyleSheet for Divider {
    fn active(&self) -> divider::Style {
        divider::Style {
            // Really dubious anti-aliasing issue workaround
            #[cfg(feature = "gui_glow")]
            color: Color::from_rgb8(0x70, 0x70, 0x70),
            #[cfg(feature = "gui_wgpu")]
            color: Color::from_rgb(0.8, 0.8, 0.8),
        }
    }
}
