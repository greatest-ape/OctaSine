use iced_audio::{knob, text_marks, tick_marks, Normal, NormalParam};
use iced_baseview::{
    keyboard::Modifiers, Align, Column, Element, HorizontalAlignment, Length, Space, Text,
};

use crate::constants::*;
use crate::parameters::values::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoShapeValue,
    MasterFrequencyValue, MasterVolumeValue, OperatorAdditiveValue, OperatorFeedbackValue,
    OperatorFrequencyFineValue, OperatorFrequencyFreeValue, OperatorFrequencyRatioValue,
    OperatorModulationIndexValue, OperatorPanningValue, OperatorVolumeValue, ParameterValue,
};
use crate::GuiSyncHandle;

use super::style::Theme;
use super::{Message, FONT_BOLD, LINE_HEIGHT};

const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);

enum TickMarkType {
    MinMaxAndDefault,
    MinMax,
    Other(tick_marks::Group),
}

pub fn master_volume<H: GuiSyncHandle>(
    sync_handle: &H,
    style: Theme,
) -> OctaSineKnob<MasterVolumeValue> {
    let parameter_index = 0;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "VOLUME",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn master_frequency<H: GuiSyncHandle>(
    sync_handle: &H,
    style: Theme,
) -> OctaSineKnob<MasterFrequencyValue> {
    let parameter_index = 1;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREQ",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_volume<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    operator_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorVolumeValue> {
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        parameter_index,
        "VOLUME",
        TickMarkType::MinMaxAndDefault,
        OperatorVolumeValue::new(operator_index).to_sync(),
        style,
    )
}

pub fn operator_panning<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorPanningValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "PAN",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_additive<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorAdditiveValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "ADDITIVE",
        TickMarkType::MinMax,
        style,
    )
}

pub fn operator_mod_index<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorModulationIndexValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "MOD",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_feedback<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFeedbackValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FEEDBACK",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_frequency_ratio<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyRatioValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_frequency_free<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFreeValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn operator_frequency_fine<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<OperatorFrequencyFineValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FINE",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn lfo_shape<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoShapeValue> {
    let tick_marks = tick_marks::Group::evenly_spaced(LFO_SHAPE_STEPS.len(), tick_marks::Tier::Two);

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "SHAPE",
        TickMarkType::Other(tick_marks),
        style,
    )
}

pub fn lfo_frequency_ratio<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyRatioValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn lfo_frequency_free<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoFrequencyFreeValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault,
        style,
    )
}

pub fn lfo_amount<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> OctaSineKnob<LfoAmountValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "AMOUNT",
        TickMarkType::MinMaxAndDefault,
        style,
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
}

impl<P: ParameterValue + Default> OctaSineKnob<P> {
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
        style: Theme,
    ) -> Self {
        Self::new_with_default_sync_value(
            sync_handle,
            parameter_index,
            title,
            tick_mark_type,
            P::default().to_sync(),
            style,
        )
    }
}

impl<P: ParameterValue> OctaSineKnob<P> {
    fn new_with_default_sync_value<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
        default_sync_value: f64,
        style: Theme,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let value_text = P::from_sync(sync_value).format();

        let knob_state = knob::State::new(NormalParam {
            value: Normal::new(sync_value as f32),
            default: Normal::new(default_sync_value as f32),
        });

        let tick_marks = match tick_mark_type {
            TickMarkType::MinMaxAndDefault => tick_marks_from_min_max_and_value(default_sync_value),
            TickMarkType::MinMax => tick_marks::Group::min_max(tick_marks::Tier::Two),
            TickMarkType::Other(tick_marks) => tick_marks,
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
        }
    }

    pub fn set_value(&mut self, value: f64) {
        if !self.knob_state.is_dragging() {
            self.knob_state.set_normal(Normal::new(value as f32));
        }

        self.value_text = P::from_sync(value).format();
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);

        let value =
            Text::new(self.value_text.clone()).horizontal_alignment(HorizontalAlignment::Center);

        let parameter_index = self.parameter_index;

        let modifier_keys = Modifiers {
            shift: true,
            ..Default::default()
        };

        let mut knob = knob::Knob::new(&mut self.knob_state, move |value| {
            Message::ParameterChange(parameter_index, value.as_f32() as f64)
        })
        .size(Length::from(KNOB_SIZE))
        .modifier_keys(modifier_keys)
        .style(self.style);

        if let Some(text_marks) = self.text_marks.as_ref() {
            knob = knob.text_marks(text_marks);
        }
        if let Some(tick_marks) = self.tick_marks.as_ref() {
            knob = knob.tick_marks(tick_marks);
        }

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Align::Center)
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
