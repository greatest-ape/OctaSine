use iced_baseview::{button, container, pick_list, radio, Background, Color};

use super::{envelope, mod_matrix};

pub const BACKGROUND: Color = Color::BLACK;

const SURFACE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

pub const ACTIVE: Color = Color::from_rgb(
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(0.7, 0.7, 0.7);
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
            background: SURFACE.into(),
            dot_color: ACTIVE,
            border_width: 1.0,
            border_color: ACTIVE,
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
            background: Color::BLACK.into(),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: ACTIVE,
            text_color: ACTIVE,
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
            background: Background::from(ACTIVE),
            selected_background: Background::from(SURFACE),
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::BLACK.into(),
            text_color: ACTIVE,
            border_color: ACTIVE,
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
        color: ACTIVE,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        diameter: StyleLength::Scaled(0.17),
        offset: StyleLength::Scaled(0.15),
    };

    const ACTIVE_CIRCLE_STYLE: CircleStyle = CircleStyle {
        color: Color::BLACK,
        border_width: 1.0,
        border_color: ACTIVE,
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
            path_color: ACTIVE,
            dragger_fill_color_active: Color::BLACK,
            dragger_fill_color_hover: ACTIVE,
            dragger_border_color: ACTIVE,
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
            operator_box_color_active: ACTIVE,
            operator_box_color_hover: HOVERED,
            operator_box_color_dragging: HOVERED,
            modulation_box_color_active: ACTIVE,
            modulation_box_color_inactive: Color::BLACK,
            line_max_color: Color::WHITE,
        }
    }
}
