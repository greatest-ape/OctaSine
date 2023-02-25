use iced_baseview::widget::tooltip::Position;
use iced_baseview::widget::Container;
use iced_baseview::{
    alignment::Horizontal, alignment::Vertical, widget::Column, widget::Row, widget::Space,
    widget::Text, Element, Length,
};

use crate::parameters::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoParameter, LfoShapeValue,
    Parameter,
};
use crate::sync::GuiSyncHandle;

use super::boolean_button::{
    lfo_active_button, lfo_bpm_sync_button, lfo_mode_button, BooleanButton,
};
use super::common::{container_l1, container_l2, container_l3, space_l3, tooltip};
use super::knob::{self, OctaSineKnob};
use super::lfo_target_picker::LfoTargetPicker;
use super::style::Theme;
use super::wave_picker::WavePicker;
use super::{Message, FONT_SIZE, LINE_HEIGHT};

pub struct LfoWidgets {
    index: usize,
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
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> Self {
        let lfo_wave_type_parameter = Parameter::Lfo(lfo_index as u8, LfoParameter::Shape);

        Self {
            index: lfo_index,
            target: LfoTargetPicker::new(sync_handle, lfo_index),
            shape: WavePicker::new(sync_handle, lfo_wave_type_parameter, "SHAPE"),
            mode: lfo_mode_button(sync_handle, lfo_index),
            bpm_sync: lfo_bpm_sync_button(sync_handle, lfo_index),
            frequency_ratio: knob::lfo_frequency_ratio(sync_handle, lfo_index),
            frequency_free: knob::lfo_frequency_free(sync_handle, lfo_index),
            amount: knob::lfo_amount(sync_handle, lfo_index),
            active: lfo_active_button(sync_handle, lfo_index),
        }
    }

    pub fn theme_changed(&mut self) {
        self.mode.theme_changed();
        self.bpm_sync.theme_changed();
        self.active.theme_changed();
        self.shape.theme_changed();
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let title = Text::new(format!("LFO {}", self.index + 1))
            .size(FONT_SIZE + FONT_SIZE / 2)
            .height(Length::Fixed(f32::from(FONT_SIZE + FONT_SIZE / 2)))
            .font(theme.font_heading())
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 9)))
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center);

        let bpm_sync = tooltip(
            theme,
            "Toggle BPM sync. When turned off, base frequency is 1 Hz",
            Position::Top,
            self.bpm_sync.view(),
        );

        let mode = tooltip(
            theme,
            "Toggle oneshot mode",
            Position::Top,
            self.mode.view(),
        );
        let active = tooltip(theme, "Toggle mute", Position::Top, self.active.view());

        container_l1(
            Row::new()
                .push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT))))
                .push(
                    Container::new(
                        Column::new()
                            .push(Space::with_height(Length::Fixed(f32::from(
                                LINE_HEIGHT * 1,
                            ))))
                            .push(
                                Row::new()
                                    .push(active)
                                    .push(Space::with_width(Length::Fixed(f32::from(
                                        LINE_HEIGHT * 6 - 3 - 1,
                                    ))))
                                    .push(bpm_sync)
                                    .push(Space::with_width(Length::Fixed(3.0)))
                                    .push(mode),
                            )
                            .push(title)
                            .push(Space::with_height(Length::Fixed(f32::from(LINE_HEIGHT))))
                            .push(Row::new().push(self.target.view(theme))),
                    )
                    .width(Length::Fixed(f32::from(LINE_HEIGHT * 9))),
                )
                .push(Space::with_width(Length::Fixed(f32::from(LINE_HEIGHT))))
                .push(container_l2(
                    Row::new()
                        .push(container_l3(self.shape.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.amount.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.frequency_ratio.view(theme)))
                        .push(space_l3())
                        .push(container_l3(self.frequency_free.view(theme))),
                )),
        )
        .into()
    }
}
