use iced_baseview::tooltip::Position;
use iced_baseview::Tooltip;
use iced_baseview::{
    alignment::Horizontal, button, Alignment, Button, Column, Container, Element, Length, Row,
    Space, Text,
};

use crate::parameter_values::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
    OperatorFeedbackValue, OperatorFrequencyFineValue, OperatorFrequencyFreeValue,
    OperatorFrequencyRatioValue, OperatorMixValue, OperatorModulationIndexValue,
    OperatorPanningValue, OperatorVolumeValue, OperatorWaveTypeValue,
};
use crate::sync::GuiSyncHandle;

use super::boolean_button::{operator_mute_button, BooleanButton};
use super::common::{container_l1, container_l2, container_l3, space_l2, space_l3};
use super::envelope::Envelope;
use super::knob::{self, OctaSineKnob};
use super::mod_target_picker;
use super::style::Theme;
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
    pub mix: OctaSineKnob<OperatorMixValue>,
    pub panning: OctaSineKnob<OperatorPanningValue>,
    pub wave_type: WavePicker<OperatorWaveTypeValue>,
    pub mod_index: Option<OctaSineKnob<OperatorModulationIndexValue>>,
    pub mod_target: Option<ModTargetPicker>,
    pub feedback: OctaSineKnob<OperatorFeedbackValue>,
    pub frequency_ratio: OctaSineKnob<OperatorFrequencyRatioValue>,
    pub frequency_free: OctaSineKnob<OperatorFrequencyFreeValue>,
    pub frequency_fine: OctaSineKnob<OperatorFrequencyFineValue>,
    pub envelope: Envelope,
    pub zoom_in: button::State,
    pub zoom_out: button::State,
    pub sync_viewport: button::State,
    pub zoom_to_fit: button::State,
}

impl OperatorWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let (
            volume,
            volume_toggle,
            mix,
            panning,
            wave,
            mod_target,
            mod_index,
            feedback,
            ratio,
            free,
            fine,
        ) = match operator_index {
            0 => (2, 3, 4, 5, 6, 0, 0, 7, 8, 9, 10),
            1 => (16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26),
            2 => (32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42),
            3 => (48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58),
            _ => unreachable!(),
        };

        let mod_index = if operator_index != 0 {
            Some(knob::operator_mod_index(sync_handle, mod_index, style))
        } else {
            None
        };

        let mute_button = operator_mute_button(sync_handle, volume_toggle, style);

        let mod_target = match operator_index {
            3 => Some(ModTargetPicker::Operator4(
                mod_target_picker::operator_4_target(sync_handle, mod_target, style),
            )),
            2 => Some(ModTargetPicker::Operator3(
                mod_target_picker::operator_3_target(sync_handle, mod_target, style),
            )),
            1 => Some(ModTargetPicker::Operator2(
                mod_target_picker::operator_2_target(sync_handle, mod_target, style),
            )),
            _ => None,
        };

        Self {
            index: operator_index,
            style,
            volume: knob::operator_volume(sync_handle, volume, style),
            mute_button,
            mix: knob::operator_mix(sync_handle, mix, operator_index, style),
            panning: knob::operator_panning(sync_handle, panning, style),
            wave_type: WavePicker::new(sync_handle, wave, style, "WAVE"),
            mod_index,
            mod_target,
            feedback: knob::operator_feedback(sync_handle, feedback, style),
            frequency_ratio: knob::operator_frequency_ratio(sync_handle, ratio, style),
            frequency_free: knob::operator_frequency_free(sync_handle, free, style),
            frequency_fine: knob::operator_frequency_fine(sync_handle, fine, style),
            envelope: Envelope::new(sync_handle, operator_index, style),
            zoom_in: button::State::default(),
            zoom_out: button::State::default(),
            sync_viewport: button::State::default(),
            zoom_to_fit: button::State::default(),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.mute_button.set_style(style);
        self.volume.style = style;
        self.mix.style = style;
        self.panning.style = style;
        self.wave_type.set_style(style);
        if let Some(mod_index) = self.mod_index.as_mut() {
            mod_index.style = style;
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
        self.feedback.style = style;
        self.frequency_ratio.style = style;
        self.frequency_free.style = style;
        self.frequency_fine.style = style;
        self.envelope.set_style(style);
    }

    pub fn view(&mut self) -> Element<Message> {
        let heading = Container::new(
            Column::new()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(0)
                .push(Space::with_height(Length::Units(LINE_HEIGHT * 3)))
                .push(
                    Text::new(format!("OP {}", self.index + 1))
                        .size(FONT_SIZE + FONT_SIZE / 2)
                        .font(self.style.font_heading())
                        .color(self.style.heading_color())
                        .horizontal_alignment(Horizontal::Center),
                )
                .push(self.mute_button.view()),
        )
        .width(Length::Units(LINE_HEIGHT * 8))
        .height(Length::Units(LINE_HEIGHT * 7));

        let group_1 = container_l2(
            self.style,
            Row::new()
                .push(container_l3(self.style, self.wave_type.view()))
                .push(space_l3())
                .push(container_l3(self.style, self.volume.view()))
                .push(space_l3())
                .push(container_l3(self.style, self.panning.view())),
        );

        let routing_group = {
            let mut group = Row::new()
                .push(container_l3(self.style, self.mix.view()))
                .push(space_l3());

            if let Some(mod_index) = self.mod_index.as_mut() {
                group = group.push(container_l3(self.style, mod_index.view()));
            } else {
                group = group.push(Space::with_width(Length::Units(LINE_HEIGHT * 5)));
            }

            group = group.push(space_l3());

            match self.mod_target.as_mut() {
                Some(ModTargetPicker::Operator2(picker)) => {
                    group = group.push(container_l3(self.style, picker.view()))
                }
                Some(ModTargetPicker::Operator3(picker)) => {
                    group = group.push(container_l3(self.style, picker.view()))
                }
                Some(ModTargetPicker::Operator4(picker)) => {
                    group = group.push(container_l3(self.style, picker.view()))
                }
                None => group = group.push(Space::with_width(Length::Units(LINE_HEIGHT * 3))),
            }

            group = group.push(space_l3());
            group = group.push(container_l3(self.style, self.feedback.view()));

            container_l2(self.style, group)
        };

        let frequency_group = container_l2(
            self.style,
            Row::new()
                .push(container_l3(self.style, self.frequency_ratio.view()))
                .push(space_l3())
                .push(container_l3(self.style, self.frequency_free.view()))
                .push(space_l3())
                .push(container_l3(self.style, self.frequency_fine.view())),
        );

        let sync_viewports_message = Message::EnvelopeSyncViewports {
            viewport_factor: self.envelope.get_viewport_factor(),
            x_offset: self.envelope.get_x_offset(),
        };

        let envelope = container_l2(
            self.style,
            Row::new()
                .push(container_l3(self.style, self.envelope.view()))
                .push(container_l3(
                    self.style,
                    Column::new()
                        .width(Length::Units(LINE_HEIGHT * 3))
                        .align_items(Alignment::End)
                        .push(
                            Row::new()
                                .push(
                                    Tooltip::new(
                                        Button::new(
                                            &mut self.zoom_out,
                                            Text::new("−").font(self.style.font_bold()),
                                        )
                                        .on_press(Message::EnvelopeZoomOut(self.index))
                                        .style(self.style.button()),
                                        "Zoom out",
                                        Position::Top,
                                    )
                                    .style(self.style.tooltip())
                                    .font(self.style.font_regular())
                                    .padding(self.style.tooltip_padding()),
                                )
                                .push(Space::with_width(Length::Units(3)))
                                .push(
                                    Tooltip::new(
                                        Button::new(
                                            &mut self.zoom_in,
                                            Text::new("+").font(self.style.font_bold()),
                                        )
                                        .on_press(Message::EnvelopeZoomIn(self.index))
                                        .style(self.style.button()),
                                        "Zoom in",
                                        Position::Top,
                                    )
                                    .style(self.style.tooltip())
                                    .font(self.style.font_regular())
                                    .padding(self.style.tooltip_padding()),
                                ),
                        )
                        .push(Space::with_height(Length::Units(2)))
                        .push(
                            Row::new().push(
                                Tooltip::new(
                                    Button::new(
                                        &mut self.zoom_to_fit,
                                        Text::new("FIT").font(self.style.font_regular()),
                                    )
                                    .on_press(Message::EnvelopeZoomToFit(self.index))
                                    .style(self.style.button()),
                                    "Zoom to fit",
                                    Position::Top,
                                )
                                .style(self.style.tooltip())
                                .font(self.style.font_regular())
                                .padding(self.style.tooltip_padding()),
                            ),
                        )
                        .push(Space::with_height(Length::Units(2)))
                        .push(
                            Row::new().push(
                                Tooltip::new(
                                    Button::new(
                                        &mut self.sync_viewport,
                                        Text::new("DIST").font(self.style.font_regular()),
                                    )
                                    .on_press(sync_viewports_message)
                                    .style(self.style.button()),
                                    "Distribute to other envelopes",
                                    Position::Top,
                                )
                                .style(self.style.tooltip())
                                .font(self.style.font_regular())
                                .padding(self.style.tooltip_padding()),
                            ),
                        ),
                )),
        )
        .height(Length::Units(LINE_HEIGHT * 8));

        container_l1(
            self.style,
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
