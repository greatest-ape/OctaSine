use iced_baseview::{
    Column, Element, keyboard::Modifiers, Text, Length, HorizontalAlignment, Align, Space
};
use iced_audio::{
    knob, Normal, NormalParam, text_marks, tick_marks
};

use crate::parameters::utils::map_value_to_parameter_value_with_steps;
use crate::GuiSyncHandle;
use crate::constants::*;

use super::{FONT_BOLD, LINE_HEIGHT, Message};


const KNOB_SIZE: Length = Length::Units(LINE_HEIGHT * 2);


#[derive(Debug, Clone)]
pub struct OctaSineKnob {
    knob_state: knob::State,
    text_marks: Option<text_marks::Group>,
    tick_marks: Option<tick_marks::Group>,
    title: String,
    value_text: String,
    parameter_index: usize,
}


impl OctaSineKnob {
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        title: String,
        parameter_index: usize,
        text_marks: Option<text_marks::Group>,
        tick_marks: Option<tick_marks::Group>,
        default: f64,
    ) -> Self {
        let value = Normal::new(sync_handle.get_parameter(
            parameter_index
        ) as f32);

        let normal_param = NormalParam {
            value,
            default: Normal::new(default as f32),
        };

        let value_text = format_value(
            sync_handle,
            parameter_index,
            normal_param.value.as_f32() as f64
        );
        
        Self {
            knob_state: knob::State::new(normal_param),
            text_marks,
            tick_marks,
            title,
            value_text,
            parameter_index
        }
    }

    pub fn new_min_max_center<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str
    ) -> Self {
        let default_sync_value = 0.5;

        let tick_marks = tick_marks::Group::min_max_and_center(
            tick_marks::Tier::Two,
            tick_marks::Tier::Two,
        );

        Self::new(
            sync_handle,
            title.to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_sync_value
        )
    }

    pub fn new_with_steps<H: GuiSyncHandle>(
        sync_handle: &H,
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
            sync_handle,
            title.to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_value_sync
        )
    }

    pub fn master_volume<H: GuiSyncHandle>(
        sync_handle: &H,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            0,
            "MASTER\nVOLUME"
        )
    }

    pub fn master_frequency<H: GuiSyncHandle>(
        sync_handle: &H,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            1,
            "MASTER\nFREQ",
            &MASTER_FREQUENCY_STEPS,
            DEFAULT_MASTER_FREQUENCY
        )
    }

    pub fn operator_volume<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            parameter_index,
            "VOLUME"
        )
    }

    pub fn operator_panning<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            parameter_index,
            "PAN"
        )
    }

    pub fn operator_mod_index<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "MOD",
            &OPERATOR_BETA_STEPS,
            DEFAULT_OPERATOR_MODULATION_INDEX,
        )
    }

    pub fn operator_feedback<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        let default_sync_value = 0.5;

        let tick_marks = tick_marks::Group::min_max(
            tick_marks::Tier::Two,
        );

        Self::new(
            sync_handle,
            "FEEDBACK".to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_sync_value
        )
    }

    pub fn operator_frequency_ratio<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "RATIO",
            &OPERATOR_RATIO_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_RATIO
        )
    }

    pub fn operator_frequency_free<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "FREE",
            &OPERATOR_FREE_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_FREE
        )
    }

    pub fn operator_frequency_fine<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            "FINE",
            &OPERATOR_FINE_STEPS,
            DEFAULT_OPERATOR_FREQUENCY_FINE
        )
    }

    pub fn operator_additive<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        let default_sync_value = 0.0;

        let tick_marks = tick_marks::Group::min_max(
            tick_marks::Tier::Two,
        );

        Self::new(
            sync_handle,
            "ADDITIVE".to_string(),
            parameter_index,
            None,
            Some(tick_marks),
            default_sync_value
        )
    }

    pub fn lfo_frequency_ratio<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            &format!("LFO {}\nRATIO", lfo_index + 1),
            &LFO_FREQUENCY_RATIO_STEPS,
            1.0
        )
    }

    pub fn lfo_frequency_free<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
    ) -> Self {
        Self::new_with_steps(
            sync_handle,
            parameter_index,
            &format!("LFO {}\nFREE", lfo_index + 1),
            &LFO_FREQUENCY_FREE_STEPS,
            1.0
        )
    }

    pub fn lfo_magnitude<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
    ) -> Self {
        Self::new_min_max_center(
            sync_handle,
            parameter_index,
            &format!("LFO {}\nAMOUNT", lfo_index + 1),
        )
    }

    pub fn lfo_other<H: GuiSyncHandle>(
        sync_handle: &H,
        lfo_index: usize,
        parameter_index: usize,
        title: &str
    ) -> Self {
        Self::new(
            sync_handle,
            format!("LFO {}\n{}", lfo_index + 1, title),
            parameter_index,
            None,
            None,
            0.5,
        )
    }

    pub fn set_value<H: GuiSyncHandle>(&mut self, sync_handle: &H, value: f64){
        if !self.knob_state.is_dragging() {
            self.knob_state.set_normal(Normal::new(value as f32));
        }

        self.value_text = format_value(
            sync_handle,
            self.parameter_index,
            self.knob_state.normal_param.value.as_f32() as f64
        );
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


fn format_value<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    value: f64
) -> String {
    sync_handle.format_parameter_value(parameter_index, value)
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
