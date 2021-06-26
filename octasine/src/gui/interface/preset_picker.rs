use iced_baseview::widget::{pick_list, PickList};
use iced_baseview::{
    Align, Column, Element, HorizontalAlignment, Length, Row, Space, Text, VerticalAlignment,
};

use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT};

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
    pub style: Theme,
}

impl PresetPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, style: Theme) -> Self {
        let (selected, names) = sync_handle.get_presets();

        let options = names
            .into_iter()
            .enumerate()
            .map(|(index, title)| Preset { index, title })
            .collect();

        Self {
            state: pick_list::State::default(),
            options,
            selected,
            style,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("PRESET")
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .font(FONT_VERY_BOLD);

        let list = PickList::new(
            &mut self.state,
            &self.options[..],
            Some(self.options[self.selected].clone()),
            |option| Message::PresetChange(option.index),
        )
        .text_size(FONT_SIZE)
        .style(self.style)
        // Will be limited by parent, but setting a size here ensures that
        // it doesn't shrink too much when choice strings are short.
        .width(Length::Units(LINE_HEIGHT * 12 - 3));

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 12))
            .align_items(Align::Center)
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .push(title)
                    .push(Space::with_width(Length::Units(LINE_HEIGHT / 2)))
                    .push(list),
            )
            .into()
    }
}
