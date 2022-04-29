use iced_baseview::Container;
use iced_baseview::{
    alignment::Horizontal, Alignment, Column, Element, Length, Radio, Space, Text,
};

use crate::hex_gray;
use crate::parameter_values::lfo_mode::LfoMode;
use crate::parameter_values::{LfoBpmSyncValue, LfoModeValue, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::common::container_l2;
use super::style::Theme;
use super::{Message, FONT_BOLD, FONT_SIZE, LINE_HEIGHT};

pub fn bpm_sync<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> BooleanPicker<LfoBpmSyncValue> {
    BooleanPicker::new(
        sync_handle,
        parameter_index,
        "BPM SYNC",
        vec![true, false],
        style,
    )
}

pub fn lfo_mode<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> BooleanPicker<LfoModeValue> {
    BooleanPicker::new(
        sync_handle,
        parameter_index,
        "MODE",
        vec![LfoMode::Forever, LfoMode::Once],
        style,
    )
}

#[derive(Debug, Clone)]
pub struct BooleanPicker<P: ParameterValue> {
    title: String,
    parameter_index: usize,
    pub style: Theme,
    selected: P::Value,
    choices: Vec<P::Value>,
}

impl<P: ParameterValue> BooleanPicker<P>
where
    P::Value: Eq + Copy + 'static,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        choices: Vec<P::Value>,
        style: Theme,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let selected = P::new_from_patch(sync_value).get();

        Self {
            title: title.into(),
            parameter_index,
            style,
            choices,
            selected,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.selected = P::new_from_patch(value).get();
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(FONT_BOLD);

        let mut radios = Column::new().spacing(4);

        for choice in self.choices.clone().into_iter() {
            let parameter_index = self.parameter_index;

            let radio = Radio::new(
                choice,
                P::new_from_audio(choice).get_formatted().to_uppercase(),
                Some(self.selected),
                move |choice| {
                    Message::ChangeSingleParameterImmediate(
                        parameter_index,
                        P::new_from_audio(choice).to_patch(),
                    )
                },
            )
            .size(FONT_SIZE)
            .text_size(FONT_SIZE)
            .spacing(4)
            .style(self.style);

            radios = radios.push(radio);
        }

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(radios)
            .into()
    }
}
