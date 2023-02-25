use iced_baseview::widget::tooltip::Position;
use iced_baseview::{
    alignment::Horizontal, widget::Column, widget::Container, widget::Row, widget::Space,
    widget::Text, Alignment, Element, Length,
};

use crate::parameters::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
    OperatorFeedbackValue, OperatorFrequencyFineValue, OperatorFrequencyFreeValue,
    OperatorFrequencyRatioValue, OperatorMixOutValue, OperatorModOutValue, OperatorPanningValue,
    OperatorParameter, OperatorVolumeValue, OperatorWaveTypeValue, Parameter,
};
use crate::sync::GuiSyncHandle;

use super::boolean_button::{operator_mute_button, BooleanButton};
use super::common::{container_l1, container_l2, container_l3, space_l2, space_l3, tooltip};
use super::envelope::Envelope;
use super::knob::{self, OctaSineKnob};
use super::mod_target_picker;
use super::style::Theme;
use super::wave_display::WaveDisplay;
use super::wave_picker::WavePicker;
use super::{Message, FONT_SIZE, LINE_HEIGHT};

pub enum ModTargetPicker {
    Operator4(mod_target_picker::ModTargetPicker<Operator4ModulationTargetValue>),
    Operator3(mod_target_picker::ModTargetPicker<Operator3ModulationTargetValue>),
    Operator2(mod_target_picker::ModTargetPicker<Operator2ModulationTargetValue>),
}

pub struct OperatorWidgets {
    index: usize,
    pub volume: OctaSineKnob<OperatorVolumeValue>,
    pub mute_button: BooleanButton,
    pub mix: OctaSineKnob<OperatorMixOutValue>,
    pub panning: OctaSineKnob<OperatorPanningValue>,
    pub wave_type: WavePicker<OperatorWaveTypeValue>,
    pub mod_index: Option<OctaSineKnob<OperatorModOutValue>>,
    pub mod_target: Option<ModTargetPicker>,
    pub feedback: OctaSineKnob<OperatorFeedbackValue>,
    pub frequency_ratio: OctaSineKnob<OperatorFrequencyRatioValue>,
    pub frequency_free: OctaSineKnob<OperatorFrequencyFreeValue>,
    pub frequency_fine: OctaSineKnob<OperatorFrequencyFineValue>,
    pub envelope: Envelope,
    pub wave_display: WaveDisplay,
}

impl OperatorWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize) -> Self {
        let mod_index = if operator_index != 0 {
            Some(knob::operator_mod_index(sync_handle, operator_index))
        } else {
            None
        };

        let mod_target = match operator_index {
            3 => Some(ModTargetPicker::Operator4(
                mod_target_picker::operator_4_target(sync_handle, operator_index),
            )),
            2 => Some(ModTargetPicker::Operator3(
                mod_target_picker::operator_3_target(sync_handle, operator_index),
            )),
            1 => Some(ModTargetPicker::Operator2(
                mod_target_picker::operator_2_target(sync_handle, operator_index),
            )),
            _ => None,
        };

        let wave_type_parameter =
            Parameter::Operator(operator_index as u8, OperatorParameter::WaveType);

        Self {
            index: operator_index,
            volume: knob::operator_volume(sync_handle, operator_index),
            mute_button: operator_mute_button(sync_handle, operator_index),
            mix: knob::operator_mix(sync_handle, operator_index),
            panning: knob::operator_panning(sync_handle, operator_index),
            wave_type: WavePicker::new(sync_handle, wave_type_parameter, "WAVE"),
            mod_index,
            mod_target,
            feedback: knob::operator_feedback(sync_handle, operator_index),
            frequency_ratio: knob::operator_frequency_ratio(sync_handle, operator_index),
            frequency_free: knob::operator_frequency_free(sync_handle, operator_index),
            frequency_fine: knob::operator_frequency_fine(sync_handle, operator_index),
            envelope: Envelope::new(sync_handle, operator_index),
            wave_display: WaveDisplay::new(sync_handle, operator_index),
        }
    }

    pub fn theme_changed(&mut self) {
        self.mute_button.theme_changed();
        self.wave_type.theme_changed();
        self.envelope.theme_changed();
        self.wave_display.theme_changed();
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let heading = {
            let mute_button = tooltip(theme, "Toggle mute", Position::Top, self.mute_button.view());

            Container::new(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(0)
                    .push(Space::with_height(Length::Fixed(f32::from(LINE_HEIGHT))))
                    .push(
                        Row::new()
                            .width(Length::Fill)
                            .push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT))))
                            .push(mute_button),
                    )
                    .push(
                        Text::new(format!("OP {}", self.index + 1))
                            .size(FONT_SIZE + FONT_SIZE / 2)
                            .height(Length::Fixed(f32::from(FONT_SIZE + FONT_SIZE / 2)))
                            .font(theme.font_heading())
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Fixed(f32::from(
                        LINE_HEIGHT / 2,
                    ))))
                    .push(self.wave_display.view(theme)),
            )
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 8)))
            .height(Length::Fixed(f32::from(LINE_HEIGHT * 7)))
        };

        let group_1 = container_l2(
            Row::new()
                .push(container_l3(self.wave_type.view(theme)))
                .push(space_l3())
                .push(container_l3(self.volume.view(theme)))
                .push(space_l3())
                .push(container_l3(self.panning.view(theme))),
        );

        let routing_group = {
            let mut group = Row::new()
                .push(container_l3(self.mix.view(theme)))
                .push(space_l3());

            if let Some(mod_index) = self.mod_index.as_ref() {
                group = group.push(container_l3(mod_index.view(theme)));
            } else {
                group = group.push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT * 5))));
            }

            group = group.push(space_l3());

            match self.mod_target.as_ref() {
                Some(ModTargetPicker::Operator2(picker)) => {
                    group = group.push(container_l3(picker.view(theme)))
                }
                Some(ModTargetPicker::Operator3(picker)) => {
                    group = group.push(container_l3(picker.view(theme)))
                }
                Some(ModTargetPicker::Operator4(picker)) => {
                    group = group.push(container_l3(picker.view(theme)))
                }
                None => {
                    group = group.push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT * 3))))
                }
            }

            group = group.push(space_l3());
            group = group.push(container_l3(self.feedback.view(theme)));

            container_l2(group)
        };

        let frequency_group = container_l2(
            Row::new()
                .push(container_l3(self.frequency_ratio.view(theme)))
                .push(space_l3())
                .push(container_l3(self.frequency_free.view(theme)))
                .push(space_l3())
                .push(container_l3(self.frequency_fine.view(theme))),
        );

        let envelope = container_l2(self.envelope.view(theme))
            .height(Length::Fixed(f32::from(LINE_HEIGHT * 8)));

        container_l1(
            Row::new()
                .push(heading)
                .push(group_1)
                .push(space_l2())
                .push(routing_group)
                .push(space_l2())
                .push(frequency_group)
                .push(space_l2())
                .push(envelope),
        )
        .into()
    }
}
