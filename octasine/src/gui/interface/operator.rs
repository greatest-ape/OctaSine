use iced_baseview::{
    button, Alignment, Button, Column, Container, Element, Horizontal, Length, Row, Rule, Space,
    Text, Vertical,
};

use crate::parameters::values::{
    OperatorFeedbackValue, OperatorFrequencyFineValue,
    OperatorFrequencyFreeValue, OperatorFrequencyRatioValue, OperatorModulationIndexValue,
    OperatorPanningValue, OperatorMixValue, OperatorWaveTypeValue, Operator4ModulationTargetValue, Operator3ModulationTargetValue,
};
use crate::GuiSyncHandle;

use super::boolean_picker::{self, BooleanPicker};
use super::envelope::Envelope;
use super::knob::{self, OctaSineKnob};
use super::style::Theme;
use super::{Message, FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT};

pub enum ModTargetPicker {
    Operator4(BooleanPicker<Operator4ModulationTargetValue>),
    Operator3(BooleanPicker<Operator3ModulationTargetValue>),
}

pub struct OperatorWidgets {
    index: usize,
    style: Theme,
    pub mix: OctaSineKnob<OperatorMixValue>,
    pub panning: OctaSineKnob<OperatorPanningValue>,
    pub wave_type: BooleanPicker<OperatorWaveTypeValue>,
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
        let (mix, panning, wave, mod_target, mod_index, feedback, ratio, free, fine) =
            match operator_index {
                0 => (2, 3, 4, 0, 0, 5, 6, 7, 8),
                1 => (14, 15, 16, 17, 18, 19, 20, 21, 22),
                2 => (28, 29, 30, 31, 32, 33, 34, 35, 36),
                3 => (42, 43, 44, 45, 46, 47, 48, 49, 50),
                _ => unreachable!(),
            };
        
        let mod_index = if operator_index != 0 {
            Some(knob::operator_mod_index(sync_handle, mod_index, style))
        } else {
            None
        };

        let mod_target = match operator_index {
            3 => Some(ModTargetPicker::Operator4(boolean_picker::operator_4_target(sync_handle, mod_target, style))),
            2 => Some(ModTargetPicker::Operator3(boolean_picker::operator_3_target(sync_handle, mod_target, style))),
            _ => None,
        };

        Self {
            index: operator_index,
            style,
            mix: knob::operator_mix(sync_handle, mix, operator_index, style),
            panning: knob::operator_panning(sync_handle, panning, style),
            wave_type: boolean_picker::wave_type(sync_handle, wave, style),
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
        self.mix.style = style;
        self.panning.style = style;
        self.wave_type.style = style;
        if let Some(mod_index) = self.mod_index.as_mut() {
            mod_index.style = style;
        }
        self.feedback.style = style;
        self.frequency_ratio.style = style;
        self.frequency_free.style = style;
        self.frequency_fine.style = style;
        self.envelope.set_style(style);
    }

    pub fn view(&mut self) -> Element<Message> {
        let operator_number = Text::new(format!("{}", self.index + 1))
            .size(FONT_SIZE * 2)
            .font(FONT_VERY_BOLD)
            .color(self.style.heading_color())
            .horizontal_alignment(Horizontal::Center);

        let mut row = Row::new()
            .push(
                Container::new(operator_number)
                    .width(Length::Units(LINE_HEIGHT * 4))
                    .height(Length::Units(LINE_HEIGHT * 6))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            // .push(Space::with_width(Length::Units(LINE_HEIGHT)))
            .push(self.wave_type.view())
            .push(self.panning.view());

        row = row
            .push(
                Container::new(Rule::vertical(LINE_HEIGHT)).height(Length::Units(LINE_HEIGHT * 6)),
            )
            .push(self.mix.view());

        if let Some(mod_index) = self.mod_index.as_mut() {
            row = row.push(mod_index.view())
        } else {
            row = row.push(Space::with_width(Length::Units(LINE_HEIGHT * 4)))
        }

        match self.mod_target.as_mut() {
            Some(ModTargetPicker::Operator3(picker)) => {
                row = row.push(picker.view())
            },
            Some(ModTargetPicker::Operator4(picker)) => {
                row = row.push(picker.view())
            },
            None => {
                row = row.push(Space::with_width(Length::Units(LINE_HEIGHT * 4)))
            }
        }

        row = row
            .push(self.feedback.view());

        row = row
            .push(
                Container::new(Rule::vertical(LINE_HEIGHT)).height(Length::Units(LINE_HEIGHT * 6)),
            )
            .push(self.frequency_ratio.view())
            .push(self.frequency_free.view())
            .push(self.frequency_fine.view());

        let sync_viewports_message = Message::EnvelopeSyncViewports {
            viewport_factor: self.envelope.get_viewport_factor(),
            x_offset: self.envelope.get_x_offset(),
        };
        let zoom_to_fit_message = Message::EnvelopeZoomToFit(self.index);

        row = row
            .push(
                Container::new(Rule::vertical(LINE_HEIGHT)).height(Length::Units(LINE_HEIGHT * 6)),
            )
            .push(Column::new().push(self.envelope.view()))
            .push(
                Column::new()
                    .width(Length::Units(LINE_HEIGHT * 3))
                    .align_items(Alignment::End)
                    .push(
                        Row::new()
                            .push(
                                Button::new(
                                    &mut self.zoom_out,
                                    Text::new("−").font(FONT_VERY_BOLD),
                                )
                                .on_press(Message::EnvelopeZoomOut(self.index))
                                .style(self.style),
                            )
                            .push(Space::with_width(Length::Units(3)))
                            .push(
                                Button::new(&mut self.zoom_in, Text::new("+").font(FONT_VERY_BOLD))
                                    .on_press(Message::EnvelopeZoomIn(self.index))
                                    .style(self.style),
                            ),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT * 1 - 10)))
                    .push(
                        Row::new().push(
                            Button::new(&mut self.zoom_to_fit, Text::new("FIT"))
                                .on_press(zoom_to_fit_message)
                                .style(self.style),
                        ),
                    )
                    .push(Space::with_height(Length::Units(LINE_HEIGHT * 1 - 10)))
                    .push(
                        Row::new().push(
                            Button::new(&mut self.sync_viewport, Text::new("DIST"))
                                .on_press(sync_viewports_message)
                                .style(self.style),
                        ),
                    ),
            );

        row.into()
    }
}
