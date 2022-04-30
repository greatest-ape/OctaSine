use iced_audio::{knob, text_marks, tick_marks, Normal, NormalParam};
use iced_baseview::{
    alignment::Horizontal, keyboard::Modifiers, Alignment, Column, Element, Length, Space, Text,
};

use crate::parameter_values::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, MasterFrequencyValue,
    MasterVolumeValue, OperatorFeedbackValue, OperatorFrequencyFineValue,
    OperatorFrequencyFreeValue, OperatorFrequencyRatioValue, OperatorMixValue,
    OperatorModulationIndexValue, OperatorPanningValue, OperatorVolumeValue, ParameterValue,
};
use crate::sync::GuiSyncHandle;

use super::style::{Style, Theme};
use super::{Message, FONT_BOLD, LINE_HEIGHT};

const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);

enum TickMarkType {
    MinMaxAndDefault,
}

pub fn master_volume<H>(sync_handle: &H, style: Theme) -> OctaSineKnob<MasterVolumeValue>
where
    H: GuiSyncHandle,
{
    let parameter_index = 0;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "VOLUME",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn master_frequency<H>(sync_handle: &H, style: Theme) -> OctaSineKnob<MasterFrequencyValue>
where
    H: GuiSyncHandle,
{
    let parameter_index = 1;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREQ",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_volume<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        parameter_index,
        "VOL",
        TickMarkType::MinMaxAndDefault,
        OperatorVolumeValue::default().to_patch(),
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_mix<H>(
    sync_handle: &H,
    parameter_index: usize,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorMixValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        parameter_index,
        "MIX OUT",
        TickMarkType::MinMaxAndDefault,
        OperatorMixValue::new(operator_index).to_patch(),
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_panning<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorPanningValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "PAN",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn operator_mod_index<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorModulationIndexValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "MOD OUT",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_feedback<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFeedbackValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FEEDBACK",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_frequency_ratio<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_frequency_free<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_frequency_fine<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFineValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FINE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn lfo_frequency_ratio<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn lfo_frequency_free<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn lfo_amount<H>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoAmountValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "AMOUNT",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

#[derive(Debug, Clone)]
pub struct OctaSineKnob<P: ParameterValue> {
    pub style: Theme,
    knob_state: knob::State,
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    value_text: String,
    parameter_index: usize,
    phantom_data: ::std::marker::PhantomData<P>,
    style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
}

impl<P> OctaSineKnob<P>
where
    P: ParameterValue + Default,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
        style: Theme,
        style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
    ) -> Self {
        Self::new_with_default_sync_value(
            sync_handle,
            parameter_index,
            title,
            tick_mark_type,
            P::default().to_patch(),
            style,
            style_extractor,
        )
    }

    fn new_with_default_sync_value<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
        default_sync_value: f64,
        style: Theme,
        style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let value_text = P::new_from_patch(sync_value).get_formatted();

        let knob_state = knob::State::new(NormalParam {
            value: Normal::new(sync_value as f32),
            default: Normal::new(default_sync_value as f32),
        });

        let tick_marks = match tick_mark_type {
            TickMarkType::MinMaxAndDefault => tick_marks_from_min_max_and_value(default_sync_value),
        };

        Self {
            style,
            knob_state,
            text_marks: None,
            tick_marks: Some(tick_marks),
            title: title.to_string(),
            value_text,
            parameter_index,
            phantom_data: ::std::marker::PhantomData::default(),
            style_extractor,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        if !self.knob_state.is_dragging() {
            self.knob_state.set_normal(Normal::new(value as f32));
        }

        self.value_text = P::new_from_patch(value).get_formatted();
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(FONT_BOLD);

        let value = Text::new(self.value_text.clone()).horizontal_alignment(Horizontal::Center);

        let parameter_index = self.parameter_index;

        let modifier_keys = Modifiers::SHIFT;

        let mut knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| {
                Message::ChangeSingleParameterSetValue(parameter_index, value.as_f32() as f64)
            },
            move || Some(Message::ChangeSingleParameterBegin(parameter_index)),
            move || Some(Message::ChangeSingleParameterEnd(parameter_index)),
        )
        .size(Length::from(KNOB_SIZE))
        .modifier_keys(modifier_keys)
        .style((self.style_extractor)(self.style));

        if let Some(text_marks) = self.text_marks.as_ref() {
            knob = knob.text_marks(text_marks);
        }
        if let Some(tick_marks) = self.tick_marks.as_ref() {
            knob = knob.tick_marks(tick_marks);
        }

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(knob)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(value)
            .into()
    }
}

fn tick_marks_from_min_max_and_value(sync_value: f64) -> tick_marks::Group {
    let marks = vec![
        (Normal::new(0.0), tick_marks::Tier::Two),
        (Normal::new(sync_value as f32), tick_marks::Tier::Two),
        (Normal::new(1.0), tick_marks::Tier::Two),
    ];

    tick_marks::Group::from(marks)
}
