use iced_baseview::{
    Column, Element, Text, Length, HorizontalAlignment, Align, Row, Radio, Space
};

use vst2_helpers::processing_parameters::utils::{
    map_parameter_value_to_step,
    map_step_to_parameter_value
};

use crate::GuiSyncHandle;

use super::{ParameterWidget, Message};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    One,
    Two,
    Three
}


impl Choice {
    fn to_str(&self) -> &str {
        match self {
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
        }
    }
}


#[derive(Debug, Clone)]
pub struct ModOutputPicker {
    title: String,
    parameter_index: usize,
    selected: Choice,
    choices: Vec<Choice>,
}


impl ModOutputPicker {
    fn selected_as_f64(&self) -> f64 {
        map_step_to_parameter_value(&self.choices, self.selected)
    }

    pub fn operator_3_mod_output<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        let value = sync_handle.get_presets()
            .get_parameter_value_float(parameter_index);
        
        let choices = vec![Choice::Two, Choice::One];
        
        let selected = map_parameter_value_to_step(&choices[..], value);
        
        Self {
            title: "Mod. out".to_string(),
            parameter_index,
            choices,
            selected,
        }
    }

    pub fn operator_4_mod_output<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        let value = sync_handle.get_presets()
            .get_parameter_value_float(parameter_index);
        
        let choices = vec![Choice::Three, Choice::Two, Choice::One];
        
        let selected = map_parameter_value_to_step(&choices[..], value);
        
        Self {
            title: "Mod. out".to_string(),
            parameter_index,
            choices,
            selected,
        }
    }
}


impl <H: GuiSyncHandle>ParameterWidget<H> for ModOutputPicker {
    fn view(&mut self, sync_handle: &H) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .size(12)
            .horizontal_alignment(HorizontalAlignment::Center);

        let value = {
            let value = format_value(
                sync_handle,
                self.parameter_index,
                self.selected_as_f64()
            );

            Text::new(value)
                .size(12)
                .horizontal_alignment(HorizontalAlignment::Center)
        };
        
        let mut radios = Column::new()
            .spacing(8);
        
        for choice in self.choices.clone().into_iter() {
            let parameter_index = self.parameter_index;
            let choices = self.choices.clone();

            let radio = Radio::new(
                choice,
                choice.to_str(),
                Some(self.selected),
                move |choice| {
                    let value = map_step_to_parameter_value(
                        &choices[..],
                        choice
                    );

                    Message::ParameterChange(parameter_index, value)
                }
            )
                .size(12)
                .text_size(12)
                .spacing(4);

            radios = radios.push(radio);
        }
            
        Column::new()
            .width(Length::Units(64))
            .align_items(Align::Center)
            .push(Space::with_height(Length::Units(4)))
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .push(title)
            )
            .push(Space::with_height(Length::Units(16)))
            .push(
                Row::new()
                    .align_items(Align::Start)
                    .height(Length::Units(64))
                    .push(radios)
            )
            .into()
    }

    fn set_value(&mut self, value: f64) {
        self.selected = map_parameter_value_to_step(&self.choices[..], value);
    }
}


fn format_value<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    value: f64
) -> String {
    sync_handle.get_presets().format_parameter_value(parameter_index, value)
}
