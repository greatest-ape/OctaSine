use iced_baseview::widget::tooltip::Position;
use iced_baseview::widget::Tooltip;
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
use super::common::{container_l1, container_l2, container_l3, space_l2, space_l3};
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
    style: Theme,
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
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let mod_index = if operator_index != 0 {
            Some(knob::operator_mod_index(sync_handle, operator_index, style))
        } else {
            None
        };

        let mod_target = match operator_index {
            3 => Some(ModTargetPicker::Operator4(
                mod_target_picker::operator_4_target(sync_handle, operator_index, style),
            )),
            2 => Some(ModTargetPicker::Operator3(
                mod_target_picker::operator_3_target(sync_handle, operator_index, style),
            )),
            1 => Some(ModTargetPicker::Operator2(
                mod_target_picker::operator_2_target(sync_handle, operator_index, style),
            )),
            _ => None,
        };

        let wave_type_parameter =
            Parameter::Operator(operator_index as u8, OperatorParameter::WaveType);

        Self {
            index: operator_index,
            style,
            volume: knob::operator_volume(sync_handle, operator_index, style),
            mute_button: operator_mute_button(sync_handle, operator_index, style),
            mix: knob::operator_mix(sync_handle, operator_index, style),
            panning: knob::operator_panning(sync_handle, operator_index, style),
            wave_type: WavePicker::new(sync_handle, wave_type_parameter, style, "WAVE"),
            mod_index,
            mod_target,
            feedback: knob::operator_feedback(sync_handle, operator_index, style),
            frequency_ratio: knob::operator_frequency_ratio(sync_handle, operator_index, style),
            frequency_free: knob::operator_frequency_free(sync_handle, operator_index, style),
            frequency_fine: knob::operator_frequency_fine(sync_handle, operator_index, style),
            envelope: Envelope::new(sync_handle, operator_index, style),
            wave_display: WaveDisplay::new(sync_handle, operator_index, style),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.mute_button.set_style(style);
        self.volume.set_style(style);
        self.mix.set_style(style);
        self.panning.set_style(style);
        self.wave_type.set_style(style);
        if let Some(mod_index) = self.mod_index.as_mut() {
            mod_index.set_style(style);
        }
        match self.mod_target.as_mut() {
            Some(ModTargetPicker::Operator2(p)) => {
                p.style = style;
            }
            Some(ModTargetPicker::Operator3(p)) => {
                p.style = style;
            }
            Some(ModTargetPicker::Operator4(p)) => {
                p.style = style;
            }
            None => {}
        }
        self.feedback.set_style(style);
        self.frequency_ratio.set_style(style);
        self.frequency_free.set_style(style);
        self.frequency_fine.set_style(style);
        self.envelope.set_style(style);
        self.wave_display.set_style(style);
    }

    pub fn view(&self) -> Element<Message, Theme> {
        let heading = {
            let mute_button = Tooltip::new(self.mute_button.view(), "Toggle mute", Position::Top)
                .style(self.style.tooltip())
                .font(self.style.font_regular())
                .padding(self.style.tooltip_padding());

            Container::new(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(0)
                    .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                    .push(
                        Row::new()
                            .width(Length::Fill)
                            .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                            .push(mute_button),
                    )
                    .push(
                        Text::new(format!("OP {}", self.index + 1))
                            .size(FONT_SIZE + FONT_SIZE / 2)
                            .height(Length::Units(FONT_SIZE + FONT_SIZE / 2))
                            .font(self.style.font_heading())
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT / 2)))
                    .push(self.wave_display.view()),
            )
            .width(Length::Units(LINE_HEIGHT * 8))
            .height(Length::Units(LINE_HEIGHT * 7))
        };

        let group_1 = container_l2(
            Row::new()
                .push(container_l3(self.wave_type.view()))
                .push(space_l3())
                .push(container_l3(self.volume.view()))
                .push(space_l3())
                .push(container_l3(self.panning.view())),
        );

        let routing_group = {
            let mut group = Row::new()
                .push(container_l3(self.mix.view()))
                .push(space_l3());

            if let Some(mod_index) = self.mod_index.as_ref() {
                group = group.push(container_l3(mod_index.view()));
            } else {
                group = group.push(Space::with_width(Length::Units(LINE_HEIGHT * 5)));
            }

            group = group.push(space_l3());

            match self.mod_target.as_ref() {
                Some(ModTargetPicker::Operator2(picker)) => {
                    group = group.push(container_l3(picker.view()))
                }
                Some(ModTargetPicker::Operator3(picker)) => {
                    group = group.push(container_l3(picker.view()))
                }
                Some(ModTargetPicker::Operator4(picker)) => {
                    group = group.push(container_l3(picker.view()))
                }
                None => group = group.push(Space::with_width(Length::Units(LINE_HEIGHT * 3))),
            }

            group = group.push(space_l3());
            group = group.push(container_l3(self.feedback.view()));

            container_l2(group)
        };

        let frequency_group = container_l2(
            Row::new()
                .push(container_l3(self.frequency_ratio.view()))
                .push(space_l3())
                .push(container_l3(self.frequency_free.view()))
                .push(space_l3())
                .push(container_l3(self.frequency_fine.view())),
        );

        let envelope = container_l2(self.envelope.view()).height(Length::Units(LINE_HEIGHT * 8));

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
