use iced_baseview::{button, pick_list, Background, Color, Vector};

use super::envelope;

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::from_rgb(0.11, 0.42, 0.87).into(),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            selected_background: Background::from(Color::from_rgb(0.4, 0.4, 0.4)),
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        Default::default()
    }
    fn hovered(&self) -> pick_list::Style {
        Default::default()
    }
}

pub struct Envelope;

impl envelope::StyleSheet for Envelope {
    fn active(&self) -> envelope::Style {
        envelope::Style {
            time_marker_minor_color: Color::from_rgb(0.9, 0.9, 0.9),
            time_marker_color_major: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }
}
