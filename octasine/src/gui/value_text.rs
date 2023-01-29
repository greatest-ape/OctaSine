use std::marker::PhantomData;

use iced_baseview::alignment::Horizontal;
use iced_baseview::widget::Text;
use iced_baseview::{widget::Button, Element, Length};

use crate::parameters::{ParameterValue, WrappedParameter};

use super::style::button::ButtonStyle;
use super::LINE_HEIGHT;
use super::{style::Theme, GuiSyncHandle, Message};

#[derive(Debug, Clone)]
pub struct ValueText<P: ParameterValue> {
    parameter: WrappedParameter,
    text: String,
    phantom_data: PhantomData<P>,
}

impl<P: ParameterValue> ValueText<P> {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter: WrappedParameter) -> Self {
        let value = sync_handle.get_parameter(parameter);
        let text = P::new_from_patch(value).get_formatted();

        Self {
            parameter,
            text,
            phantom_data: Default::default(),
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.text = P::new_from_patch(value).get_formatted();
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        Button::new(
            Text::new(self.text.clone())
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .font(theme.font_regular())
                .height(Length::Units(LINE_HEIGHT)),
        )
        .padding(0)
        .width(Length::Fill)
        .style(ButtonStyle::Value)
        .on_press(Message::ChangeParameterByTextInput(
            self.parameter,
            self.text.clone(),
        ))
        .into()
    }
}
