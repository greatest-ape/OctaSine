use iced_baseview::{
    alignment::Horizontal,
    widget::tooltip::Position,
    widget::Button,
    widget::Container,
    widget::Row,
    widget::Space,
    widget::Text,
    widget::{Column, PickList},
    Alignment, Element, Length,
};

use crate::{
    parameters::{
        list::{MasterParameter, Parameter},
        master_pitch_bend_range::{MasterPitchBendRangeDownValue, MasterPitchBendRangeUpValue},
        glide_mode::{GlideMode, GlideModeValue},
        glide_time::GlideTimeValue,
        velocity_sensitivity::VelocitySensitivityValue,
        voice_mode::VoiceModeValue,
        MasterFrequencyValue, MasterVolumeValue, ParameterValue,
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
    pub glide_mode: OctaSineKnob<GlideModeValue>,
    pub glide_time: OctaSineKnob<GlideTimeValue>,
    pub voice_mode_button: BooleanButton,
    pub glide_mode_value: f32,
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
        let glide_mode = knob::glide_mode(sync_handle);
        let glide_time = knob::glide_time(sync_handle);

        let voice_mode_button = BooleanButton::new(
            sync_handle,
            Parameter::Master(MasterParameter::VoiceMode),
            "POLY",
            LINE_HEIGHT * 2 + 6,
            LINE_HEIGHT,
            |v| v < 0.5,
            |b| if b { 0.0 } else { 1.0 },
            BooleanButtonStyle::Regular,
        );

        let glide_mode_value =
            sync_handle.get_parameter(Parameter::Master(MasterParameter::GlideMode).into());

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
            glide_mode,
            glide_time,
            voice_mode_button,
            glide_mode_value,
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
            let glide_mode_title = Text::new("GLIDE")
                .horizontal_alignment(Horizontal::Center)
                .font(theme.font_bold())
                .height(Length::Fixed(LINE_HEIGHT.into()))
                .width(LINE_HEIGHT * 4);
            let glide_mode_title =
                tooltip(theme, "Glide", Position::Top, glide_mode_title);

            let voice_mode_title = Text::new("VOICES")
                .horizontal_alignment(Horizontal::Center)
                .font(theme.font_bold())
                .height(Length::Fixed(LINE_HEIGHT.into()))
                .width(LINE_HEIGHT * 4);
            let voice_mode_title =
                tooltip(theme, "Voice settings", Position::Top, voice_mode_title);

            // This order is more intuitive in the GUI
            const MODE_STEPS_REVERSE: &[GlideMode] = &[
                GlideMode::On,
                GlideMode::Auto,
                GlideMode::Off,
            ];

            let portmento_mode_picker = PickList::new(
                MODE_STEPS_REVERSE,
                Some(GlideModeValue::new_from_patch(self.glide_mode_value).get()),
                move |option| {
                    let v = GlideModeValue::new_from_audio(option).to_patch();

                    Message::ChangeSingleParameterImmediate(
                        Parameter::Master(MasterParameter::GlideMode).into(),
                        v,
                    )
                },
            )
            .font(theme.font_regular())
            .text_size(FONT_SIZE)
            .padding(theme.picklist_padding())
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 3)));

            let voice_mode_button = tooltip(
                theme,
                "Toggle polyphonic / monophonic",
                Position::Top,
                self.voice_mode_button.view(),
            );

            Container::new(
                Column::new()
                    .width(Length::Fixed(f32::from(LINE_HEIGHT * 4)))
                    .align_items(Alignment::Center)
                    .push(glide_mode_title)
                    .push(Space::with_height(LINE_HEIGHT / 4))
                    .push(portmento_mode_picker)
                    .push(Space::with_height(LINE_HEIGHT - LINE_HEIGHT / 4))
                    .push(voice_mode_title)
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(voice_mode_button),
            )
        };

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

        let bottom = Row::new()
            .push(triple_container(logo))
            .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
            .push(container_l1(container_l2(
                Row::new()
                    .push(container_l3(self.master_volume.view(theme)))
                    .push(space_l3())
                    .push(container_l3(self.glide_time.view(theme)))
                    .push(space_l3())
                    .push(container_l3(voice_buttons)),
            )));

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
