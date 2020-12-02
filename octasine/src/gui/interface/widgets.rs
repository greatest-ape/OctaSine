use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text, Length, Align
};
use iced_audio::{
    knob, Normal, NormalParam, text_marks, tick_marks
};

use crate::SyncOnlyState;
use crate::constants::{MASTER_FREQUENCY_STEPS};

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
    pub fn new(
        sync_only: &Arc<SyncOnlyState>,
        title: String,
        parameter_index: usize,
        text_marks: Option<text_marks::Group>,
        tick_marks: Option<tick_marks::Group>,
    ) -> Self {
        let value = Normal::new(sync_only.presets.get_parameter_value_float(
            parameter_index
        ) as f32);

        let normal_param = NormalParam {
            value,
            default: value, // FIXME
        };
        
        Self {
            knob_state: knob::State::new(normal_param),
            text_marks,
            tick_marks,
            title,
            parameter_index
        }
    }

    pub fn master_volume(
        sync_only: &Arc<SyncOnlyState>,
    ) -> Self {
        let parameter_index = 0;
        let text_marks = text_marks_from_min_max_center(
            &sync_only,
            parameter_index
        );
        let tick_marks = tick_marks::Group::min_max_and_center(
            tick_marks::Tier::One,
            tick_marks::Tier::One,
        );

        Self::new(
            &sync_only,
            "Master\nvolume".to_string(),
            parameter_index,
            Some(text_marks),
            Some(tick_marks),
        )
    }

    pub fn master_frequency(
        sync_only: &Arc<SyncOnlyState>,
    ) -> Self {
        let parameter_index = 1;
        let text_marks = text_marks_from_steps(&MASTER_FREQUENCY_STEPS);
        let tick_marks = tick_marks_from_steps(&MASTER_FREQUENCY_STEPS);

        Self::new(
            &sync_only,
            "Master\nfrequency".to_string(),
            parameter_index,
            Some(text_marks),
            Some(tick_marks),
        )
    }
}


impl ParameterWidget for OctaSineKnob {
    fn view(&mut self, sync_only: &Arc<SyncOnlyState>) -> Element<Message> {
        let title = Text::new(self.title.clone()).size(12);
        let value = format_value(
            sync_only,
            self.parameter_index,
            self.knob_state.normal_param.value.as_f32() as f64
        );
        let value = Text::new(value).size(12);

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

        let column = Column::new()
            .push(Container::new(title).padding(16))
            .push(Container::new(knob).padding(16))
            .push(Container::new(value).padding(16));
        
        Container::new(column)
            .padding(16)
            .into()
    }

    fn set_value(&mut self, value: f64) {
        if !self.knob_state.is_dragging() {
            self.knob_state.normal_param.value = Normal::new(value as f32);
        }
    }
}


fn format_value(
    sync_only: &Arc<SyncOnlyState>,
    parameter_index: usize,
    value: f64
) -> String {
    sync_only.presets.format_parameter_value(parameter_index, value)
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



fn text_marks_from_min_max_center(
    sync_only: &Arc<SyncOnlyState>,
    parameter_index: usize,
) -> text_marks::Group {
    let min = format_value(sync_only, parameter_index, 0.0);
    let max = format_value(sync_only, parameter_index, 1.0);
    let center = format_value(sync_only, parameter_index, 0.5);

    text_marks::Group::min_max_and_center(&min, &max, &center)
}