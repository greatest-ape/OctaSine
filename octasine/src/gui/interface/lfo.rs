use iced_baseview::{
    alignment::Horizontal, alignment::Vertical, Column, Element, Length, Row, Space, Text,
};

use crate::parameter_values::{
    LfoAmountValue, LfoBpmSyncValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoModeValue,
    LfoShapeValue,
};
use crate::sync::GuiSyncHandle;

use super::boolean_picker::{self, BooleanPicker};
use super::common::{container_l1, container_l2};
use super::knob::{self, OctaSineKnob};
use super::lfo_target_picker::LfoTargetPicker;
use super::style::Theme;
use super::wave_picker::WavePicker;
use super::{Message, FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT};

pub struct LfoWidgets {
    index: usize,
    style: Theme,
    pub target: LfoTargetPicker,
    pub shape: WavePicker<LfoShapeValue>,
    pub mode: BooleanPicker<LfoModeValue>,
    pub bpm_sync: BooleanPicker<LfoBpmSyncValue>,
    pub frequency_ratio: OctaSineKnob<LfoFrequencyRatioValue>,
    pub frequency_free: OctaSineKnob<LfoFrequencyFreeValue>,
    pub amount: OctaSineKnob<LfoAmountValue>,
}

impl LfoWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize, style: Theme) -> Self {
        let offset = 64 + lfo_index * 7;
        let target = offset + 0;
        let bpm_sync = offset + 1;
        let ratio = offset + 2;
        let free = offset + 3;
        let mode = offset + 4;
        let shape = offset + 5;
        let amount = offset + 6;

        Self {
            index: lfo_index,
            style,
            target: LfoTargetPicker::new(sync_handle, lfo_index, target, style),
            shape: WavePicker::new(sync_handle, shape, style, "SHAPE"),
            mode: boolean_picker::lfo_mode(sync_handle, mode, style),
            bpm_sync: boolean_picker::bpm_sync(sync_handle, bpm_sync, style),
            frequency_ratio: knob::lfo_frequency_ratio(sync_handle, ratio, style),
            frequency_free: knob::lfo_frequency_free(sync_handle, free, style),
            amount: knob::lfo_amount(sync_handle, amount, style),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.target.style = style;
        self.shape.set_style(style);
        self.mode.style = style;
        self.bpm_sync.style = style;
        self.frequency_ratio.style = style;
        self.frequency_free.style = style;
        self.amount.style = style;
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(format!("LFO {}", self.index + 1))
            .size((FONT_SIZE * 3) / 2)
            .height(Length::Units(LINE_HEIGHT * 2))
            .width(Length::Units(LINE_HEIGHT * 9))
            .font(FONT_VERY_BOLD)
            .color(self.style.heading_color())
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center);

        let c = Row::new()
            .push(Space::with_width(Length::Units(LINE_HEIGHT)))
            .push(
                Column::new()
                    .push(Space::with_height(Length::Units(LINE_HEIGHT * 2)))
                    .push(Row::new().push(title))
                    .push(Space::with_height(Length::Units(LINE_HEIGHT * 0)))
                    .push(self.target.view())
            )
            .push(Space::with_width(Length::Units(LINE_HEIGHT * 1)))
            .push(container_l2(
                Row::new()
                    // .push(self.mode.view())
                    // .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    // .push(self.bpm_sync.view())
                    // .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(self.shape.view())
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(self.amount.view())
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(self.frequency_ratio.view())
                    .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                    .push(self.frequency_free.view()),
            ));

        container_l1(c, 0).into()
    }
}
