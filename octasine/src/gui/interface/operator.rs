use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text, Length, HorizontalAlignment, Align, Row, Rule
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
        Self {
            index: operator_index,
            volume: OctaSineKnob::operator_volume(sync_handle, 2),
            panning: OctaSineKnob::operator_panning(sync_handle, 3),
            frequency_ratio: OctaSineKnob::operator_frequency_ratio(sync_handle, 7),
        }
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        let title = format!("Operator {}", self.index + 1);

        Row::new()
            .push(
                Column::new()
                    .spacing(16)
                    .push(
                        Row::new()
                            .push(Text::new(title).size(16))
                    )
                    .push(
                        Row::new()
                            .push(self.volume.view(sync_handle))
                            .push(self.panning.view(sync_handle))
                            .push(self.frequency_ratio.view(sync_handle))
                    )
            )
            .into()
    }
}