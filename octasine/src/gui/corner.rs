use iced_baseview::{
    alignment::Horizontal, widget::tooltip::Position, widget::Button, widget::Column,
    widget::Container, widget::Row, widget::Space, widget::Text, Alignment, Element, Length,
};

use crate::{
    parameters::{MasterFrequencyValue, MasterVolumeValue},
    sync::GuiSyncHandle,
    utils::get_version_info,
};

use super::{
    common::{container_l1, container_l2, container_l3, space_l3, tooltip, triple_container},
    knob::{self, OctaSineKnob},
    mod_matrix::ModulationMatrix,
    patch_picker::PatchPicker,
    style::{container::ContainerStyle, Theme},
    Message, FONT_SIZE, LINE_HEIGHT,
};

pub struct CornerWidgets {
    pub master_volume: OctaSineKnob<MasterVolumeValue>,
    pub master_frequency: OctaSineKnob<MasterFrequencyValue>,
    pub modulation_matrix: ModulationMatrix,
    pub patch_picker: PatchPicker,
}

impl CornerWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let master_volume = knob::master_volume(sync_handle);
        let master_frequency = knob::master_frequency(sync_handle);
        let modulation_matrix = ModulationMatrix::new(sync_handle);
        let patch_picker = PatchPicker::new(sync_handle);

        Self {
            master_volume,
            master_frequency,
            modulation_matrix,
            patch_picker,
        }
    }

    pub fn theme_changed(&mut self) {
        self.modulation_matrix.theme_changed();
    }

    pub fn view(&self, theme: &Theme) -> Element<'_, Message, Theme> {
        let mod_matrix = Container::new(
            Column::new()
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                        .push(self.modulation_matrix.view())
                        // Allow room for modulation matrix extra pixel
                        .push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT - 1)))),
                )
                .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into()))),
        )
        .height(Length::Fixed(f32::from(LINE_HEIGHT * 8)))
        .width(Length::Fixed(f32::from(LINE_HEIGHT * 7)))
        .style(ContainerStyle::L3);

        let master = container_l1(container_l2(
            Row::new()
                .push(container_l3(self.master_volume.view(theme)))
                .push(space_l3())
                .push(container_l3(self.master_frequency.view(theme)))
                .push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT * 3)))), // Extend to end
        ));

        let logo = {
            let theme_button = tooltip(
                theme,
                "Switch color theme",
                Position::Top,
                Button::new(
                    Text::new("THEME")
                        .font(theme.font_regular())
                        .height(Length::Fixed(LINE_HEIGHT.into())),
                )
                .on_press(Message::SwitchTheme)
                .padding(theme.button_padding()),
            );

            let info_button = tooltip(
                theme,
                get_info_text(),
                Position::FollowCursor,
                Button::new(
                    Text::new("INFO")
                        .font(theme.font_regular())
                        .height(Length::Fixed(LINE_HEIGHT.into())),
                )
                .on_press(Message::NoOp)
                .padding(theme.button_padding()),
            );

            // Helps with issues arising from use of different font weights
            let logo_button_space = match theme {
                Theme::Dark => 3.0,
                Theme::Light => 2.0,
            };

            Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                    .push(
                        Text::new("OctaSine")
                            .size(FONT_SIZE * 3 / 2)
                            .height(Length::Fixed(f32::from(FONT_SIZE * 3 / 2)))
                            .width(Length::Fixed(f32::from(LINE_HEIGHT * 8)))
                            .font(theme.font_heading())
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
                    .push(
                        Row::new()
                            .push(theme_button)
                            .push(Space::with_width(Length::Fixed(logo_button_space)))
                            .push(info_button),
                    ),
            )
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 7)))
            .height(Length::Fixed(f32::from(LINE_HEIGHT * 6)))
        };

        Column::new()
            .push(
                Row::new()
                    .push(mod_matrix)
                    .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                    .push(master),
            )
            .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
            .push(
                Row::new()
                    .push(triple_container(self.patch_picker.view(theme)))
                    .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                    .push(triple_container(logo)),
            )
            .into()
    }
}

fn get_info_text() -> String {
    format!(
        "OctaSine frequency modulation synthesizer
Site: OctaSine.com
Build: {}
Copyright © 2019-2023 Joakim Frostegård",
        get_version_info()
    )
}
