use iced_baseview::widget::{pick_list, PickList};
use iced_baseview::{Element, Length};

use crate::parameter_values::{
    get_lfo_target_parameters, Lfo1TargetParameterValue, Lfo2TargetParameterValue,
    Lfo3TargetParameterValue, Lfo4TargetParameterValue, LfoTargetParameter, ParameterValue,
};

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
    state: pick_list::State<LfoTarget>,
    options: Vec<LfoTarget>,
    selected: usize,
    lfo_index: usize,
    parameter_index: usize,
    pub style: Theme,
}

impl LfoTargetPicker {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
        style: Theme,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let selected = Self::get_index_from_sync(lfo_index, sync_value);
        let target_parameters = get_lfo_target_parameters(lfo_index);

        let options = target_parameters
            .into_iter()
            .map(|target| LfoTarget {
                value: *target,
                title: target.to_string().to_uppercase(),
            })
            .collect();

        Self {
            state: pick_list::State::default(),
            options,
            selected,
            lfo_index,
            parameter_index,
            style,
        }
    }

    fn get_index_from_sync(lfo_index: usize, sync_value: f64) -> usize {
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

    pub fn set_value(&mut self, sync_value: f64) {
        self.selected = Self::get_index_from_sync(self.lfo_index, sync_value);
    }

    pub fn view(&mut self) -> Element<Message> {
        let lfo_index = self.lfo_index;
        let parameter_index = self.parameter_index;

        PickList::new(
            &mut self.state,
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

                Message::ChangeSingleParameterImmediate(parameter_index, sync)
            },
        )
        .font(self.style.font_regular())
        .text_size(FONT_SIZE)
        .padding(self.style.picklist_padding())
        .style(self.style.pick_list())
        .width(Length::Fill)
        .into()
    }
}
