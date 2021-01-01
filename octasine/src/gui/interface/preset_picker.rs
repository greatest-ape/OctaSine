use iced_baseview::{Element, Text, Column, Align, HorizontalAlignment, Length, Space};
use iced_baseview::widget::{pick_list, PickList};

use super::{FONT_BOLD, LINE_HEIGHT, FONT_SIZE, Message, GuiSyncHandle};


#[derive(Clone, PartialEq, Eq)]
struct Preset {
    index: usize,
    title: String,
}


impl ToString for Preset {
    fn to_string(&self) -> String {
        self.title.clone()
    }
}


pub struct PresetPicker {
    state: pick_list::State<Preset>,
    options: Vec<Preset>,
    selected: usize,
}


impl PresetPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let (selected, names) = sync_handle.get_presets();

        let options = names.into_iter()
            .enumerate()
            .map(|(index, title)| Preset {
                index,
                title
            })
            .collect();

        Self {
            state: pick_list::State::default(),
            options,
            selected,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("PRESET")
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);
        
        let list = PickList::new(
            &mut self.state,
            &self.options[..],
            Some(self.options[self.selected].clone()),
            |option| Message::PresetChange(option.index)
        )
            .text_size(FONT_SIZE)
            .width(Length::Units(LINE_HEIGHT * 8 - 3));
        
        Column::new()
            .width(Length::Units(LINE_HEIGHT * 8))
            .align_items(Align::Center)
            .push(title)
            .push(
                Space::with_height(Length::Units(LINE_HEIGHT))
            )
            .push(list)
            .into()
    }
}