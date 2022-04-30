use iced_baseview::{alignment::Horizontal, Column, Container, Element, Length, Row, Rule, Space};

use crate::gui::interface::LINE_HEIGHT;

use super::{
    style::{Style, Theme},
    Message,
};

pub fn container_l1<'a, T>(theme: Theme, contents: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message>>,
{
    Container::new(contents)
        .padding(0)
        .style(theme.container_l1())
}

pub fn container_l2<'a, T>(theme: Theme, contents: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message>>,
{
    let contents = Row::new()
        .push(Space::with_width(Length::Units(LINE_HEIGHT)))
        .push(
            Column::new()
                // .push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
                .push(contents), // .push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
        )
        .push(Space::with_width(Length::Units(LINE_HEIGHT)));

    Container::new(contents)
        .padding(0)
        .style(theme.container_l2())
        .into()
}

pub fn container_l3<'a, T>(theme: Theme, contents: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message>>,
{
    let padding_x = 0;
    let padding_y = LINE_HEIGHT;

    let contents = Row::new()
        .push(Space::with_width(Length::Units(padding_x)))
        .push(
            Column::new()
                .push(Space::with_height(Length::Units(padding_y)))
                .push(contents)
                .push(Space::with_height(Length::Units(padding_y))),
        )
        .push(Space::with_width(Length::Units(padding_x)));

    Container::new(contents)
        .padding(0)
        .style(theme.container_l3())
        .into()
}

pub fn space_l2<'a>() -> Container<'a, Message> {
    Container::new(Space::with_width(Length::Units(LINE_HEIGHT)))
}

pub fn space_l3<'a>() -> Container<'a, Message> {
    Container::new(Space::with_width(Length::Units(0)))
}
