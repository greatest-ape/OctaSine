use iced_baseview::{Container, Element, Length, Column, Space, Row};

use crate::{hex_gray, gui::interface::LINE_HEIGHT};

use super::Message;

pub fn container_l1<'a, T>(contents: T, padding: u16) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
        struct Bla;

        impl iced_baseview::container::StyleSheet for Bla {
            fn style(&self) -> iced_baseview::container::Style {
                iced_baseview::container::Style {
                    background: Some(hex_gray!(0x30).into()),
                    border_radius: 4.0,
                    ..Default::default()
                }
            }
        }

        Container::new(contents).padding(padding).style(Bla)
}

pub fn container_l2<'a, T>(contents: T) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
        struct Bla;

        impl iced_baseview::container::StyleSheet for Bla {
            fn style(&self) -> iced_baseview::container::Style {
                iced_baseview::container::Style {
                    background: Some(hex_gray!(0x30).into()),
                    border_radius: 4.0,
                    ..Default::default()
                }
            }
        }

        Container::new(contents).padding(0).style(Bla).into()
}

pub fn container_l3<'a, T>(contents: T) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
        struct Bla;

        impl iced_baseview::container::StyleSheet for Bla {
            fn style(&self) -> iced_baseview::container::Style {
                iced_baseview::container::Style {
                    background: Some(hex_gray!(0x40).into()),
                    border_radius: 4.0,
                    ..Default::default()
                }
            }
        }

	let contents = Row::new()
	    .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
	    .push(
		Column::new()
			.push(Space::with_height(Length::Units(LINE_HEIGHT)))
			.push(contents)
			.push(Space::with_height(Length::Units(LINE_HEIGHT)))
	    )
	    .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)));

        Container::new(contents).padding(0).style(Bla).into()
}