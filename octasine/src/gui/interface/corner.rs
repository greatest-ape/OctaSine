use iced_baseview::{
    alignment::Horizontal, button, tooltip::Position, Alignment, Button, Column, Container,
    Element, Length, Row, Space, Text, Tooltip,
};

use crate::{
    get_version_info,
    parameters::{MasterFrequencyValue, MasterVolumeValue},
    sync::GuiSyncHandle,
};

use super::{
    common::{container_l1, container_l2, container_l3, space_l3, triple_container},
    knob::{self, OctaSineKnob},
    mod_matrix::ModulationMatrix,
    patch_picker::PatchPicker,
    style::Theme,
    Message, FONT_SIZE, LINE_HEIGHT,
};

pub struct CornerWidgets {
    pub style: Theme,
    pub master_volume: OctaSineKnob<MasterVolumeValue>,
    pub master_frequency: OctaSineKnob<MasterFrequencyValue>,
    pub modulation_matrix: ModulationMatrix,
    pub patch_picker: PatchPicker,
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
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.master_volume.set_style(style);
        self.master_frequency.set_style(style);
        self.modulation_matrix.set_style(style);
        self.patch_picker.style = style;
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mod_matrix = Container::new(
            Column::new()
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                        .push(self.modulation_matrix.view())
                        // Allow room for modulation matrix extra pixel
                        .push(Space::with_width(Length::Units(LINE_HEIGHT - 1))),
                )
                .push(Space::with_height(Length::Units(LINE_HEIGHT))),
        )
        .height(Length::Units(LINE_HEIGHT * 8))
        .width(Length::Units(LINE_HEIGHT * 7))
        .style(self.style.container_l3());

        let master = container_l1(
            self.style,
            container_l2(
                self.style,
                Row::new()
                    .push(container_l3(self.style, self.master_volume.view()))
                    .push(space_l3())
                    .push(container_l3(self.style, self.master_frequency.view()))
                    .push(Space::with_width(Length::Units(LINE_HEIGHT * 3))), // Extend to end
            ),
        );

        let logo = {
            let theme_button = Tooltip::new(
                Button::new(
                    Text::new("THEME")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::SwitchTheme)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                "Switch color theme",
                Position::Top,
            )
            .style(self.style.tooltip());

            let info_button = Tooltip::new(
                Button::new(
                    Text::new("INFO")
                        .font(self.style.font_regular())
                        .height(Length::Units(LINE_HEIGHT)),
                )
                .on_press(Message::NoOp)
                .padding(self.style.button_padding())
                .style(self.style.button()),
                get_info_text(),
                Position::FollowCursor,
            )
            .style(self.style.tooltip());

            // Helps with issues arising from use of different font weights
            let logo_button_space = match self.style {
                Theme::Dark => 3,
                Theme::Light => 2,
            };

            Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    // .push(Space::with_height(Length::Units(LINE_HEIGHT * 2 + LINE_HEIGHT / 4)))
                    .push(
                        Text::new("OctaSine")
                            .size(FONT_SIZE * 3 / 2)
                            .height(Length::Units(FONT_SIZE * 3 / 2))
                            .width(Length::Units(LINE_HEIGHT * 8))
                            .font(self.style.font_heading())
                            // .color(self.style.heading_color()) // FIXME
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    // .push(Space::with_height(Length::Units(LINE_HEIGHT / 2 + LINE_HEIGHT / 4)))
                    .push(
                        Row::new()
                            .push(theme_button)
                            .push(Space::with_width(Length::Units(logo_button_space)))
                            .push(info_button),
                    ),
            )
            .width(Length::Units(LINE_HEIGHT * 7))
            .height(Length::Units(LINE_HEIGHT * 6))
        };

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
                    .push(triple_container(self.style, self.patch_picker.view()))
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(triple_container(self.style, logo)),
            )
            .into()
    }
}

fn get_info_text() -> String {
    format!(
        "OctaSine frequency modulation synthesizer
Site: OctaSine.com
Build: {}
Copyright © 2019-2022 Joakim Frostegård",
        get_version_info()
    )
}
