use iced_baseview::{
    alignment::Horizontal, widget::tooltip::Position, widget::Button, widget::Column,
    widget::Container, widget::Row, widget::Space, widget::Text, Alignment, Element, Length,
};

use crate::{
    parameters::{
        list::{MasterParameter, Parameter},
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
    boolean_button::BooleanButton,
    common::{container_l1, container_l2, container_l3, space_l3, tooltip, triple_container},
    knob::{self, OctaSineKnob},
    mod_matrix::ModulationMatrix,
    patch_picker::PatchPicker,
    style::{boolean_button::BooleanButtonStyle, container::ContainerStyle, Theme},
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
    pub voice_mode_button: BooleanButton,
    pub portamento_button: BooleanButton,
    pub legato_button: BooleanButton,
    pub portamento_type_button: BooleanButton,
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

        let voice_mode_button = BooleanButton::new(
            sync_handle,
            Parameter::Master(MasterParameter::VoiceMode),
            "POLY",
            LINE_HEIGHT * 3 + 4,
            LINE_HEIGHT,
            Default::default(),
            |v| v < 0.5,
            |b| if b { 0.0 } else { 1.0 },
            BooleanButtonStyle::Regular,
        );
        let portamento_button = BooleanButton::new(
            sync_handle,
            Parameter::Master(MasterParameter::PortamentoMode),
            "PORT",
            LINE_HEIGHT * 3 + 4,
            LINE_HEIGHT,
            Default::default(),
            |v| v > 0.5,
            |b| if b { 1.0 } else { 0.0 },
            BooleanButtonStyle::Regular,
        );
        let legato_button = BooleanButton::new(
            sync_handle,
            Parameter::Master(MasterParameter::PortamentoMode),
            "LEG",
            LINE_HEIGHT * 2,
            LINE_HEIGHT,
            // super::boolean_button::BooleanButtonTextAlignment::Offsets { x: 3.5, y: -0.5 },
            Default::default(),
            |v| v < 0.5,
            |b| if b { 0.0 } else { 1.0 },
            BooleanButtonStyle::Regular,
        );
        let portamento_type_button = BooleanButton::new(
            sync_handle,
            Parameter::Master(MasterParameter::PortamentoMode),
            "R",
            LINE_HEIGHT,
            LINE_HEIGHT,
            // super::boolean_button::BooleanButtonTextAlignment::Offsets { x: 3.5, y: 0.0 },
            Default::default(),
            |v| v >= 0.5,
            |b| if b { 1.0 } else { 0.0 },
            BooleanButtonStyle::Regular,
        );

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
            voice_mode_button,
            portamento_button,
            legato_button,
            portamento_type_button,
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

        let voice_buttons = {
            let title = Text::new("VOICES")
                .horizontal_alignment(Horizontal::Center)
                .font(theme.font_bold())
                .height(Length::Fixed(LINE_HEIGHT.into()))
                .width(LINE_HEIGHT * 4);
            let title = tooltip(theme, "Voice settings", Position::Top, title);

            let voice_mode_button = tooltip(
                theme,
                "Toggle polyphonic / monophonic",
                Position::Top,
                self.voice_mode_button.view(),
            );

            let portamento_button = tooltip(
                theme,
                "Toggle portamento",
                Position::Top,
                self.portamento_button.view(),
            );
            let legato_button = tooltip(
                theme,
                "Toggle legato",
                Position::Bottom,
                self.legato_button.view(),
            );
            let portamento_type_button = tooltip(
                theme,
                "Toggle constant rate portamento",
                Position::Bottom,
                self.portamento_type_button.view(),
            );

            Container::new(
                Column::new()
                    .width(Length::Fixed(f32::from(LINE_HEIGHT * 4)))
                    .align_items(Alignment::Center)
                    .push(title)
                    .push(Space::with_height(LINE_HEIGHT))
                    .push(voice_mode_button)
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(portamento_button)
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(
                        Row::new()
                            .push(legato_button)
                            .push(Space::with_width(Length::Fixed(4.0)))
                            .push(portamento_type_button)
                    )
            )
            // .height(Length::Fixed(f32::from(LINE_HEIGHT * 6)))
        };

        let bottom = Row::new()
            .push(triple_container(logo))
            .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
            .push(container_l1(container_l2(
                Row::new()
                    .push(container_l3(self.master_volume.view(theme)))
                    .push(space_l3())
                    .push(container_l3(voice_buttons))
                    .push(space_l3())
                    .push(container_l3(self.portamento_time.view(theme))), // .push(container_l3(self.volume_velocity_sensitivity.view(theme))),
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
                        .push(container_l3(self.master_frequency.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.volume_velocity_sensitivity.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.master_pitch_bend_up.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.master_pitch_bend_down.view(theme)))
                        .push(space_l3())
                        .push(container_l3(Space::with_width(LINE_HEIGHT * 4))),
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
