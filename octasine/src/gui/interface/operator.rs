use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text, Length, HorizontalAlignment, Align, Row, Rule, Space, Canvas
};


use crate::{OctaSine, SyncHandle, preset_parameters::OctaSinePresetParameters};

use super::{Message, ParameterWidget};
use super::knob::OctaSineKnob;
use super::envelope::Envelope;


pub struct OperatorWidgets {
    index: usize,
    pub volume: OctaSineKnob,
    pub panning: OctaSineKnob,
    pub mod_index: OctaSineKnob,
    pub feedback: OctaSineKnob,
    pub frequency_ratio: OctaSineKnob,
    pub frequency_free: OctaSineKnob,
    pub frequency_fine: OctaSineKnob,
    pub additive: Option<OctaSineKnob>,
    pub mod_out: Option<OctaSineKnob>,
    pub envelope: Envelope,
}


impl OperatorWidgets {
    pub fn new<H: SyncHandle>(
        sync_handle: &Arc<H>,
        operator_index: usize,
    ) -> Self {
        let (volume, panning, additive, mod_out, mod_index, feedback, ratio, free, fine) = match operator_index {
            0 => ( 2,  3,  0,   0, 5,  6,  7,  8,  9),
            1 => (15, 16, 18,  0, 19, 20, 21, 22, 23),
            2 => (29, 30, 32, 33, 34, 35, 36, 37, 38),
            3 => (44, 45, 47, 48, 49, 50, 51, 52, 53),
            _ => unreachable!(),
        };

        let additive_knob = if operator_index == 0 {
            None
        } else {
            Some(OctaSineKnob::operator_additive(sync_handle, additive))
        };

        let mod_out_knob = match operator_index {
            2 => Some(OctaSineKnob::operator_3_mod_output(sync_handle, mod_out)),
            3 => Some(OctaSineKnob::operator_4_mod_output(sync_handle, mod_out)),
            _ => None,
        };

        Self {
            index: operator_index,
            volume: OctaSineKnob::operator_volume(sync_handle, volume),
            panning: OctaSineKnob::operator_panning(sync_handle, panning),
            mod_index: OctaSineKnob::operator_mod_index(sync_handle, mod_index),
            feedback: OctaSineKnob::operator_feedback(sync_handle, feedback),
            frequency_ratio: OctaSineKnob::operator_frequency_ratio(sync_handle, ratio),
            frequency_free: OctaSineKnob::operator_frequency_free(sync_handle, free),
            frequency_fine: OctaSineKnob::operator_frequency_fine(sync_handle, fine),
            additive: additive_knob,
            mod_out: mod_out_knob,
            envelope: Envelope::new(&sync_handle, operator_index),
        }
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        let title = format!("{}", self.index + 1);

        let mut row = Row::new()
            .align_items(Align::Center)
            .push(Text::new(title).size(32))
            .push(Space::with_width(Length::Units(16)))
            .push(self.volume.view(sync_handle))
            .push(self.panning.view(sync_handle))
            .push(
                Container::new(
                    Rule::vertical(16)
                )
                    .height(Length::Units(64)))
            .push(self.mod_index.view(sync_handle))
            .push(self.feedback.view(sync_handle))
            .push(
                Container::new(
                    Rule::vertical(16)
                )
                    .height(Length::Units(64)))
            .push(self.frequency_ratio.view(sync_handle))
            .push(self.frequency_free.view(sync_handle))
            .push(self.frequency_fine.view(sync_handle))
            .push(
                Container::new(
                    Rule::vertical(16)
                )
                    .height(Length::Units(64))
            );
        
        if let Some(additive) = self.additive.as_mut() {
            row = row.push(additive.view(sync_handle))
        } else {
            row = row.push(Space::with_width(Length::Units(64)))
        }
        if let Some(mod_out) = self.mod_out.as_mut() {
            row = row.push(mod_out.view(sync_handle))
        } else {
            row = row.push(Space::with_width(Length::Units(64)))
        }
        
        row = row
            .push(
                Container::new(
                    Rule::vertical(16)
                )
                    .height(Length::Units(64)))
            .push(
                Container::new(self.envelope.view())
                    .width(Length::Fill)
                    // .padding(16)
            );

        row.into()
    }
}
