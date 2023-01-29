use iced_audio::{graphics::knob, text_marks, tick_marks, Normal, NormalParam};
use iced_baseview::widget::Container;
use iced_baseview::{
    alignment::Horizontal, keyboard::Modifiers, widget::Column, widget::Space, widget::Text,
    Alignment, Element, Length,
};

use crate::parameters::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoParameter,
    MasterFrequencyValue, MasterParameter, MasterVolumeValue, OperatorFeedbackValue,
    OperatorFrequencyFineValue, OperatorFrequencyFreeValue, OperatorFrequencyRatioValue,
    OperatorMixOutValue, OperatorModOutValue, OperatorPanningValue, OperatorParameter,
    OperatorVolumeValue, Parameter, ParameterValue, WrappedParameter,
};
use crate::sync::GuiSyncHandle;

use super::style::knob::KnobStyle;
use super::style::Theme;
use super::value_text::ValueText;
use super::{Message, LINE_HEIGHT};

const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);

enum TickMarkType {
    MinMaxAndDefault,
}

pub fn master_volume<H>(sync_handle: &H) -> OctaSineKnob<MasterVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::Volume),
        "VOLUME",
        TickMarkType::MinMaxAndDefault,
        // |theme| theme.knob_regular(),
        KnobStyle::Regular,
    )
}

pub fn master_frequency<H>(sync_handle: &H) -> OctaSineKnob<MasterFrequencyValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::Frequency),
        "FREQ",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn operator_volume<H>(
    sync_handle: &H,
    operator_index: usize,
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
        KnobStyle::Regular,
    )
}

pub fn operator_mix<H>(sync_handle: &H, operator_index: usize) -> OctaSineKnob<OperatorMixOutValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::MixOut),
        "MIX OUT",
        TickMarkType::MinMaxAndDefault,
        OperatorMixOutValue::new(operator_index).to_patch(),
        KnobStyle::Regular,
    )
}

pub fn operator_panning<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorPanningValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Panning),
        "PAN",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn operator_mod_index<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorModOutValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::ModOut),
        "MOD OUT",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Regular,
    )
}

pub fn operator_feedback<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorFeedbackValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Feedback),
        "FEEDBACK",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Regular,
    )
}

pub fn operator_frequency_ratio<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyRatio),
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn operator_frequency_free<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyFree),
        "FREE",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn operator_frequency_fine<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorFrequencyFineValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::FrequencyFine),
        "FINE",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn lfo_frequency_ratio<H>(
    sync_handle: &H,
    lfo_index: usize,
) -> OctaSineKnob<LfoFrequencyRatioValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::FrequencyRatio),
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn lfo_frequency_free<H>(
    sync_handle: &H,
    lfo_index: usize,
) -> OctaSineKnob<LfoFrequencyFreeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::FrequencyFree),
        "FREE",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Bipolar,
    )
}

pub fn lfo_amount<H>(sync_handle: &H, lfo_index: usize) -> OctaSineKnob<LfoAmountValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Amount),
        "AMOUNT",
        TickMarkType::MinMaxAndDefault,
        KnobStyle::Regular,
    )
}

pub struct OctaSineKnob<P: ParameterValue> {
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    value: NormalParam,
    value_text: ValueText<P>,
    default_value: Normal,
    parameter: WrappedParameter,
    phantom_data: ::std::marker::PhantomData<P>,
    knob_style: KnobStyle,
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
        knob_style: KnobStyle,
    ) -> Self {
        Self::new_with_default_sync_value(
            sync_handle,
            parameter,
            title,
            tick_mark_type,
            P::default().to_patch(),
            knob_style,
        )
    }

    fn new_with_default_sync_value<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        title: &str,
        tick_mark_type: TickMarkType,
        default_patch_value: f32,
        knob_style: KnobStyle,
    ) -> Self {
        let parameter = parameter.into();

        let default_value = Normal::from_clipped(default_patch_value);
        let value = NormalParam {
            value: Normal::from_clipped(sync_handle.get_parameter(parameter) as f32),
            default: default_value,
        };
        let value_text = ValueText::new(sync_handle, parameter);

        let tick_marks = match tick_mark_type {
            TickMarkType::MinMaxAndDefault => {
                tick_marks_from_min_max_and_value(default_patch_value)
            }
        };

        Self {
            text_marks: None,
            tick_marks: Some(tick_marks),
            title: title.to_string(),
            value,
            value_text,
            default_value,
            parameter,
            phantom_data: ::std::marker::PhantomData::default(),
            knob_style,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        // FIXME
        // if !self.knob_state.is_dragging() {
        //     self.knob_state.set_normal(Normal::new(value as f32));
        // }
        self.value.update(Normal::from_clipped(value as f32));

        self.value_text.set_value(value);
    }

    pub fn view<'a>(&'a self, theme: &Theme) -> Element<Message, Theme> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_bold())
            .height(Length::Units(LINE_HEIGHT));

        let parameter = self.parameter;

        let modifier_keys = Modifiers::SHIFT;

        let mut knob: knob::Knob<'a, Message, Theme> = knob::Knob::new(self.value, move |value| {
            Message::ChangeSingleParameterSetValue(parameter, value.as_f32())
        })
        .on_grab(move || Some(Message::ChangeSingleParameterBegin(parameter)))
        .on_release(move || Some(Message::ChangeSingleParameterEnd(parameter)))
        .size(Length::from(KNOB_SIZE))
        .modifier_keys(modifier_keys)
        .style(self.knob_style)
        .bipolar_center(self.default_value);

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
                .push(self.value_text.view(theme)),
        )
        .height(Length::Units(LINE_HEIGHT * 6))
        .into()
    }
}

fn tick_marks_from_min_max_and_value(patch_value: f32) -> tick_marks::Group {
    let marks = vec![
        (Normal::from_clipped(0.0), tick_marks::Tier::One),
        (
            Normal::from_clipped(patch_value as f32),
            tick_marks::Tier::Two,
        ),
        (Normal::from_clipped(1.0), tick_marks::Tier::One),
    ];

    tick_marks::Group::from(marks)
}
