use std::fmt::Display;

use compact_str::CompactString;
use iced_baseview::core::alignment::Horizontal;
use iced_baseview::widget::tooltip::Position;
use iced_baseview::widget::{PickList, Row};
use iced_baseview::{
    core::{Element, Length},
    widget::Column, widget::Container, widget::Space, widget::Text,
};

use super::boolean_button::{voice_mode_button, BooleanButton};
use super::common::tooltip;
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
    title: CompactString,
}

impl Display for Patch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.title)
    }
}

pub struct PatchPicker {
    patch_options: Vec<Patch>,
    patch_index: usize,
    pub voice_mode_button: BooleanButton,
}

impl PatchPicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let (patch_index, patch_names) = sync_handle.get_patches();

        let patch_options = patch_names
            .into_iter()
            .enumerate()
            .map(|(index, title)| Patch { index, title })
            .collect();

        let voice_mode_button = voice_mode_button(sync_handle);

        Self {
            patch_options,
            patch_index,
            voice_mode_button,
        }
    }

    pub fn theme_changed(&mut self) {
        self.voice_mode_button.theme_changed();
    }

    pub fn view(&self, theme: &Theme) -> crate::gui::Element {
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

        let voice_mode_button = tooltip(
            theme,
            "Toggle polyphonic / monophonic voice mode",
            Position::Top,
            self.voice_mode_button.view(),
        );

        Container::new(
            Column::new()
                .push(action_picker)
                .push(Space::with_height(Length::Fixed(f32::from(
                    LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                ))))
                .push(
                    Row::new()
                        .push(Column::new().width(LINE_HEIGHT * 3))
                        .push(
                            Text::new("Patch")
                                .size(f32::from(FONT_SIZE * 3 / 2))
                                .height(Length::Fixed(f32::from(FONT_SIZE * 3 / 2)))
                                .font(theme.font_heading())
                                .horizontal_alignment(Horizontal::Center)
                                .width(LINE_HEIGHT * 6),
                        )
                        .push(Space::with_width(LINE_HEIGHT / 2))
                        .push(
                            Column::new()
                                .push(Space::with_height(3))
                                .push(voice_mode_button),
                        ),
                )
                .push(Space::with_height(Length::Fixed(f32::from(
                    LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                ))))
                .push(patch_picker),
        )
        .width(Length::Fixed(f32::from(LINE_HEIGHT * 12)))
        .height(Length::Fixed(f32::from(LINE_HEIGHT * 6)))
        .into()
    }
}
