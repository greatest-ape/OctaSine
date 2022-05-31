pub mod widget;

use iced_baseview::alignment::Horizontal;
use iced_baseview::tooltip::Position;
use iced_baseview::{button, Alignment, Button, Column, Element, Length, Row, Space, Text};
use iced_baseview::{Font, Tooltip};

use crate::parameters::list::{OperatorParameter, Parameter};
use crate::parameters::operator_envelope::OperatorEnvelopeLockGroupValue;
use crate::parameters::ParameterValue;
use crate::sync::GuiSyncHandle;

use super::boolean_button::{envelope_group_a_button, envelope_group_b_button, BooleanButton};
use super::common::container_l3;
use super::style::Theme;
use super::{Message, LINE_HEIGHT};

pub struct Envelope {
    operator_index: usize,
    style: Theme,
    lock_group: OperatorEnvelopeLockGroupValue,
    pub widget: widget::Envelope,
    pub zoom_in: button::State,
    pub zoom_out: button::State,
    pub sync_viewport: button::State,
    pub zoom_to_fit: button::State,
    pub group_a: BooleanButton,
    pub group_b: BooleanButton,
}

impl Envelope {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let lock_group = {
            let p = Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup);

            OperatorEnvelopeLockGroupValue::new_from_patch(sync_handle.get_parameter(p))
        };

        Self {
            operator_index,
            style,
            lock_group,
            widget: widget::Envelope::new(sync_handle, operator_index, style),
            zoom_in: button::State::default(),
            zoom_out: button::State::default(),
            sync_viewport: button::State::default(),
            zoom_to_fit: button::State::default(),
            group_a: envelope_group_a_button(sync_handle, operator_index, style),
            group_b: envelope_group_b_button(sync_handle, operator_index, style),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.widget.set_style(style);
        self.group_a.set_style(style);
        self.group_b.set_style(style);
    }

    pub fn set_lock_group(&mut self, value: f32) {
        let lock_group = OperatorEnvelopeLockGroupValue::new_from_patch(value);

        self.lock_group = lock_group;
        self.widget.set_lock_group(lock_group);

        self.group_a.set_value(value);
        self.group_b.set_value(value);
    }

    pub fn is_in_group(&self, group: OperatorEnvelopeLockGroupValue) -> bool {
        group == self.lock_group && group != OperatorEnvelopeLockGroupValue::Off
    }

    pub fn view(&mut self) -> Element<Message> {
        let zoom_out = button_with_tooltip(
            self.style,
            &mut self.zoom_out,
            self.style.font_bold(),
            "−",
            Message::EnvelopeZoomOut {
                operator_index: self.operator_index as u8,
                group: self.lock_group,
            },
            "Zoom out",
        );

        let zoom_in = button_with_tooltip(
            self.style,
            &mut self.zoom_in,
            self.style.font_bold(),
            "+",
            Message::EnvelopeZoomIn {
                operator_index: self.operator_index as u8,
                group: self.lock_group,
            },
            "Zoom in",
        );

        let fit = button_with_tooltip(
            self.style,
            &mut self.zoom_to_fit,
            self.style.font_regular(),
            "F",
            Message::EnvelopeZoomToFit {
                operator_index: self.operator_index as u8,
                group: self.lock_group,
            },
            "Zoom to fit",
        );

        let distribute = button_with_tooltip(
            self.style,
            &mut self.sync_viewport,
            self.style.font_regular(),
            "D",
            Message::EnvelopeSyncViewports {
                viewport_factor: self.widget.get_viewport_factor(),
                x_offset: self.widget.get_x_offset(),
            },
            "Distribute view to other envelopes",
        );

        let group_a = Tooltip::new(
            self.group_a.view(),
            "Toggle group A membership",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        let group_b = Tooltip::new(
            self.group_b.view(),
            "Toggle group B membership",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        Row::new()
            .push(container_l3(self.style, self.widget.view()))
            .push(container_l3(
                self.style,
                Column::new()
                    .width(Length::Units(LINE_HEIGHT * 3))
                    .align_items(Alignment::End)
                    .push(
                        Row::new()
                            .push(zoom_out)
                            .push(Space::with_width(Length::Units(3)))
                            .push(zoom_in),
                    )
                    .push(Space::with_height(Length::Units(6)))
                    .push(
                        Row::new()
                            .push(fit)
                            .push(Space::with_width(Length::Units(3)))
                            .push(distribute),
                    )
                    .push(Space::with_height(Length::Units(12)))
                    .push(
                        Row::new()
                            .push(group_a)
                            .push(Space::with_width(Length::Units(3)))
                            .push(group_b),
                    ),
            ))
            .into()
    }
}

fn button_with_tooltip<'a>(
    style: Theme,
    button_state: &'a mut button::State,
    button_font: Font,
    button_text: &'static str,
    button_message: Message,
    tooltip_text: &'static str,
) -> Element<'a, Message> {
    Tooltip::new(
        Button::new(
            button_state,
            Text::new(button_text)
                .font(button_font)
                .height(Length::Units(LINE_HEIGHT))
                .width(Length::Units(9))
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(button_message)
        .padding(style.button_padding())
        .style(style.button()),
        tooltip_text,
        Position::Top,
    )
    .style(style.tooltip())
    .font(style.font_regular())
    .padding(style.tooltip_padding())
    .into()
}
