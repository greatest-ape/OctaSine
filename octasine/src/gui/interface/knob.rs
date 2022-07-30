use iced_audio::{knob, text_marks, tick_marks, Normal, NormalParam};
use iced_baseview::Container;
use iced_baseview::{
    alignment::Horizontal, keyboard::Modifiers, Alignment, Column, Element, Length, Space, Text,
};

use crate::parameters::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoParameter,
    MasterFrequencyValue, MasterParameter, MasterVolumeValue, OperatorFeedbackValue,
    OperatorFrequencyFineValue, OperatorFrequencyFreeValue, OperatorFrequencyRatioValue,
    OperatorMixOutValue, OperatorModOutValue, OperatorPanningValue, OperatorParameter,
    OperatorVolumeValue, Parameter, ParameterValue,
};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::value_text::ValueText;
use super::{Message, LINE_HEIGHT};

const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);

enum TickMarkType {
    MinMaxAndDefault,
}

pub fn master_volume<H>(sync_handle: &H, style: Theme) -> OctaSineKnob<MasterVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::Volume),
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
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::Frequency),
        "FREQ",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn operator_volume<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Volume),
        "VOL",
        TickMarkType::MinMaxAndDefault,
        OperatorVolumeValue::default().to_patch(),
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_mix<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorMixOutValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::MixOut),
        "MIX OUT",
        TickMarkType::MinMaxAndDefault,
        OperatorMixOutValue::new(operator_index).to_patch(),
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_panning<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorPanningValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Panning),
        "PAN",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn operator_mod_index<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorModOutValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::ModOut),
        "MOD OUT",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_feedback<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFeedbackValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Feedback),
        "FEEDBACK",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

pub fn operator_frequency_ratio<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyRatio),
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn operator_frequency_free<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyFree),
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn operator_frequency_fine<H>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFineValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyFine),
        "FINE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn lfo_frequency_ratio<H>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::FrequencyRatio),
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn lfo_frequency_free<H>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::FrequencyFree),
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_bipolar(),
    )
}

pub fn lfo_amount<H>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoAmountValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Amount),
        "AMOUNT",
        TickMarkType::MinMaxAndDefault,
        style,
        |theme| theme.knob_regular(),
    )
}

#[derive(Debug, Clone)]
pub struct OctaSineKnob<P: ParameterValue> {
    style: Theme,
    knob_state: knob::State,
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    value_text: ValueText<P>,
    default_patch_value: f32,
    parameter: Parameter,
    phantom_data: ::std::marker::PhantomData<P>,
    style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
}

impl<P> OctaSineKnob<P>
where
    P: ParameterValue + Default,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        title: &str,
        tick_mark_type: TickMarkType,
        style: Theme,
        style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
    ) -> Self {
        Self::new_with_default_sync_value(
            sync_handle,
            parameter,
            title,
            tick_mark_type,
            P::default().to_patch(),
            style,
            style_extractor,
        )
    }

    fn new_with_default_sync_value<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        title: &str,
        tick_mark_type: TickMarkType,
        default_patch_value: f32,
        style: Theme,
        style_extractor: fn(Theme) -> Box<dyn iced_audio::knob::StyleSheet>,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter);
        let value_text = ValueText::new(sync_handle, style, parameter);

        let knob_state = knob::State::new(NormalParam {
            value: Normal::new(sync_value as f32),
            default: Normal::new(default_patch_value),
        });

        let tick_marks = match tick_mark_type {
            TickMarkType::MinMaxAndDefault => {
                tick_marks_from_min_max_and_value(default_patch_value)
            }
        };

        Self {
            style,
            knob_state,
            text_marks: None,
            tick_marks: Some(tick_marks),
            title: title.to_string(),
            value_text,
            default_patch_value,
            parameter,
            phantom_data: ::std::marker::PhantomData::default(),
            style_extractor,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        if !self.knob_state.is_dragging() {
            self.knob_state.set_normal(Normal::new(value as f32));

            self.value_text.set_value(value);
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.value_text.style = style;
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(self.style.font_bold())
            .height(Length::Units(LINE_HEIGHT));

        let parameter = self.parameter;

        let modifier_keys = Modifiers::SHIFT;

        let mut knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| Message::ChangeSingleParameterSetValue(parameter, value.as_f32()),
            move || Some(Message::ChangeSingleParameterBegin(parameter)),
            move || Some(Message::ChangeSingleParameterEnd(parameter)),
        )
        .size(Length::from(KNOB_SIZE))
        .modifier_keys(modifier_keys)
        .bipolar_center(iced_audio::Normal::new(self.default_patch_value as f32))
        .style((self.style_extractor)(self.style));

        if let Some(text_marks) = self.text_marks.as_ref() {
            knob = knob.text_marks(text_marks);
        }
        if let Some(tick_marks) = self.tick_marks.as_ref() {
            knob = knob.tick_marks(tick_marks);
        }

        Container::new(
            Column::new()
                .width(Length::Units(LINE_HEIGHT * 4))
                .align_items(Alignment::Center)
                .push(title)
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(knob)
                .push(Space::with_height(Length::Units(LINE_HEIGHT)))
                .push(self.value_text.view()),
        )
        .height(Length::Units(LINE_HEIGHT * 6))
        .into()
    }
}

fn tick_marks_from_min_max_and_value(patch_value: f32) -> tick_marks::Group {
    let marks = vec![
        (Normal::new(0.0), tick_marks::Tier::One),
        (Normal::new(patch_value as f32), tick_marks::Tier::Two),
        (Normal::new(1.0), tick_marks::Tier::One),
    ];

    tick_marks::Group::from(marks)
}
