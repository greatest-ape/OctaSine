use iced_baseview::widget::{pick_list, PickList};
use iced_baseview::{Element, Length};

use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE};

#[derive(Clone, PartialEq, Eq)]
struct Patch {
    index: usize,
    title: String,
}

impl ToString for Patch {
    fn to_string(&self) -> String {
        self.title.clone()
    }
}

pub struct PatchPicker {
    state: pick_list::State<Patch>,
    options: Vec<Patch>,
    selected: usize,
    pub style: Theme,
}

impl PatchPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, style: Theme) -> Self {
        let (selected, names) = sync_handle.get_patches();

        let options = names
            .into_iter()
            .enumerate()
            .map(|(index, title)| Patch { index, title })
            .collect();

        Self {
            state: pick_list::State::default(),
            options,
            selected,
            style,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        PickList::new(
            &mut self.state,
            &self.options[..],
            Some(self.options[self.selected].clone()),
            |option| Message::PatchChange(option.index),
        )
        .font(self.style.font_regular())
        .text_size(FONT_SIZE)
        .style(self.style.pick_list())
        .width(Length::Fill)
        .into()
    }
}
