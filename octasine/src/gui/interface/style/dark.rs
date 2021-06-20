use iced_baseview::{
    button, container, pick_list, radio, Background, Color,
};

const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x40 as f32 / 255.0,
    0x40 as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x70 as f32 / 255.0,
    0x70 as f32 / 255.0,
    0x70 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
            text_color: Color::from_rgb8(0xAA, 0xAA, 0xAA).into(),
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
            background: Color { a: 0.5, ..SURFACE }.into(),
            ..self.active()
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: ACTIVE.into(),
            border_radius: 3.0,
            text_color: Color::BLACK.into(),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: HOVERED.into(),
            text_color: Color::BLACK.into(),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1.0,
            border_color: Color::WHITE,
            ..self.hovered()
        }
    }
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            selected_background: Background::from(SURFACE),
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
	pick_list::Style {
            ..Default::default()
	}
    }
    fn hovered(&self) -> pick_list::Style {
        Default::default()
    }
}

pub(super) mod knob {
    use iced_audio::knob::*;
    use iced_baseview::Color;

    pub const BORDER: Color = Color::from_rgb(0.315, 0.315, 0.315);
    pub const LIGHT_BACK: Color = Color::from_rgb(0.97, 0.97, 0.97);

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

    pub const KNOB_BACK_HOVER: Color = Color::from_rgb(0.96, 0.96, 0.96);

    const ACTIVE_CIRCLE_STYLE: CircleStyle = CircleStyle {
        color: LIGHT_BACK,
        border_width: 1.0,
        border_color: BORDER,
        notch: NotchShape::Circle(CircleNotch {
            color: BORDER,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            diameter: StyleLength::Scaled(0.17),
            offset: StyleLength::Scaled(0.15),
        }),
    };

    pub struct Knob;

    impl iced_audio::knob::StyleSheet for Knob {
        fn active(&self) -> iced_audio::knob::Style {
            Style::Circle(ACTIVE_CIRCLE_STYLE)
        }
        fn hovered(&self) -> iced_audio::knob::Style {
            Style::Circle(CircleStyle {
                color: KNOB_BACK_HOVER,
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
