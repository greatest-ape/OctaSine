use iced_baseview::tooltip::Position;
use iced_baseview::{
    alignment::Horizontal, alignment::Vertical, Column, Element, Length, Row, Space, Text,
};
use iced_baseview::{Container, Tooltip};

use crate::parameters::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoParameter, LfoShapeValue,
    Parameter,
};
use crate::sync::GuiSyncHandle;

use super::boolean_button::{
    lfo_active_button, lfo_bpm_sync_button, lfo_mode_button, BooleanButton,
};
use super::common::{container_l1, container_l2, container_l3, space_l3};
use super::knob::{self, OctaSineKnob};
use super::lfo_target_picker::LfoTargetPicker;
use super::style::Theme;
use super::wave_picker::WavePicker;
use super::{Message, FONT_SIZE, LINE_HEIGHT};

pub struct LfoWidgets {
    index: usize,
    style: Theme,
    pub target: LfoTargetPicker,
    pub shape: WavePicker<LfoShapeValue>,
    pub mode: BooleanButton,
    pub bpm_sync: BooleanButton,
    pub frequency_ratio: OctaSineKnob<LfoFrequencyRatioValue>,
    pub frequency_free: OctaSineKnob<LfoFrequencyFreeValue>,
    pub amount: OctaSineKnob<LfoAmountValue>,
    pub active: BooleanButton,
}

impl LfoWidgets {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize, style: Theme) -> Self {
        let lfo_wave_type_parameter = Parameter::Lfo(lfo_index as u8, LfoParameter::Shape);

        Self {
            index: lfo_index,
            style,
            target: LfoTargetPicker::new(sync_handle, lfo_index, style),
            shape: WavePicker::new(sync_handle, lfo_wave_type_parameter, style, "SHAPE"),
            mode: lfo_mode_button(sync_handle, lfo_index, style),
            bpm_sync: lfo_bpm_sync_button(sync_handle, lfo_index, style),
            frequency_ratio: knob::lfo_frequency_ratio(sync_handle, lfo_index, style),
            frequency_free: knob::lfo_frequency_free(sync_handle, lfo_index, style),
            amount: knob::lfo_amount(sync_handle, lfo_index, style),
            active: lfo_active_button(sync_handle, lfo_index, style),
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.target.style = style;
        self.shape.set_style(style);
        self.mode.set_style(style);
        self.bpm_sync.set_style(style);
        self.frequency_ratio.set_style(style);
        self.frequency_free.set_style(style);
        self.amount.set_style(style);
        self.active.set_style(style);
    }

    pub fn view(&self) -> Element<Message> {
        let title = Text::new(format!("LFO {}", self.index + 1))
            .size(FONT_SIZE + FONT_SIZE / 2)
            .height(Length::Units(FONT_SIZE + FONT_SIZE / 2))
            .font(self.style.font_heading())
            .width(Length::Units(LINE_HEIGHT * 9))
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center);

        let bpm_sync = Tooltip::new(
            self.bpm_sync.view(),
            "Toggle BPM sync. When turned off, base frequency is 1 Hz",
            Position::Top,
        )
        .style(self.style.tooltip())
        .font(self.style.font_regular())
        .padding(self.style.tooltip_padding());

        let mode = Tooltip::new(self.mode.view(), "Toggle oneshot mode", Position::Top)
            .style(self.style.tooltip())
            .font(self.style.font_regular())
            .padding(self.style.tooltip_padding());
        let active = Tooltip::new(self.active.view(), "Toggle mute", Position::Top)
            .style(self.style.tooltip())
            .font(self.style.font_regular())
            .padding(self.style.tooltip_padding());

        container_l1(
            self.style,
            Row::new()
                .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                .push(
                    Container::new(
                        Column::new()
                            .push(Space::with_height(Length::Units(LINE_HEIGHT * 1)))
                            .push(
                                Row::new()
                                    .push(active)
                                    .push(Space::with_width(Length::Units(LINE_HEIGHT * 6 - 3 - 1)))
                                    .push(bpm_sync)
                                    .push(Space::with_width(Length::Units(3)))
                                    .push(mode),
                            )
                            .push(title)
                            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                            .push(Row::new().push(self.target.view())),
                    )
                    .width(Length::Units(LINE_HEIGHT * 9)),
                )
                .push(Space::with_width(Length::Units(LINE_HEIGHT)))
                .push(container_l2(
                    self.style,
                    Row::new()
                        .push(container_l3(self.style, self.shape.view()))
                        .push(space_l3())
                        .push(container_l3(self.style, self.amount.view()))
                        .push(space_l3())
                        .push(container_l3(self.style, self.frequency_ratio.view()))
                        .push(space_l3())
                        .push(container_l3(self.style, self.frequency_free.view())),
                )),
        )
        .into()
    }
}
