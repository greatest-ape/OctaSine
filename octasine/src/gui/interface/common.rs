use iced_baseview::{widget::Column, widget::Container, Element, Length, widget::Row, widget::Space};

use crate::gui::interface::LINE_HEIGHT;

use super::{style::{Theme, container::ContainerStyle}, Message};

pub fn container_l1<'a, T>(theme: Theme, contents: T) -> Container<'a, Message, Theme>
where
    T: Into<Element<'a, Message, Theme>>,
{
    Container::new(contents).style(ContainerStyle::L1)
}

pub fn container_l2<'a, T>(theme: Theme, contents: T) -> Container<'a, Message, Theme>
where
    T: Into<Element<'a, Message, Theme>>,
{
    let padding_x = LINE_HEIGHT;
    let padding_y = 0;

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
        .style(ContainerStyle::L2)
        .into()
}

pub fn container_l3<'a, T>(theme: Theme, contents: T) -> Container<'a, Message, Theme>
where
    T: Into<Element<'a, Message, Theme>>,
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
        .style(ContainerStyle::L3)
        .into()
}

pub fn triple_container<'a, T>(theme: Theme, contents: T) -> Container<'a, Message, Theme>
where
    T: Into<Element<'a, Message, Theme>>,
{
    container_l1(theme, container_l2(theme, container_l3(theme, contents)))
}

pub fn space_l2<'a>() -> Container<'a, Message, Theme> {
    Container::new(Space::with_width(Length::Units(LINE_HEIGHT)))
}

pub fn space_l3<'a>() -> Container<'a, Message, Theme> {
    Container::new(Space::with_width(Length::Units(0)))
}
