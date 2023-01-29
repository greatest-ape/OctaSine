use iced_baseview::{
    alignment::Horizontal, widget::Checkbox, widget::Column, widget::Space, widget::Text,
    Alignment, Element, Length,
};

use crate::parameters::operator_mod_target::ModTargetStorage;
use crate::parameters::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
    OperatorParameter, Parameter, ParameterValue, WrappedParameter,
};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::{Message, FONT_SIZE, LINE_HEIGHT};

pub fn operator_2_target<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> ModTargetPicker<Operator2ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, operator_index, "TARGET", vec![0])
}

pub fn operator_3_target<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> ModTargetPicker<Operator3ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, operator_index, "TARGET", vec![1, 0])
}

pub fn operator_4_target<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> ModTargetPicker<Operator4ModulationTargetValue> {
    ModTargetPicker::new(sync_handle, operator_index, "TARGET", vec![2, 1, 0])
}

#[derive(Debug, Clone)]
pub struct ModTargetPicker<P> {
    title: String,
    parameter: WrappedParameter,
    choices: Vec<usize>,
    parameter_value: P,
}

impl<P> ModTargetPicker<P>
where
    P: 'static + ParameterValue<Value = ModTargetStorage> + Copy,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        operator_index: usize,
        title: &str,
        choices: Vec<usize>,
    ) -> Self {
        let parameter =
            Parameter::Operator(operator_index as u8, OperatorParameter::ModTargets).into();
        let sync_value = sync_handle.get_parameter(parameter);

        Self {
            title: title.into(),
            parameter,
            choices,
            parameter_value: P::new_from_patch(sync_value),
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.parameter_value = P::new_from_patch(value);
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_bold())
            .height(Length::Units(LINE_HEIGHT));

        let mut checkboxes = Column::new().spacing(4);

        for index in self.choices.iter().copied() {
            let active = self.parameter_value.get().index_active(index);
            let label = format!("{}", index + 1);
            let v = self.parameter_value.get();
            let parameter = self.parameter;

            let checkbox = Checkbox::new(active, label, move |active| {
                let mut v = v;

                v.set_index(index, active);

                let sync = P::new_from_audio(v).to_patch();

                Message::ChangeSingleParameterImmediate(parameter, sync)
            })
            .font(theme.font_regular())
            .size(FONT_SIZE)
            .text_size(FONT_SIZE)
            .spacing(4);

            checkboxes = checkboxes.push(checkbox);
        }

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .height(Length::Units(LINE_HEIGHT * 6))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(checkboxes)
            .into()
    }
}
