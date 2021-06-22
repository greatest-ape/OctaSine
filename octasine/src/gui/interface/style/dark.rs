use iced_baseview::{
    button, container, pick_list, radio, Background, Color,
};

use super::envelope;

const SURFACE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
    0x90 as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
    0x80 as f32 / 255.0,
);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(0x0, 0x0, 0x0).into(),
            text_color: Color::from_rgb8(0x90, 0x90, 0x90).into(),
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
            background: SURFACE.into(),
	    border_width: 1.0,
	    border_color: ACTIVE,
            text_color: ACTIVE,
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
	    background: Background::from(ACTIVE),
            selected_background: Background::from(SURFACE),
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
	pick_list::Style {
	    background: Background::from(Color::BLACK),
	    text_color: ACTIVE,
	    border_color: ACTIVE,
            ..Default::default()
	}
    }
    fn hovered(&self) -> pick_list::Style {
	pick_list::Style {
	    background: Background::from(SURFACE),
	    text_color: ACTIVE,
	    border_color: ACTIVE,
            ..Default::default()
	}
    }
}

pub(super) mod knob {
    use iced_audio::knob::*;
    use super::*;

    pub const BORDER: Color = Color::from_rgb(0.315, 0.315, 0.315);

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
        color: ACTIVE,
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

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            background_color: Color::BLACK,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            time_marker_minor_color: Color::from_rgb(0.3, 0.3, 0.3),
            time_marker_color_major: Color::from_rgb(0.5, 0.5, 0.5),
            path_color: ACTIVE,
        }
    }
}
