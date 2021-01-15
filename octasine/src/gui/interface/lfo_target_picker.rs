use iced_baseview::{Element, Text, Column, Align, HorizontalAlignment, Length, Space};
use iced_baseview::widget::{pick_list, PickList};

use crate::constants::LFO_TARGET_CONTEXT_STEPS;

use super::{FONT_BOLD, LINE_HEIGHT, FONT_SIZE, Message, GuiSyncHandle};


#[derive(Clone, PartialEq, Eq)]
struct LfoTarget {
    index: usize,
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
}


impl LfoTargetPicker {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
    ) -> Self {
        let selected = 0;
        let names = LFO_TARGET_CONTEXT_STEPS.to_vec();

        let options = names.into_iter()
            .enumerate()
            .map(|(index, target)| LfoTarget {
                index,
                title: target.to_string().to_uppercase(),
            })
            .collect();

        Self {
            state: pick_list::State::default(),
            options,
            selected,
            lfo_index,
            parameter_index
        }
    }

    pub fn set_value<H: GuiSyncHandle>(
        &mut self,
        sync_handle: &H,
        value: f64,
    ){
        // FIXME
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("TARGET")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);
        
        let list = PickList::new(
            &mut self.state,
            &self.options[..],
            Some(self.options[self.selected].clone()),
            |option| Message::ParameterChange(option.index, 0.0) // FIXME
        )
            .text_size(FONT_SIZE)
            .width(Length::Units(LINE_HEIGHT * 12 - 3));
        
        Column::new()
            .width(Length::Units(LINE_HEIGHT * 12))
            .align_items(Align::Center)
            .push(list)
            .into()
    }
}