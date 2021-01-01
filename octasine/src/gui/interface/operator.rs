use iced_baseview::{
    Container, Element, Text, Length, Align, Row, Rule, Space, HorizontalAlignment, Column
};


use crate::GuiSyncHandle;

use super::{FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT, Message};
use super::envelope::Envelope;
use super::knob::OctaSineKnob;
use super::wave_picker::WaveTypePicker;


pub struct OperatorWidgets {
    index: usize,
    pub volume: OctaSineKnob,
    pub panning: OctaSineKnob,
    pub wave_type: WaveTypePicker,
    pub mod_index: OctaSineKnob,
    pub feedback: OctaSineKnob,
    pub frequency_ratio: OctaSineKnob,
    pub frequency_free: OctaSineKnob,
    pub frequency_fine: OctaSineKnob,
    pub additive: Option<OctaSineKnob>,
    pub envelope: Envelope,
}


impl OperatorWidgets {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        operator_index: usize,
    ) -> Self {
        let (volume, panning, wave, additive, mod_index, feedback, ratio, free, fine) = match operator_index {
            0 => ( 2,  3,  4,  0,  5,  6,  7,  8,  9),
            1 => (15, 16, 17, 18, 19, 20, 21, 22, 23),
            2 => (29, 30, 31, 32, 34, 35, 36, 37, 38),
            3 => (44, 45, 46, 47, 49, 50, 51, 52, 53),
            _ => unreachable!(),
        };

        let additive_knob = if operator_index == 0 {
            None
        } else {
            Some(OctaSineKnob::operator_additive(sync_handle, additive))
        };

        Self {
            index: operator_index,
            volume: OctaSineKnob::operator_volume(sync_handle, volume),
            panning: OctaSineKnob::operator_panning(sync_handle, panning),
            wave_type: WaveTypePicker::new(sync_handle, wave),
            mod_index: OctaSineKnob::operator_mod_index(sync_handle, mod_index),
            feedback: OctaSineKnob::operator_feedback(sync_handle, feedback),
            frequency_ratio: OctaSineKnob::operator_frequency_ratio(sync_handle, ratio),
            frequency_free: OctaSineKnob::operator_frequency_free(sync_handle, free),
            frequency_fine: OctaSineKnob::operator_frequency_fine(sync_handle, fine),
            additive: additive_knob,
            envelope: Envelope::new(sync_handle, operator_index),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let operator_number = Text::new(format!("{}", self.index + 1))
            .size(FONT_SIZE * 2)
            .font(FONT_VERY_BOLD)
            .horizontal_alignment(HorizontalAlignment::Center);

        let mut row = Row::new()
            .push(
                Container::new(operator_number)
                    .width(Length::Units(LINE_HEIGHT * 4))
                    .height(Length::Units(LINE_HEIGHT * 6))
                    .align_x(Align::Center)
                    .align_y(Align::Center)
            )
            // .push(Space::with_width(Length::Units(LINE_HEIGHT)))
            .push(self.wave_type.view())
            .push(self.volume.view())
            .push(self.panning.view());
        
        if let Some(additive) = self.additive.as_mut() {
            row = row.push(additive.view())
        } else {
            row = row.push(Space::with_width(Length::Units(LINE_HEIGHT * 4)))
        }

        row = row
            .push(
                Container::new(
                    Rule::vertical(LINE_HEIGHT)
                )
                    .height(Length::Units(LINE_HEIGHT * 6))
            )
            .push(self.mod_index.view())
            .push(self.feedback.view());
        
        row = row
            .push(
                Container::new(
                    Rule::vertical(LINE_HEIGHT)
                )
                    .height(Length::Units(LINE_HEIGHT * 6)))
            .push(self.frequency_ratio.view())
            .push(self.frequency_free.view())
            .push(self.frequency_fine.view());
        
        row = row
            .push(
                Container::new(
                    Rule::vertical(LINE_HEIGHT)
                )
                    .height(Length::Units(LINE_HEIGHT * 6))
            )
            .push(
                Column::new()
                    // .push(
                    //     Space::with_height(Length::Units(14))
                    // )
                    .push(
                        Container::new(self.envelope.view())
                            .width(Length::Fill)
                    )
            );

        row.into()
    }
}
