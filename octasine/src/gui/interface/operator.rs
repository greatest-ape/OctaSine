use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text, Length, HorizontalAlignment, Align, Row, Rule, Space
};

use crate::SyncHandle;

use super::{Message, ParameterWidget};
use super::widgets::OctaSineKnob;


pub struct OperatorWidgets {
    index: usize,
    volume: OctaSineKnob,
    panning: OctaSineKnob,
    frequency_ratio: OctaSineKnob,
}


impl OperatorWidgets {
    pub fn new<H: SyncHandle>(
        sync_handle: &Arc<H>,
        operator_index: usize,
    ) -> Self {
        let (volume, panning, ratio) = match operator_index {
            0 => (2, 3, 7),
            1 => (15, 16, 21),
            2 => (29, 30, 36),
            3 => (44, 45, 52),
            _ => unreachable!(),
        };

        Self {
            index: operator_index,
            volume: OctaSineKnob::operator_volume(sync_handle, volume),
            panning: OctaSineKnob::operator_panning(sync_handle, panning),
            frequency_ratio: OctaSineKnob::operator_frequency_ratio(sync_handle, ratio),
        }
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        let title = format!("{}", self.index + 1);

        Row::new()
            .align_items(Align::Center)
            .push(Text::new(title).size(32))
            .push(Space::with_width(Length::Units(16)))
            .push(self.volume.view(sync_handle))
            .push(self.panning.view(sync_handle))
            .push(self.frequency_ratio.view(sync_handle))
            .into()
    }
}