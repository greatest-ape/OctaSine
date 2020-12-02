use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text, Length, HorizontalAlignment, Align, Row
};
use iced_audio::{
    knob, Normal, NormalParam, text_marks, tick_marks
};
use vst2_helpers::processing_parameters::utils::map_value_to_parameter_value_with_steps;

use crate::SyncHandle;
use crate::constants::*;

use super::{ParameterWidget, Message};


#[derive(Debug, Clone)]
pub struct OctaSineKnob {
    knob_state: knob::State,
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    parameter_index: usize,
}


impl OctaSineKnob {
    fn new<H: SyncHandle>(
        sync_handle: &Arc<H>,
        title: String,
        parameter_index: usize,
        text_marks: Option<text_marks::Group>,
        tick_marks: Option<tick_marks::Group>,
        default: f64,
    ) -> Self {
        let value = Normal::new(sync_handle.get_presets().get_parameter_value_float(
            parameter_index
        ) as f32);

        let normal_param = NormalParam {
            value,
            default: Normal::new(default as f32),
        };
        
        Self {
            knob_state: knob::State::new(normal_param),
            text_marks,
            tick_marks,
            title,
            parameter_index
        }
    }

    pub fn new_min_max_center<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
        title: &str
    ) -> Self {
        let default_sync_value = 0.5;

        let tick_marks = tick_marks::Group::min_max_and_center(
            tick_marks::Tier::One,
            tick_marks::Tier::One,
        );

        Self::new(
            &sync_handle,
            title.to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_sync_value
        )
    }

    pub fn new_with_steps<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
        title: &str,
        steps: &[f64],
        default_value: f64,
    ) -> Self {
        let default_value_sync = map_value_to_parameter_value_with_steps(
            steps,
            default_value
        );

        let tick_marks = tick_marks_from_min_max_and_value(
            default_value_sync,
        );

        Self::new(
            &sync_handle,
            title.to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_value_sync
        )
    }

    pub fn master_volume<H: SyncHandle>(
        sync_handle: &Arc<H>,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            0,
            "Master\nvolume"
        )
    }

    pub fn master_frequency<H: SyncHandle>(
        sync_handle: &Arc<H>,
    ) -> Self {
        Self::new_with_steps(
            &sync_handle,
            1,
            "Master\nfrequency",
            &MASTER_FREQUENCY_STEPS,
            DEFAULT_MASTER_FREQUENCY
        )
    }

    pub fn operator_volume<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            parameter_index,
            "Volume"
        )
    }

    pub fn operator_panning<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            parameter_index,
            "Panning"
        )
    }

    pub fn operator_frequency_ratio<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "Ratio",
            &OPERATOR_RATIO_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_RATIO
        )
    }

    pub fn operator_frequency_free<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "Free",
            &OPERATOR_FREE_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_FREE
        )
    }

    pub fn operator_frequency_fine<H: SyncHandle>(
        sync_handle: &Arc<H>,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "Fine",
            &OPERATOR_FINE_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_FINE
        )
    }
}


impl <H: SyncHandle>ParameterWidget<H> for OctaSineKnob {
    fn view(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .size(12)
            .horizontal_alignment(HorizontalAlignment::Center);

        let value = {
            let value = format_value(
                sync_handle,
                self.parameter_index,
                self.knob_state.normal_param.value.as_f32() as f64
            );

            Text::new(value)
                .size(12)
                .horizontal_alignment(HorizontalAlignment::Center)
        };

        let parameter_index = self.parameter_index;

        let mut knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| Message::ParameterChange(parameter_index, value),
        )
            .size(Length::from(Length::Units(31)));
        
        if let Some(text_marks) = self.text_marks.as_ref() {
            knob = knob.text_marks(text_marks);
        }
        if let Some(tick_marks) = self.tick_marks.as_ref() {
            knob = knob.tick_marks(tick_marks);
        }

        Column::new()
            .width(Length::Units(64))
            .align_items(Align::Center)
            .spacing(16)
            .push(
                Row::new()
                    // .height(Length::Units(32))
                    .align_items(Align::Center)
                    .push(title)
            )
            .push(knob)
            .push(value)
            .into()
    }

    fn set_value(&mut self, value: f64) {
        if !self.knob_state.is_dragging() {
            self.knob_state.normal_param.value = Normal::new(value as f32);
        }
    }
}


fn format_value<H: SyncHandle>(
    sync_handle: &Arc<H>,
    parameter_index: usize,
    value: f64
) -> String {
    sync_handle.get_presets().format_parameter_value(parameter_index, value)
}


fn text_marks_from_steps(steps: &[f64]) -> text_marks::Group {
    let mut text_marks = Vec::with_capacity(steps.len());

    let len = (steps.len() - 1) as f32;

    for (index, value) in steps.iter().enumerate(){
        let normal = Normal::new(index as f32 / len);
        let text = format!("{:}", value);

        text_marks.push((normal, text));
    }

    text_marks::Group::from(text_marks)
}


fn tick_marks_from_steps(steps: &[f64]) -> tick_marks::Group {
    let mut tick_marks = Vec::with_capacity(steps.len());

    let len = (steps.len() - 1) as f32;

    for (index, value) in steps.iter().enumerate(){
        let normal = Normal::new(index as f32 / len);

        let tier = if index == 0 || index == steps.len() - 1 {
            tick_marks::Tier::One
        } else {
            tick_marks::Tier::Two
        };

        tick_marks.push((normal, tier));
    }

    tick_marks::Group::from(tick_marks)
}


fn text_marks_from_min_max_center<H: SyncHandle>(
    sync_handle: &Arc<H>,
    parameter_index: usize,
) -> text_marks::Group {
    let min = format_value(sync_handle, parameter_index, 0.0);
    let max = format_value(sync_handle, parameter_index, 1.0);
    let center = format_value(sync_handle, parameter_index, 0.5);

    text_marks::Group::min_max_and_center(&min, &max, &center)
}


fn text_marks_from_min_max_and_value<H: SyncHandle>(
    sync_handle: &Arc<H>,
    parameter_index: usize,
    sync_value: f64,
) -> text_marks::Group {
    let min_str = format_value(sync_handle, parameter_index, 0.0);
    let max_str = format_value(sync_handle, parameter_index, 1.0);
    let sync_value_str = format_value(sync_handle, parameter_index, sync_value);

    let marks = vec![
        (Normal::new(0.0), min_str),
        (Normal::new(sync_value as f32), sync_value_str),
        (Normal::new(1.0), max_str),
    ];

    text_marks::Group::from(marks)
}


fn tick_marks_from_min_max_and_value(
    sync_value: f64,
) -> tick_marks::Group {
    let marks = vec![
        (Normal::new(0.0), tick_marks::Tier::One),
        (Normal::new(sync_value as f32), tick_marks::Tier::One),
        (Normal::new(1.0), tick_marks::Tier::One),
    ];

    tick_marks::Group::from(marks)
}


fn text_mark_from_value<H: SyncHandle>(
    sync_handle: &Arc<H>,
    parameter_index: usize,
    sync_value: f64,
) -> text_marks::Group {
    let sync_value_str = format_value(sync_handle, parameter_index, sync_value);

    text_marks::Group::from(vec![
        (Normal::new(sync_value as f32), sync_value_str),
    ])
}


fn tick_mark_from_value(
    sync_value: f64,
) -> tick_marks::Group {
    tick_marks::Group::from(vec![
        (Normal::new(sync_value as f32), tick_marks::Tier::One),
    ])
}