use iced_baseview::{button, Button, Element, Text};

use crate::{
    parameters::values::{OperatorVolumeToggleValue, ParameterValue},
    GuiSyncHandle,
};

use super::{style::Theme, Message, FONT_SIZE, LINE_HEIGHT};

pub struct OperatorMuteButton {
    parameter_index: usize,
    volume_on: bool,
    style: Theme,
    button_state: button::State,
}

impl OperatorMuteButton {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter_index: usize, style: Theme) -> Self {
        Self {
            parameter_index,
            volume_on: Self::volume_on(sync_handle.get_parameter(parameter_index)),
            style,
            button_state: button::State::new(),
        }
    }

    fn volume_on(sync_value: f64) -> bool {
        OperatorVolumeToggleValue::from_sync(sync_value).get() > 0.5
    }

    pub fn set_value(&mut self, value: f64) {
        self.volume_on = Self::volume_on(value);
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
    }

    pub fn view(&mut self) -> Element<Message> {
        let message = {
            let sync_value = if self.volume_on { 0.0 } else { 1.0 };

            Message::ChangeSingleParameterImmediate(self.parameter_index, sync_value)
        };

        let mut text = Text::new("M").size(FONT_SIZE);

        if !self.volume_on {
            text = text.color(iced_style::Color::new(0.906, 0.333, 0.424, 1.0));
        }

        Button::new(&mut self.button_state, text)
            .min_width(LINE_HEIGHT.into())
            .min_height(LINE_HEIGHT.into())
            .padding(2)
            .style(self.style)
            .on_press(message)
            .into()
    }
}
