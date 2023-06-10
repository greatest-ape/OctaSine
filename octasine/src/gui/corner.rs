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
        glide_active::{GlideActiveValue, GLIDE_ACTIVE_STEPS},
        glide_time::GlideTimeValue,
        list::{MasterParameter, Parameter},
        master_pitch_bend_range::{MasterPitchBendRangeDownValue, MasterPitchBendRangeUpValue},
        velocity_sensitivity::VelocitySensitivityValue,
        MasterFrequencyValue, MasterVolumeValue, ParameterValue,
    },
    sync::GuiSyncHandle,
    utils::get_version_info,
};

use super::{
    boolean_button::{
        glide_bpm_sync_button, glide_mode_button, glide_retrigger_button, BooleanButton,
    },
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
    pub glide_time: OctaSineKnob<GlideTimeValue>,
    pub glide_bpm_sync: BooleanButton,
    pub glide_mode: BooleanButton,
    pub glide_retrigger: BooleanButton,
    pub glide_active: f32,
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
        let glide_time = knob::glide_time(sync_handle);

        let glide_active =
            sync_handle.get_parameter(Parameter::Master(MasterParameter::GlideActive).into());

        let glide_bpm_sync = glide_bpm_sync_button(sync_handle);
        let glide_mode = glide_mode_button(sync_handle);
        let glide_retrigger = glide_retrigger_button(sync_handle);

        Self {
            alternative_controls: false,
            master_volume,
            master_frequency,
            volume_velocity_sensitivity,
            modulation_matrix,
            patch_picker,
            master_pitch_bend_up,
            master_pitch_bend_down,
            glide_active,
            glide_time,
            glide_bpm_sync,
            glide_mode,
            glide_retrigger,
        }
    }

    pub fn theme_changed(&mut self) {
        self.patch_picker.theme_changed();
        self.modulation_matrix.theme_changed();
        self.glide_bpm_sync.theme_changed();
        self.glide_mode.theme_changed();
        self.glide_retrigger.theme_changed();
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
                .on_press(Message::ToggleAlternativeControls),
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
            let glide_mode_title = tooltip(
                theme,
                "Glide (portamento)\n\nLEG = glide only when playing legato",
                Position::Top,
                Text::new("GLIDE")
                    .horizontal_alignment(Horizontal::Center)
                    .font(theme.font_bold())
                    .height(Length::Fixed(LINE_HEIGHT.into()))
                    .width(LINE_HEIGHT * 4),
            );

            let glide_bpm_sync =
                tooltip(theme, "BPM sync", Position::Top, self.glide_bpm_sync.view());

            let glide_retrigger = tooltip(
                theme,
                "Retrigger envelopes and LFOs when gliding in monophonic mode\n(envelopes in release phase will always be retriggered)",
                Position::Top,
                self.glide_retrigger.view(),
            );

            let glide_mode = tooltip(
                theme,
                "Linear constant rate / linear constant time",
                Position::Top,
                self.glide_mode.view(),
            );

            let glide_active_picker = PickList::new(
                GLIDE_ACTIVE_STEPS,
                Some(GlideActiveValue::new_from_patch(self.glide_active).get()),
                move |option| {
                    let v = GlideActiveValue::new_from_audio(option).to_patch();

                    Message::ChangeSingleParameterImmediate(
                        Parameter::Master(MasterParameter::GlideActive).into(),
                        v,
                    )
                },
            )
            .font(theme.font_regular())
            .text_size(FONT_SIZE)
            .padding(theme.picklist_padding())
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 3)));

            Container::new(
                Column::new()
                    .width(Length::Fixed(f32::from(LINE_HEIGHT * 4)))
                    .align_items(Alignment::Center)
                    .push(glide_mode_title)
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(glide_active_picker)
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(
                        Row::new()
                            .push(glide_bpm_sync)
                            .push(Space::with_width(Length::Fixed(4.0)))
                            .push(glide_retrigger),
                    )
                    .push(Space::with_height(LINE_HEIGHT / 2))
                    .push(glide_mode),
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
            .push(container_l1(container_l2(
                Row::new()
                    .push(container_l3(self.master_volume.view(theme)))
                    .push(space_l3())
                    .push(container_l3(voice_buttons))
                    .push(space_l3())
                    .push(container_l3(self.glide_time.view(theme))),
            )))
            .push(Space::with_width(Length::Fixed(LINE_HEIGHT.into())))
            .push(triple_container(logo));

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
