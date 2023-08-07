pub mod plain;

use iced_baseview::widget::tooltip::Position;
use iced_baseview::widget::Container;
use iced_baseview::{
    core::alignment::Horizontal, widget::Column, widget::Text, core::Alignment, core::Element, core::Length,
};

use crate::parameters::glide_time::GlideTimeValue;
use crate::parameters::master_pitch_bend_range::{
    MasterPitchBendRangeDownValue, MasterPitchBendRangeUpValue,
};
use crate::parameters::velocity_sensitivity::VelocitySensitivityValue;
use crate::parameters::{
    LfoAmountValue, LfoFrequencyFreeValue, LfoFrequencyRatioValue, LfoParameter,
    MasterFrequencyValue, MasterParameter, MasterVolumeValue, OperatorFeedbackValue,
    OperatorFrequencyFineValue, OperatorFrequencyFreeValue, OperatorFrequencyRatioValue,
    OperatorMixOutValue, OperatorModOutValue, OperatorPanningValue, OperatorParameter,
    OperatorVolumeValue, Parameter, ParameterValue,
};
use crate::sync::GuiSyncHandle;

use super::common::tooltip;
use super::style::Theme;
use super::value_text::ValueText;
use super::{Message, LINE_HEIGHT};

use plain::{Knob, KnobVariant};

pub fn master_volume<H>(sync_handle: &H) -> OctaSineKnob<MasterVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::Volume),
        "VOLUME",
        "Master volume",
        KnobVariant::Regular,
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
        "Master frequency",
        KnobVariant::Bipolar,
    )
}

pub fn master_velocity_sensitivity<H>(sync_handle: &H) -> OctaSineKnob<VelocitySensitivityValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::VelocitySensitivityVolume),
        "VOL VS",
        "Volume velocity sensitivity",
        KnobVariant::Regular,
    )
}

pub fn master_pitch_bend_range_up<H>(sync_handle: &H) -> OctaSineKnob<MasterPitchBendRangeUpValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_values(
        sync_handle,
        Parameter::Master(MasterParameter::PitchBendRangeUp),
        "PB UP",
        "Pitch bench range - upward",
        KnobVariant::Bipolar,
        MasterPitchBendRangeUpValue::default().to_patch(),
        0.5,
    )
}

pub fn master_pitch_bend_range_down<H>(
    sync_handle: &H,
) -> OctaSineKnob<MasterPitchBendRangeDownValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new_with_values(
        sync_handle,
        Parameter::Master(MasterParameter::PitchBendRangeDown),
        "PB DOWN",
        "Pitch bench range - downward",
        KnobVariant::Bipolar,
        MasterPitchBendRangeDownValue::default().to_patch(),
        0.5,
    )
}

pub fn glide_time<H>(sync_handle: &H) -> OctaSineKnob<GlideTimeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Master(MasterParameter::GlideTime),
        "GL TIME",
        "Glide time",
        KnobVariant::Regular,
    )
}

pub fn operator_volume<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<OperatorVolumeValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Volume),
        "VOL",
        "Volume",
        KnobVariant::Regular,
    )
}

pub fn operator_mix<H>(sync_handle: &H, operator_index: usize) -> OctaSineKnob<OperatorMixOutValue>
where
    H: GuiSyncHandle,
{
    let default_and_center = OperatorMixOutValue::new(operator_index).to_patch();

    OctaSineKnob::new_with_values(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::MixOut),
        "MIX OUT",
        "Amount of signal sent directly to DAW",
        KnobVariant::Regular,
        default_and_center,
        default_and_center,
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
        "Panning",
        KnobVariant::Bipolar,
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
        "Amount of signal sent to modulation targets",
        KnobVariant::Regular,
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
        "Amount of self-modulation",
        KnobVariant::Regular,
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
        "Frequency - fixed ratios",
        KnobVariant::Bipolar,
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
        "Frequency - free",
        KnobVariant::Bipolar,
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
        "Frequency - fine tuning",
        KnobVariant::Bipolar,
    )
}

pub fn operator_feedback_velocity_sensitivity<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<VelocitySensitivityValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(
            operator_index as u8,
            OperatorParameter::VelocitySensitivityFeedback,
        ),
        "FB VS",
        "Feedback velocity sensitivity",
        KnobVariant::Regular,
    )
}

pub fn operator_mod_out_velocity_sensitivity<H>(
    sync_handle: &H,
    operator_index: usize,
) -> OctaSineKnob<VelocitySensitivityValue>
where
    H: GuiSyncHandle,
{
    OctaSineKnob::new(
        sync_handle,
        Parameter::Operator(
            operator_index as u8,
            OperatorParameter::VelocitySensitivityModOut,
        ),
        "MOD VS",
        "Modulation output velocity sensitivity",
        KnobVariant::Regular,
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
        "Frequency - fixed ratios",
        KnobVariant::Bipolar,
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
        "Frequency - free",
        KnobVariant::Bipolar,
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
        "How much LFO affects target parameter",
        KnobVariant::Regular,
    )
}

pub struct OctaSineKnob<P: ParameterValue> {
    title: String,
    tooltip_text: String,
    value_text: ValueText<P>,
    phantom_data: ::std::marker::PhantomData<P>,
    knob: Knob,
}

impl<P> OctaSineKnob<P>
where
    P: ParameterValue + Default,
{
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        title: &str,
        tooltip_text: &str,
        knob_variant: KnobVariant,
    ) -> Self {
        let patch_value = P::default().to_patch();

        Self::new_with_values(
            sync_handle,
            parameter,
            title,
            tooltip_text,
            knob_variant,
            patch_value,
            patch_value,
        )
    }

    fn new_with_values<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        title: &str,
        tooltip_text: &str,
        knob_variant: KnobVariant,
        default_patch_value: f32,
        anchor_dot_value: f32,
    ) -> Self {
        let parameter = parameter.into();

        let value_text = ValueText::new(sync_handle, parameter);

        let knob = Knob::new(
            parameter,
            knob_variant,
            anchor_dot_value,
            default_patch_value,
            sync_handle.get_parameter(parameter),
        );

        Self {
            title: title.to_string(),
            tooltip_text: tooltip_text.to_string(),
            value_text,
            phantom_data: ::std::marker::PhantomData::default(),
            knob,
        }
    }
    pub fn set_value(&mut self, value: f32) {
        self.knob.set_value(value);

        self.value_text.set_value(value);
    }

    pub fn theme_changed(&mut self) {
        self.knob.theme_changed();
    }

    pub fn view<'a>(&'a self, theme: &Theme) -> Element<Message, iced_baseview::Renderer<Theme>> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_bold())
            .height(Length::Fixed(LINE_HEIGHT.into()));
        let title = tooltip(theme, &self.tooltip_text, Position::Top, title);

        Container::new(
            Column::new()
                .width(Length::Fixed(f32::from(LINE_HEIGHT * 4)))
                .align_items(Alignment::Center)
                .push(title)
                .push(self.knob.view())
                .push(self.value_text.view(theme)),
        )
        .height(Length::Fixed(f32::from(LINE_HEIGHT * 6)))
        .into()
    }
}
