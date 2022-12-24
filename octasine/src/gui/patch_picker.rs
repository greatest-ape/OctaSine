use std::fmt::Display;

use iced_baseview::alignment::Horizontal;
use iced_baseview::widget::PickList;
use iced_baseview::{
    widget::Column, widget::Container, widget::Space, widget::Text, Alignment, Element, Length,
};

use super::LINE_HEIGHT;
use super::{style::Theme, GuiSyncHandle, Message, FONT_SIZE};

const ACTIONS: &[Action] = &[
    Action::RenamePatch,
    Action::SavePatch,
    Action::SaveBank,
    Action::OpenPatchesOrBank,
    Action::ClearPatch,
    Action::ClearBank,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    RenamePatch,
    SavePatch,
    SaveBank,
    OpenPatchesOrBank,
    ClearPatch,
    ClearBank,
}

impl Action {
    fn to_message(self) -> Message {
        match self {
            Self::RenamePatch => Message::RenamePatch,
            Self::SavePatch => Message::SavePatch,
            Self::SaveBank => Message::SaveBank,
            Self::OpenPatchesOrBank => Message::LoadBankOrPatch,
            Self::ClearPatch => Message::ClearPatch,
            Self::ClearBank => Message::ClearBank,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RenamePatch => write!(f, "RENAME PATCH"),
            Self::SavePatch => write!(f, "SAVE PATCH"),
            Self::SaveBank => write!(f, "SAVE BANK"),
            Self::OpenPatchesOrBank => write!(f, "OPEN PATCHES/BANK"),
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
    patch_options: Vec<Patch>,
    patch_index: usize,
}

impl PatchPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let (patch_index, patch_names) = sync_handle.get_patches();

        let patch_options = patch_names
            .into_iter()
            .enumerate()
            .map(|(index, title)| Patch { index, title })
            .collect();

        Self {
            patch_options,
            patch_index,
        }
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let patch_picker = PickList::new(
            &self.patch_options[..],
            Some(self.patch_options[self.patch_index].clone()),
            |option| Message::ChangePatch(option.index),
        )
        .font(theme.font_regular())
        .text_size(FONT_SIZE)
        .padding(theme.picklist_padding())
        .width(Length::Fill);

        let action_picker = PickList::new(ACTIONS, None, Action::to_message)
            .font(theme.font_regular())
            .text_size(FONT_SIZE)
            .padding(theme.picklist_padding())
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
                        .font(theme.font_heading())
                        // .color(theme.heading_color()) // FIXME
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
