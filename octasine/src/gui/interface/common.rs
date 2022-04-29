use iced_baseview::{Container, Element, Length, Column, Space, Row, Rule, alignment::Horizontal};

use crate::{hex_gray, gui::interface::LINE_HEIGHT};

use super::Message;

struct ContainerL1;

impl iced_baseview::container::StyleSheet for ContainerL1 {
	fn style(&self) -> iced_baseview::container::Style {
	iced_baseview::container::Style {
		background: Some(hex_gray!(0x30).into()),
		border_radius: 4.0,
		..Default::default()
	}
	}
}


struct ContainerL2;

impl iced_baseview::container::StyleSheet for ContainerL2 {
	fn style(&self) -> iced_baseview::container::Style {
	iced_baseview::container::Style {
		background: Some(hex_gray!(0x30).into()),
		border_radius: 4.0,
		..Default::default()
	}
	}
}

struct ContainerL3;

impl iced_baseview::container::StyleSheet for ContainerL3 {
	fn style(&self) -> iced_baseview::container::Style {
	iced_baseview::container::Style {
		background: Some(hex_gray!(0x40).into()),
		border_radius: 4.0,
		..Default::default()
	}
	}
}


pub fn container_l1<'a, T>(contents: T, padding: u16) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
        Container::new(contents).padding(padding).style(ContainerL1)
}

pub fn container_l2<'a, T>(contents: T) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
	let contents = Row::new()
	    // .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
	    .push(
		Column::new()
			.push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
			.push(contents)
			.push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
	    );
	    // .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
	
        Container::new(contents).padding(0).style(ContainerL2).into()
}

pub fn container_l3<'a, T>(contents: T) -> Container<'a, Message> where T: Into<Element<'a, Message>>,{
	let contents = Row::new()
	    .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
	    .push(
		Column::new()
			.push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
			.push(contents)
			.push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
	    )
	    .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
	    ;

        Container::new(contents).padding(0).style(ContainerL3).into()
}

pub fn space_l2<'a>() -> Container<'a, Message> {
	Container::new(Space::with_width(Length::Units(LINE_HEIGHT + LINE_HEIGHT ))) // FIXME: grid (compensate for space_l3 unevenness)
}

pub fn space_l3<'a>() -> Container<'a, Message> {
	/*
	pub struct Bla;

	impl iced_baseview::rule::StyleSheet for Bla {
		fn style(&self) -> iced_baseview::rule::Style {
		    iced_baseview::rule::Style {
			color: hex_gray!(0x30),
			width: 1,
			radius: 4.0,
			fill_mode: iced_baseview::rule::FillMode::Full,
		    }
		}
	}
	Container::new(
		Rule::vertical(2).style(Bla)
	)
	.style(ContainerL3)
	.width(Length::Units(LINE_HEIGHT / 2))
	.height(Length::Units(LINE_HEIGHT * 8))
	.align_x(Horizontal::Center);
	*/

	Container::new(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
}