use iced_baseview::{
    alignment::Horizontal, button, tooltip::Position, Alignment, Button, Column, Container,
    Element, Length, Row, Space, Text, Tooltip,
};

use crate::{
    parameter_values::{MasterFrequencyValue, MasterVolumeValue},
    sync::GuiSyncHandle,
};

use super::{
    common::{container_l1, container_l2, container_l3, space_l3, triple_container},
    get_info_text,
    knob::{self, OctaSineKnob},
    mod_matrix::ModulationMatrix,
    patch_picker::PatchPicker,
    style::Theme,
    Message, FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT,
};

pub struct CornerWidgets {
    pub style: Theme,
    pub master_volume: OctaSineKnob<MasterVolumeValue>,
    pub master_frequency: OctaSineKnob<MasterFrequencyValue>,
    pub modulation_matrix: ModulationMatrix,
    pub patch_picker: PatchPicker,
    toggle_info_state: button::State,
    toggle_style_state: button::State,
}

impl CornerWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let style = sync_handle.get_gui_settings().theme;

        let master_volume = knob::master_volume(sync_handle, style);
        let master_frequency = knob::master_frequency(sync_handle, style);
        let modulation_matrix = ModulationMatrix::new(sync_handle, style);
        let patch_picker = PatchPicker::new(sync_handle, style);

        Self {
            style,
            master_volume,
            master_frequency,
            modulation_matrix,
            patch_picker,
            toggle_info_state: button::State::default(),
            toggle_style_state: button::State::default(),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.master_volume.style = style;
        self.master_frequency.style = style;
        self.modulation_matrix.set_style(style);
        self.patch_picker.style = style;
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        let mod_matrix = triple_container(self.style, self.modulation_matrix.view());

        let master = container_l1(
            self.style,
            container_l2(
                self.style,
                Row::new()
                    .push(container_l3(self.style, self.master_volume.view()))
                    .push(space_l3())
                    .push(container_l3(self.style, self.master_frequency.view())),
            ),
        );

        let patch_picker = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(
                    Text::new("Patch")
                        .size(FONT_SIZE * 3 / 2)
                        .width(Length::Units(LINE_HEIGHT * 10))
                        .font(FONT_VERY_BOLD)
                        .color(self.style.heading_color())
                        .horizontal_alignment(Horizontal::Center),
                )
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(self.patch_picker.view()),
        )
        .width(Length::Units(LINE_HEIGHT * 8))
        .height(Length::Units(LINE_HEIGHT * 6));

        let logo = Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(
                    Text::new("OctaSine")
                        .size(FONT_SIZE * 3 / 2)
                        .width(Length::Units(LINE_HEIGHT * 8))
                        .font(FONT_VERY_BOLD)
                        .color(self.style.heading_color())
                        .horizontal_alignment(Horizontal::Center),
                )
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(
                    Row::new()
                        .push(
                            Tooltip::new(
                                Button::new(&mut self.toggle_info_state, Text::new("INFO"))
                                    .on_press(Message::ToggleInfo)
                                    .style(self.style.button()),
                                get_info_text(),
                                Position::FollowCursor,
                            )
                            .style(self.style.tooltip()),
                        )
                        .push(Space::with_width(Length::Units(3)))
                        .push(
                            Button::new(&mut self.toggle_style_state, Text::new("THEME"))
                                .on_press(Message::ToggleColorMode)
                                .style(self.style.button()),
                        ),
                ),
        )
        .width(Length::Units(LINE_HEIGHT * 8))
        .height(Length::Units(LINE_HEIGHT * 6));

        Column::new()
            .push(
                Row::new()
                    .push(mod_matrix)
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(master),
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(
                Row::new()
                    .push(triple_container(self.style, patch_picker))
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(triple_container(self.style, logo)),
            )
            .into()
    }
}
