use iced_baseview::core::{Element, Length};
use iced_baseview::widget::text::LineHeight;
use iced_baseview::widget::PickList;

use crate::parameters::lfo_target::LfoTargetParameter;
use crate::parameters::{
    get_lfo_target_parameters, Lfo1TargetParameterValue, Lfo2TargetParameterValue,
    Lfo3TargetParameterValue, Lfo4TargetParameterValue, LfoParameter, Parameter, ParameterValue,
    WrappedParameter,
};

use super::LINE_HEIGHT;
use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE};

#[derive(Clone, PartialEq, Eq)]
struct LfoTarget {
    value: LfoTargetParameter,
    title: String,
}

impl ToString for LfoTarget {
    fn to_string(&self) -> String {
        self.title.clone()
    }
}

pub struct LfoTargetPicker {
    options: Vec<LfoTarget>,
    selected: usize,
    lfo_index: usize,
    parameter: WrappedParameter,
}

impl LfoTargetPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> Self {
        let parameter = Parameter::Lfo(lfo_index as u8, LfoParameter::Target).into();
        let sync_value = sync_handle.get_parameter(parameter);
        let selected = Self::get_index_from_sync(lfo_index, sync_value);
        let target_parameters = get_lfo_target_parameters(lfo_index);

        let options = target_parameters
            .iter()
            .map(|target| LfoTarget {
                value: *target,
                title: target.parameter().name().to_uppercase(),
            })
            .collect();

        Self {
            options,
            selected,
            lfo_index,
            parameter,
        }
    }

    fn get_index_from_sync(lfo_index: usize, sync_value: f32) -> usize {
        let target = match lfo_index {
            0 => Lfo1TargetParameterValue::new_from_patch(sync_value).0,
            1 => Lfo2TargetParameterValue::new_from_patch(sync_value).0,
            2 => Lfo3TargetParameterValue::new_from_patch(sync_value).0,
            3 => Lfo4TargetParameterValue::new_from_patch(sync_value).0,
            _ => unreachable!(),
        };

        let target_parameters = get_lfo_target_parameters(lfo_index);

        for (i, t) in target_parameters.iter().enumerate() {
            if *t == target {
                return i;
            }
        }

        unreachable!()
    }

    pub fn set_value(&mut self, sync_value: f32) {
        self.selected = Self::get_index_from_sync(self.lfo_index, sync_value);
    }

    pub fn view(&self, theme: &Theme) -> crate::gui::Element {
        let lfo_index = self.lfo_index;
        let parameter = self.parameter;

        PickList::new(
            &self.options[..],
            Some(self.options[self.selected].clone()),
            move |option| {
                let sync = match lfo_index {
                    0 => Lfo1TargetParameterValue::new_from_audio(option.value).to_patch(),
                    1 => Lfo2TargetParameterValue::new_from_audio(option.value).to_patch(),
                    2 => Lfo3TargetParameterValue::new_from_audio(option.value).to_patch(),
                    3 => Lfo4TargetParameterValue::new_from_audio(option.value).to_patch(),
                    _ => unreachable!(),
                };

                Message::ChangeSingleParameterImmediate(parameter, sync)
            },
        )
        .font(theme.font_regular())
        .text_size(FONT_SIZE)
        .text_line_height(LineHeight::Absolute(LINE_HEIGHT.into()))
        .padding(theme.picklist_padding())
        .width(Length::Fill)
        .into()
    }
}
