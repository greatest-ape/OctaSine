use iced_baseview::{
    Container, Element, Text, Length, Align, Row, Space, HorizontalAlignment, Column
};


use crate::GuiSyncHandle;

use super::{FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT, Message};
use super::knob::OctaSineKnob;
use super::lfo_target_picker::LfoTargetPicker;
use super::boolean_picker::{self, BooleanPicker};


pub struct LfoWidgets {
    index: usize,
    pub target: LfoTargetPicker,
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
            target: LfoTargetPicker::new(sync_handle, lfo_index, target),
            shape: OctaSineKnob::lfo_shape(sync_handle, shape),
            mode: OctaSineKnob::lfo_mode(sync_handle, mode),
            bpm_sync: boolean_picker::bpm_sync(sync_handle, bpm_sync),
            frequency_ratio: OctaSineKnob::lfo_frequency_ratio(sync_handle, ratio),
            frequency_free: OctaSineKnob::lfo_frequency_free(sync_handle, free),
            magnitude: OctaSineKnob::lfo_magnitude(sync_handle, magnitude),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let operator_number = Text::new(format!("LFO{}", self.index + 1))
            .size((FONT_SIZE * 3) / 2)
            .font(FONT_VERY_BOLD)
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new()
            .push(
                Row::new()
                    .push(
                        Container::new(operator_number)
                            .width(Length::Units(LINE_HEIGHT * 12))
                            .height(Length::Units(LINE_HEIGHT * 2))
                            .align_x(Align::Center)
                            .align_y(Align::Center)
                    )
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(self.target.view())
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(self.mode.view())
                    .push(self.shape.view())
                    .push(self.bpm_sync.view())
            )
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(self.frequency_ratio.view())
                    .push(self.frequency_free.view())
                    .push(self.magnitude.view())
            )
            .into()
    }
}
