use std::fmt::Display;

use iced_baseview::alignment::Horizontal;
use iced_baseview::widget::{pick_list, PickList};
use iced_baseview::{Alignment, Column, Container, Element, Length, Space, Text};

use super::LINE_HEIGHT;
use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE};

const ACTIONS: &[Action] = &[
    Action::OpenPatchesOrBank,
    Action::SavePatch,
    Action::SaveBank,
    Action::RenamePatch,
    Action::ClearPatch,
    Action::ClearBank,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    OpenPatchesOrBank,
    SavePatch,
    SaveBank,
    RenamePatch,
    ClearPatch,
    ClearBank,
}

impl Action {
    fn to_message(self) -> Message {
        match self {
            Self::SavePatch => Message::SavePatch,
            Self::SaveBank => Message::SaveBank,
            Self::OpenPatchesOrBank => Message::LoadBankOrPatch,
            Self::RenamePatch => Message::RenamePatch,
            Self::ClearPatch => Message::ClearPatch,
            Self::ClearBank => Message::ClearBank,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenPatchesOrBank => write!(f, "OPEN PATCHES/BANK"),
            Self::SavePatch => write!(f, "SAVE PATCH"),
            Self::SaveBank => write!(f, "SAVE BANK"),
            Self::RenamePatch => write!(f, "RENAME PATCH"),
            Self::ClearPatch => write!(f, "CLEAR PATCH"),
            Self::ClearBank => write!(f, "CLEAR BANK"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Patch {
    index: usize,
    title: String,
}

impl Display for Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)
    }
}

pub struct PatchPicker {
    patch_state: pick_list::State<Patch>,
    patch_options: Vec<Patch>,
    patch_index: usize,
    actions_state: pick_list::State<Action>,
    pub style: Theme,
}

impl PatchPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, style: Theme) -> Self {
        let (patch_index, patch_names) = sync_handle.get_patches();

        let patch_options = patch_names
            .into_iter()
            .enumerate()
            .map(|(index, title)| Patch { index, title })
            .collect();

        Self {
            patch_state: pick_list::State::default(),
            actions_state: Default::default(),
            patch_options,
            patch_index,
            style,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let patch_picker = PickList::new(
            &mut self.patch_state,
            &self.patch_options[..],
            Some(self.patch_options[self.patch_index].clone()),
            |option| Message::ChangePatch(option.index),
        )
        .font(self.style.font_regular())
        .text_size(FONT_SIZE)
        .padding(self.style.picklist_padding())
        .style(self.style.pick_list())
        .width(Length::Fill);

        let action_picker =
            PickList::new(&mut self.actions_state, ACTIONS, None, Action::to_message)
                .font(self.style.font_regular())
                .text_size(FONT_SIZE)
                .padding(self.style.picklist_padding())
                .style(self.style.pick_list())
                .placeholder("ACTIONS..")
                .width(Length::Fill);

        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(action_picker)
                .push(Space::with_height(Length::Units(
                    LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                )))
                .push(
                    Text::new("Patch")
                        .size(FONT_SIZE * 3 / 2)
                        .height(Length::Units(FONT_SIZE * 3 / 2))
                        .width(Length::Units(LINE_HEIGHT * 10))
                        .font(self.style.font_heading())
                        .color(self.style.heading_color())
                        .horizontal_alignment(Horizontal::Center),
                )
                .push(Space::with_height(Length::Units(
                    LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                )))
                .push(patch_picker),
        )
        .width(Length::Units(LINE_HEIGHT * 9))
        .height(Length::Units(LINE_HEIGHT * 6))
        .into()
    }
}
