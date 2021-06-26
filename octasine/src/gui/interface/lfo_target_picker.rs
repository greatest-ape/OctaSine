use iced_baseview::widget::{pick_list, PickList};
use iced_baseview::{Align, Column, Element, Length};

use crate::common::*;
use crate::parameters::values::{
    Lfo1TargetParameterValue, Lfo2TargetParameterValue, Lfo3TargetParameterValue,
    Lfo4TargetParameterValue, ParameterValue,
};

use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE, LINE_HEIGHT};

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
            0 => Lfo1TargetParameterValue::from_sync(sync_value).0,
            1 => Lfo2TargetParameterValue::from_sync(sync_value).0,
            2 => Lfo3TargetParameterValue::from_sync(sync_value).0,
            3 => Lfo4TargetParameterValue::from_sync(sync_value).0,
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

        let list = PickList::new(
            &mut self.state,
            &self.options[..],
            Some(self.options[self.selected].clone()),
            move |option| {
                let sync = match lfo_index {
                    0 => Lfo1TargetParameterValue::from_processing(option.value).to_sync(),
                    1 => Lfo2TargetParameterValue::from_processing(option.value).to_sync(),
                    2 => Lfo3TargetParameterValue::from_processing(option.value).to_sync(),
                    3 => Lfo4TargetParameterValue::from_processing(option.value).to_sync(),
                    _ => unreachable!(),
                };

                Message::ParameterChange(parameter_index, sync)
            },
        )
        .text_size(FONT_SIZE)
        .style(self.style)
        .width(Length::Units(LINE_HEIGHT * 12 - 3));

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 12))
            .align_items(Align::Center)
            .push(list)
            .into()
    }
}
