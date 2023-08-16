use std::borrow::Cow;

use iced_baseview::{
    core::{Element, Length},
    widget::Column,
    widget::Row,
    widget::Space,
    widget::{tooltip::Position, Container, Tooltip},
};

use super::LINE_HEIGHT;

use super::{
    style::{container::ContainerStyle, Theme},
    Message,
};

pub fn container_l1<'a, T>(contents: T) -> Container<'a, Message, crate::gui::Renderer>
where
    T: Into<crate::gui::Element<'a>>,
{
    Container::new(contents).style(ContainerStyle::L1)
}

pub fn container_l2<'a, T>(contents: T) -> Container<'a, Message, crate::gui::Renderer>
where
    T: Into<crate::gui::Element<'a>>,
{
    let padding_x = LINE_HEIGHT.into();
    let padding_y = 0.0;

    let contents = Row::new()
        .push(Space::with_width(Length::Fixed(padding_x)))
        .push(
            Column::new()
                .push(Space::with_height(Length::Fixed(padding_y)))
                .push(contents)
                .push(Space::with_height(Length::Fixed(padding_y))),
        )
        .push(Space::with_width(Length::Fixed(padding_x)));

    Container::new(contents)
        .padding(0)
        .style(ContainerStyle::L2)
}

pub fn container_l3<'a, T>(contents: T) -> Container<'a, Message, crate::gui::Renderer>
where
    T: Into<crate::gui::Element<'a>>,
{
    let padding_x = 0.0;
    let padding_y = LINE_HEIGHT.into();

    let contents = Row::new()
        .push(Space::with_width(Length::Fixed(padding_x)))
        .push(
            Column::new()
                .push(Space::with_height(Length::Fixed(padding_y)))
                .push(contents)
                .push(Space::with_height(Length::Fixed(padding_y))),
        )
        .push(Space::with_width(Length::Fixed(padding_x)));

    Container::new(contents)
        .padding(0)
        .style(ContainerStyle::L3)
}

pub fn triple_container<'a, T>(contents: T) -> Container<'a, Message, crate::gui::Renderer>
where
    T: Into<crate::gui::Element<'a>>,
{
    container_l1(container_l2(container_l3(contents)))
}

pub fn space_l2<'a>() -> Container<'a, Message, crate::gui::Renderer> {
    Container::new(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
}

pub fn space_l3<'a>() -> Container<'a, Message, crate::gui::Renderer> {
    Container::new(Space::with_width(Length::Fixed(0.0)))
}

pub fn tooltip<'a>(
    theme: &Theme,
    text: impl Into<Cow<'a, str>>,
    position: Position,
    contents: impl Into<crate::gui::Element<'a>>,
) -> Tooltip<'a, Message, crate::gui::Renderer> {
    Tooltip::new(contents, text, position)
        .font(theme.font_regular())
        .style(ContainerStyle::Tooltip)
        .padding(theme.tooltip_padding())
}
