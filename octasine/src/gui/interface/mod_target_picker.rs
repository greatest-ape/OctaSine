use iced_baseview::{
    alignment::Horizontal, Alignment, Checkbox, Column, Element, Length, Space, Text,
};

use crate::parameters::values::{
    ModTarget, Operator2ModulationTargetValue, Operator3ModulationTargetValue,
    Operator4ModulationTargetValue, ParameterValue,
};
use crate::GuiSyncHandle;

use super::style::Theme;
use super::{Message, FONT_BOLD, FONT_SIZE, LINE_HEIGHT};

pub fn operator_2_target<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> ModTargetPicker<Operator2ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, parameter_index, "TARGET", vec![0], style)
}

pub fn operator_3_target<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> ModTargetPicker<Operator3ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, parameter_index, "TARGET", vec![1, 0], style)
}

pub fn operator_4_target<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> ModTargetPicker<Operator4ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, parameter_index, "TARGET", vec![2, 1, 0], style)
}

#[derive(Debug, Clone)]
pub struct ModTargetPicker<P> {
    title: String,
    parameter_index: usize,
    pub style: Theme,
    choices: Vec<usize>,
    parameter_value: P,
}

impl<P> ModTargetPicker<P>
where
    P: 'static + ParameterValue + Copy,
    P::Value: ModTarget,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        choices: Vec<usize>,
        style: Theme,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);

        Self {
            title: title.into(),
            parameter_index,
            style,
            choices,
            parameter_value: P::from_sync(sync_value),
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.parameter_value = P::from_sync(value);
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(FONT_BOLD);

        let mut checkboxes = Column::new().spacing(4);

        for index in self.choices.iter().copied() {
            let active = self.parameter_value.get().index_active(index);
            let label = format!("{}", index + 1);
            let v = self.parameter_value.get();
            let parameter_index = self.parameter_index;

            let checkbox = Checkbox::new(active, label, move |active| {
                let mut v = v;

                v.set_index(index, active);

                let sync = P::from_processing(v).to_sync();

                Message::ChangeSingleParameterImmediate(parameter_index, sync)
            })
            .size(FONT_SIZE)
            .text_size(FONT_SIZE)
            .spacing(4)
            .style(self.style);

            checkboxes = checkboxes.push(checkbox);
        }

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(checkboxes)
            .into()
    }
}
