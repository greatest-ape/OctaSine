use iced_baseview::{
    Column, Element, HorizontalAlignment, Length, Row, Space, Text, VerticalAlignment,
};

use crate::parameters::values::{
    LfoAmountValue, LfoBpmSyncValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoModeValue,
    LfoShapeValue,
};
use crate::GuiSyncHandle;

use super::boolean_picker::{self, BooleanPicker};
use super::knob::{self, OctaSineKnob};
use super::lfo_target_picker::LfoTargetPicker;
use super::style::Theme;
use super::{Message, FONT_SIZE, FONT_VERY_BOLD, LINE_HEIGHT};

pub struct LfoWidgets {
    index: usize,
    style: Theme,
    pub target: LfoTargetPicker,
    pub shape: OctaSineKnob<LfoShapeValue>,
    pub mode: BooleanPicker<LfoModeValue>,
    pub bpm_sync: BooleanPicker<LfoBpmSyncValue>,
    pub frequency_ratio: OctaSineKnob<LfoFrequencyRatioValue>,
    pub frequency_free: OctaSineKnob<LfoFrequencyFreeValue>,
    pub amount: OctaSineKnob<LfoAmountValue>,
}

impl LfoWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize, style: Theme) -> Self {
        let offset = 59 + lfo_index * 7;
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
            shape: knob::lfo_shape(sync_handle, shape, style),
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
        self.shape.style = style;
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
            .width(Length::Units(LINE_HEIGHT * 12))
            .font(FONT_VERY_BOLD)
            .color(self.style.heading_color())
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center);

        Column::new()
            .push(Row::new().push(title))
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(Row::new().push(self.target.view()))
            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
            .push(
                Row::new()
                    .push(self.bpm_sync.view())
                    .push(self.frequency_ratio.view())
                    .push(self.frequency_free.view()),
            )
            .push(Space::with_height(Length::Units(
                LINE_HEIGHT * 1 + LINE_HEIGHT / 1,
            )))
            .push(
                Row::new()
                    .push(self.mode.view())
                    .push(self.shape.view())
                    .push(self.amount.view()),
            )
            .into()
    }
}
