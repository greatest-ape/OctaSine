use std::marker::PhantomData;

use compact_str::CompactString;
use iced_baseview::core::{alignment::Horizontal, Element, Length};
use iced_baseview::widget::Button;
use iced_baseview::widget::Text;

use crate::parameters::{ParameterValue, WrappedParameter};

use super::style::button::ButtonStyle;
use super::{style::Theme, GuiSyncHandle, Message};
use super::{LINE_HEIGHT, LINE_HEIGHT_RELATIVE};

#[derive(Debug, Clone)]
pub struct ValueText<P: ParameterValue> {
    parameter: WrappedParameter,
    value_text: CompactString,
    phantom_data: PhantomData<P>,
}

impl<P: ParameterValue> ValueText<P> {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter: WrappedParameter) -> Self {
        let value_patch = sync_handle.get_parameter(parameter);
        let value_text = P::new_from_patch(value_patch).get_formatted();

        Self {
            parameter,
            value_text,
            phantom_data: Default::default(),
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value_text = P::new_from_patch(value).get_formatted();
    }

    pub fn view(&self, theme: &Theme) -> crate::gui::Element {
        Button::new(
            Text::new(self.value_text.clone())
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .font(theme.font_regular())
                .line_height(LINE_HEIGHT_RELATIVE)
                .height(Length::Fixed(LINE_HEIGHT.into())),
        )
        .padding(0)
        .width(Length::Fill)
        .style(ButtonStyle::Value)
        .on_press(Message::ChangeParameterByTextInput {
            parameter: self.parameter,
            value_text: self.value_text.clone(),
        })
        .into()
    }
}
