pub mod widget;

use iced_baseview::tooltip::Position;
use iced_baseview::Tooltip;
use iced_baseview::{button, Alignment, Button, Column, Element, Length, Row, Space, Text};

use crate::parameters::list::{OperatorParameter, Parameter};
use crate::parameters::operator_envelope::OperatorEnvelopeLockGroupValue;
use crate::parameters::ParameterValue;
use crate::sync::GuiSyncHandle;

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
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.widget.set_style(style);
    }

    pub fn set_lock_group(&mut self, value: f32) {
        let lock_group = OperatorEnvelopeLockGroupValue::new_from_patch(value);

        self.lock_group = lock_group;
        self.widget.set_lock_group(lock_group);
    }

    pub fn view(&mut self) -> Element<Message> {
        let sync_viewports_message = Message::EnvelopeSyncViewports {
            viewport_factor: self.widget.get_viewport_factor(),
            x_offset: self.widget.get_x_offset(),
        };

        let zoom_out = Tooltip::new(
            Button::new(
                &mut self.zoom_out,
                Text::new("−").font(self.style.font_bold()),
            )
            .on_press(Message::EnvelopeZoomOut(self.operator_index))
            .padding(self.style.button_padding())
            .style(self.style.button()),
            "Zoom out",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        let zoom_in = Tooltip::new(
            Button::new(
                &mut self.zoom_in,
                Text::new("+").font(self.style.font_bold()),
            )
            .on_press(Message::EnvelopeZoomIn(self.operator_index))
            .padding(self.style.button_padding())
            .style(self.style.button()),
            "Zoom in",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        let fit = Tooltip::new(
            Button::new(
                &mut self.zoom_to_fit,
                Text::new("FIT").font(self.style.font_regular()),
            )
            .on_press(Message::EnvelopeZoomToFit(self.operator_index))
            .padding(self.style.button_padding())
            .style(self.style.button()),
            "Zoom to fit",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        let distribute = Tooltip::new(
            Button::new(
                &mut self.sync_viewport,
                Text::new("DIST").font(self.style.font_regular()),
            )
            .on_press(sync_viewports_message)
            .padding(self.style.button_padding())
            .style(self.style.button()),
            "Distribute to other envelopes",
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
                    .push(Space::with_height(Length::Units(2)))
                    .push(fit)
                    .push(Space::with_height(Length::Units(2)))
                    .push(distribute),
            ))
            .into()
    }
}
