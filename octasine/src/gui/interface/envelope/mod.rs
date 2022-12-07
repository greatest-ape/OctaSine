pub mod canvas;

use iced_baseview::alignment::Horizontal;
use iced_baseview::widget::tooltip::Position;
use iced_baseview::{
    widget::Button, widget::Column, widget::Row, widget::Space, widget::Text, Alignment, Element,
    Length,
};
use iced_baseview::{widget::Tooltip, Font};

use crate::parameters::list::{OperatorParameter, Parameter};
use crate::parameters::operator_envelope::OperatorEnvelopeGroupValue;
use crate::parameters::ParameterValue;
use crate::sync::GuiSyncHandle;

use super::boolean_button::{envelope_group_a_button, envelope_group_b_button, BooleanButton};
use super::common::container_l3;
use super::style::Theme;
use super::{Message, FONT_SIZE, LINE_HEIGHT};

pub struct Envelope {
    operator_index: usize,
    style: Theme,
    group: OperatorEnvelopeGroupValue,
    group_synced: bool,
    pub widget: canvas::EnvelopeCanvas,
    pub group_a: BooleanButton,
    pub group_b: BooleanButton,
}

impl Envelope {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let group = OperatorEnvelopeGroupValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup),
        ));

        let group_synced = if let OperatorEnvelopeGroupValue::Off = group {
            true
        } else {
            false
        };

        Self {
            operator_index,
            style,
            group,
            group_synced,
            widget: canvas::EnvelopeCanvas::new(sync_handle, operator_index),
            group_a: envelope_group_a_button(sync_handle, operator_index, style),
            group_b: envelope_group_b_button(sync_handle, operator_index, style),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.widget.theme_changed();
        self.group_a.set_style(style);
        self.group_b.set_style(style);
    }

    pub fn set_group(&mut self, value: f32, internal: bool) {
        let group = OperatorEnvelopeGroupValue::new_from_patch(value);

        self.group = group;
        self.widget.set_group(group, internal);

        self.group_a.set_value(value);
        self.group_b.set_value(value);
    }

    pub fn set_group_synced(&mut self, synced: bool) {
        self.group_synced = synced;
    }

    pub fn get_group(&self) -> OperatorEnvelopeGroupValue {
        self.group
    }

    pub fn is_group_member(&self, group: OperatorEnvelopeGroupValue) -> bool {
        group == self.group && group != OperatorEnvelopeGroupValue::Off
    }

    pub fn view(&self) -> Element<Message, Theme> {
        let group_synced: Element<Message, Theme> = if self.group_synced {
            Space::with_width(Length::Units(1)).into()
        } else {
            let text = Text::new("≠")
                .font(self.style.font_bold())
                .size(FONT_SIZE)
                .height(Length::Units(LINE_HEIGHT))
                .width(Length::Units(6))
                .horizontal_alignment(Horizontal::Center);

            Tooltip::new(
                text,
                "DAW automation may have affected group members",
                Position::Top,
            )
            .style(self.style.tooltip())
            .font(self.style.font_regular())
            .padding(self.style.tooltip_padding())
            .into()
        };

        let zoom_out_data = self.widget.get_zoom_out_data();
        let zoom_in_data = self.widget.get_zoom_in_data();
        let zoom_to_fit_data = self.widget.get_zoom_to_fit_data();

        let zoom_out = button_with_tooltip(
            self.style,
            self.style.font_extra_bold(),
            "−",
            Message::EnvelopeChangeViewport {
                operator_index: self.operator_index as u8,
                viewport_factor: zoom_out_data.0,
                x_offset: zoom_out_data.1,
            },
            "Zoom out",
        );

        let zoom_in = button_with_tooltip(
            self.style,
            self.style.font_extra_bold(),
            "+",
            Message::EnvelopeChangeViewport {
                operator_index: self.operator_index as u8,
                viewport_factor: zoom_in_data.0,
                x_offset: zoom_in_data.1,
            },
            "Zoom in",
        );

        let fit = button_with_tooltip(
            self.style,
            self.style.font_regular(),
            "F",
            Message::EnvelopeChangeViewport {
                operator_index: self.operator_index as u8,
                viewport_factor: zoom_to_fit_data.0,
                x_offset: zoom_to_fit_data.1,
            },
            "Zoom to fit",
        );

        let distribute = button_with_tooltip(
            self.style,
            self.style.font_regular(),
            "D",
            Message::EnvelopeDistributeViewports {
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
            .push(container_l3(self.widget.view()))
            .push(container_l3(
                Column::new()
                    .width(Length::Units(LINE_HEIGHT * 3))
                    .align_items(Alignment::End)
                    .push(
                        Row::new()
                            .push(group_synced)
                            .push(Space::with_width(Length::Units(3)))
                            .push(group_a)
                            .push(Space::with_width(Length::Units(3)))
                            .push(group_b), // .push(Space::with_width(Length::Units(3)))
                    )
                    .push(Space::with_height(Length::Units(9)))
                    .push(
                        Row::new()
                            .push(zoom_out)
                            .push(Space::with_width(Length::Units(4)))
                            .push(zoom_in),
                    )
                    .push(Space::with_height(Length::Units(6)))
                    .push(
                        Row::new()
                            .push(fit)
                            .push(Space::with_width(Length::Units(4)))
                            .push(distribute),
                    ),
            ))
            .into()
    }
}

fn button_with_tooltip<'a>(
    style: Theme,
    button_font: Font,
    button_text: &'static str,
    button_message: Message,
    tooltip_text: &'static str,
) -> Element<'a, Message, Theme> {
    Tooltip::new(
        Button::new(
            Text::new(button_text)
                .font(button_font)
                .height(Length::Units(LINE_HEIGHT))
                .width(Length::Units(10))
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
