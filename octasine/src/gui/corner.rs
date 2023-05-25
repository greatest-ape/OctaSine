use iced_baseview::{
    alignment::Horizontal, widget::tooltip::Position, widget::Button, widget::Column,
    widget::Container, widget::Row, widget::Space, widget::Text, Alignment, Element, Length,
};

use crate::{
    parameters::{
        master_pitch_bend_range::{MasterPitchBendRangeDownValue, MasterPitchBendRangeUpValue},
        portamento_mode::PortamentoModeValue,
        portamento_time::PortamentoTimeValue,
        velocity_sensitivity::VelocitySensitivityValue,
        voice_mode::VoiceModeValue,
        MasterFrequencyValue, MasterVolumeValue,
    },
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
    pub alternative_controls: bool,
    pub master_volume: OctaSineKnob<MasterVolumeValue>,
    pub master_frequency: OctaSineKnob<MasterFrequencyValue>,
    pub volume_velocity_sensitivity: OctaSineKnob<VelocitySensitivityValue>,
    pub modulation_matrix: ModulationMatrix,
    pub patch_picker: PatchPicker,
    pub master_pitch_bend_up: OctaSineKnob<MasterPitchBendRangeUpValue>,
    pub master_pitch_bend_down: OctaSineKnob<MasterPitchBendRangeDownValue>,
    pub voice_mode: OctaSineKnob<VoiceModeValue>,
    pub portamento_mode: OctaSineKnob<PortamentoModeValue>,
    pub portamento_time: OctaSineKnob<PortamentoTimeValue>,
}

impl CornerWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let master_volume = knob::master_volume(sync_handle);
        let master_frequency = knob::master_frequency(sync_handle);
        let volume_velocity_sensitivity = knob::master_velocity_sensitivity(sync_handle);
        let modulation_matrix = ModulationMatrix::new(sync_handle);
        let patch_picker = PatchPicker::new(sync_handle);
        let master_pitch_bend_up = knob::master_pitch_bend_range_up(sync_handle);
        let master_pitch_bend_down = knob::master_pitch_bend_range_down(sync_handle);
        let voice_mode = knob::voice_mode(sync_handle);
        let portamento_mode = knob::portamento_mode(sync_handle);
        let portamento_time = knob::portamento_time(sync_handle);

        Self {
            alternative_controls: false,
            master_volume,
            master_frequency,
            volume_velocity_sensitivity,
            modulation_matrix,
            patch_picker,
            master_pitch_bend_up,
            master_pitch_bend_down,
            voice_mode,
            portamento_mode,
            portamento_time,
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

        let logo = {
            let controls_button = tooltip(
                theme,
                "Change visible controls",
                Position::Top,
                Button::new(
                    Text::new("CONTROLS")
                        .font(theme.font_regular())
                        .height(Length::Fixed(LINE_HEIGHT.into()))
                        .horizontal_alignment(Horizontal::Center),
                )
                .padding(theme.button_padding())
                .on_press(Message::ToggleExtraControls),
            );
            let theme_button = tooltip(
                theme,
                "Switch color theme",
                Position::Bottom,
                Button::new(
                    Text::new("THEME")
                        .font(theme.font_regular())
                        .height(Length::Fixed(LINE_HEIGHT.into()))
                        .horizontal_alignment(Horizontal::Center),
                )
                .on_press(Message::SwitchTheme)
                .padding(theme.button_padding()),
            );

            Container::new(
                Column::new()
                    .align_items(Alignment::Center)
                    .width(Length::Fill)
                    .push(controls_button)
                    .push(Space::with_height(Length::Fixed(f32::from(
                        LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                    ))))
                    .push(tooltip(
                        theme,
                        get_info_text(),
                        Position::Top,
                        Text::new("OctaSine")
                            .size(FONT_SIZE * 3 / 2)
                            .height(Length::Fixed(f32::from(FONT_SIZE * 3 / 2)))
                            .width(Length::Fill)
                            .font(theme.font_heading())
                            .horizontal_alignment(Horizontal::Center),
                    ))
                    .push(Space::with_height(Length::Fixed(f32::from(
                        LINE_HEIGHT / 2 + LINE_HEIGHT / 4,
                    ))))
                    .push(theme_button),
            )
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 5)))
            .height(Length::Fixed(f32::from(LINE_HEIGHT * 6)))
        };

        let bottom = Row::new()
            .push(triple_container(logo))
            .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
            .push(container_l1(container_l2(
                Row::new()
                    .push(container_l3(self.master_volume.view(theme)))
                    .push(space_l3())
                    .push(container_l3(self.master_frequency.view(theme)))
                    .push(space_l3())
                    .push(container_l3(self.volume_velocity_sensitivity.view(theme))),
            )));

        let top: Element<Message, Theme> = if !self.alternative_controls {
            Row::new()
                .push(mod_matrix)
                .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
                .push(triple_container(self.patch_picker.view(theme)))
                .into()
        } else {
            Row::new()
                .push(container_l1(container_l2(
                    Row::new()
                        .push(container_l3(self.voice_mode.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.portamento_mode.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.portamento_time.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.master_pitch_bend_up.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.master_pitch_bend_down.view(theme))),
                )))
                .into()
        };

        Column::new()
            .push(top)
            .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
            .push(bottom)
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
