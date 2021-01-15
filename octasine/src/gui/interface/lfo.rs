use iced_baseview::{
    Container, Element, Text, Length, Align, Row, Rule, Space, HorizontalAlignment, Column
};


use crate::GuiSyncHandle;

use super::Message;
use super::knob::OctaSineKnob;
use super::boolean_picker::{self, BooleanPicker};


pub struct LfoWidgets {
    index: usize,
    pub target: OctaSineKnob,
    pub shape: OctaSineKnob,
    pub mode: OctaSineKnob,
    pub bpm_sync: BooleanPicker<bool>,
    pub frequency_ratio: OctaSineKnob,
    pub frequency_free: OctaSineKnob,
    pub magnitude: OctaSineKnob,
}


impl LfoWidgets {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
    ) -> Self {
        let offset = 59 + lfo_index * 7;
        let target = offset + 0;
        let shape = offset + 1;
        let mode = offset + 2;
        let bpm_sync = offset + 3;
        let ratio = offset + 4;
        let free = offset + 5;
        let magnitude = offset + 6;

        Self {
            index: lfo_index,
            target: OctaSineKnob::lfo_other(sync_handle, lfo_index, target, "TARGET"),
            shape: OctaSineKnob::lfo_other(sync_handle, lfo_index, shape, "SHAPE"),
            mode: OctaSineKnob::lfo_other(sync_handle, lfo_index, mode, "MODE"),
            bpm_sync: boolean_picker::bpm_sync(sync_handle, lfo_index, bpm_sync),
            frequency_ratio: OctaSineKnob::lfo_frequency_ratio(sync_handle, lfo_index, ratio),
            frequency_free: OctaSineKnob::lfo_frequency_free(sync_handle, lfo_index, free),
            magnitude: OctaSineKnob::lfo_magnitude(sync_handle, lfo_index, magnitude),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(self.target.view())
            .push(self.shape.view())
            .push(self.mode.view())
            .push(self.bpm_sync.view())
            .push(self.frequency_ratio.view())
            .push(self.frequency_free.view())
            .push(self.magnitude.view())
            .into()
    }
}
