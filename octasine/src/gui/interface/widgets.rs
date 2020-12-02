use std::sync::Arc;

use iced_baseview::{
    Container, Column, Element, Text,
};
use iced_audio::{
    knob, Normal, NormalParam
};

use crate::SyncOnlyState;
use super::{ParameterWidget, Message};


#[derive(Debug, Clone)]
pub struct OctaSineKnob {
    knob_state: knob::State,
    title: String,
    parameter_index: usize,
}


impl OctaSineKnob {
    pub fn new(
        sync_only: &Arc<SyncOnlyState>,
        title: String,
        parameter_index: usize
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
            title,
            parameter_index
        }
    }
}


impl ParameterWidget for OctaSineKnob {
    fn view(&mut self, sync_only: &Arc<SyncOnlyState>) -> Element<Message> {
        let parameter_index = self.parameter_index;
        let title = Text::new(self.title.clone()).size(12);

        let value = sync_only.presets.format_parameter_value(
            parameter_index,
            self.knob_state.normal_param.value.as_f32() as f64
        );
        let value = Text::new(value).size(12);

        let knob = knob::Knob::new(
            &mut self.knob_state,
            move |value| Message::ParameterChange(parameter_index, value),
        );

        let column = Column::new()
            .push(Container::new(title).padding(4))
            .push(Container::new(knob).padding(4))
            .push(Container::new(value).padding(4));
        
        Container::new(column)
            .padding(4)
            .into()
    }

    fn set_value(&mut self, value: f64) {
        if !self.knob_state.is_dragging() {
            self.knob_state.normal_param.value = Normal::new(value as f32);
        }
    }
}
