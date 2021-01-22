use iced_baseview::{
    Column, Element, keyboard::Modifiers, Text, Length, HorizontalAlignment, Align, Space
};
use iced_audio::{
    knob, Normal, NormalParam, text_marks, tick_marks
};

use crate::parameters::values::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue,
    LfoShapeValue, MasterFrequency, MasterVolume, OperatorAdditive,
    OperatorFeedback, OperatorFrequencyFine, OperatorFrequencyFree,
    OperatorFrequencyRatio, OperatorModulationIndex, OperatorPanning,
    OperatorVolume, ParameterValue
};
use crate::GuiSyncHandle;
use crate::constants::*;

use super::{FONT_BOLD, LINE_HEIGHT, Message};


const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);


enum TickMarkType {
    MinMaxAndDefault,
    MinMax,
    Other(tick_marks::Group),
}


pub fn master_volume<H: GuiSyncHandle>(
    sync_handle: &H,
) -> OctaSineKnob<MasterVolume> {
    let parameter_index = 0;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "VOLUME",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn master_frequency<H: GuiSyncHandle>(
    sync_handle: &H,
) -> OctaSineKnob<MasterFrequency> {
    let parameter_index = 1;

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREQ",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_volume<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    operator_index: usize,
) -> OctaSineKnob<OperatorVolume> {
    OctaSineKnob::new_with_default_sync_value(
        sync_handle,
        parameter_index,
        "VOLUME",
        TickMarkType::MinMaxAndDefault,
        OperatorVolume::new(operator_index).to_sync(),
    )
}


pub fn operator_panning<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorPanning> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "PAN",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_additive<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorAdditive> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "ADDITIVE",
        TickMarkType::MinMax
    )
}


pub fn operator_mod_index<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorModulationIndex> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "MOD",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_feedback<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorFeedback> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FEEDBACK",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_frequency_ratio<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorFrequencyRatio> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_frequency_free<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorFrequencyFree> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn operator_frequency_fine<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<OperatorFrequencyFine> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FINE",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn lfo_shape<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> OctaSineKnob<LfoShapeValue> {
    let tick_marks = tick_marks::Group::evenly_spaced(
        LFO_SHAPE_STEPS.len(),
        tick_marks::Tier::Two
    );

    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "SHAPE",
        TickMarkType::Other(tick_marks)
    )
}


pub fn lfo_frequency_ratio<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<LfoFrequencyRatioValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "RATIO",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn lfo_frequency_free<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<LfoFrequencyFreeValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "FREE",
        TickMarkType::MinMaxAndDefault
    )
}


pub fn lfo_amount<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize
) -> OctaSineKnob<LfoAmountValue> {
    OctaSineKnob::new(
        sync_handle,
        parameter_index,
        "AMOUNT",
        TickMarkType::MinMaxAndDefault
    )
}


#[derive(Debug, Clone)]
pub struct OctaSineKnob<P: ParameterValue> {
    knob_state: knob::State,
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    value_text: String,
    parameter_index: usize,
    phantom_data: ::std::marker::PhantomData<P>,
}


impl <P: ParameterValue + Default>OctaSineKnob<P> {
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
    ) -> Self {
        Self::new_with_default_sync_value(
            sync_handle,
            parameter_index,
            title,
            tick_mark_type,
            P::default().to_sync()
        )
    }
}


impl <P: ParameterValue>OctaSineKnob<P> {
    fn new_with_default_sync_value<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        tick_mark_type: TickMarkType,
        default_sync_value: f64,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let value_text = P::from_sync(sync_value).format();

        let knob_state = knob::State::new(
            NormalParam {
                value: Normal::new(sync_value as f32),
                default: Normal::new(default_sync_value as f32),
            }
        );

        let tick_marks = match tick_mark_type {
            TickMarkType::MinMaxAndDefault => {
                tick_marks_from_min_max_and_value(default_sync_value)
            },
            TickMarkType::MinMax => {
                tick_marks::Group::min_max(
                    tick_marks::Tier::Two,
                )
            },
            TickMarkType::Other(tick_marks) => tick_marks,
        };

        Self {
            knob_state,
            text_marks: None,
            tick_marks: Some(tick_marks),
            title: title.to_string(),
            value_text,
            parameter_index,
            phantom_data: ::std::marker::PhantomData::default(),
        }
    }

    pub fn set_value(&mut self, value: f64){
        if !self.knob_state.is_dragging() {
            self.knob_state.set_normal(Normal::new(value as f32));
        }

        self.value_text = P::from_sync(value).format();
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);

        let value = Text::new(self.value_text.clone())
            .horizontal_alignment(HorizontalAlignment::Center);

        let parameter_index = self.parameter_index;

        let modifier_keys = Modifiers {
            shift: true,
            ..Default::default()
        };

        let mut knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| Message::ParameterChange(
                parameter_index,
                value.as_f32() as f64
            ),
        )
            .size(Length::from(KNOB_SIZE))
            .modifier_keys(modifier_keys);
        
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
            .push(
                Space::with_height(Length::Units(LINE_HEIGHT))
            )
            .push(knob)
            .push(
                Space::with_height(Length::Units(LINE_HEIGHT))
            )
            .push(value)
            .into()
    }
}


fn tick_marks_from_min_max_and_value(
    sync_value: f64,
) -> tick_marks::Group {
    let marks = vec![
        (Normal::new(0.0), tick_marks::Tier::Two),
        (Normal::new(sync_value as f32), tick_marks::Tier::Two),
        (Normal::new(1.0), tick_marks::Tier::Two),
    ];

    tick_marks::Group::from(marks)
}
