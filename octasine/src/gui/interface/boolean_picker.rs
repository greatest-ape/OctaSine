use iced_baseview::{
    Column, Element, Text, Length, HorizontalAlignment, Align, Radio, Space
};

use crate::parameters::utils::{
    map_parameter_value_to_step,
    map_step_to_parameter_value
};

use crate::GuiSyncHandle;
use crate::common::*;

use super::{FONT_BOLD, FONT_SIZE, LINE_HEIGHT, Message};


pub fn wave_type<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> BooleanPicker<WaveType> {
    let value = sync_handle.get_parameter(parameter_index);
    
    let choices = vec![WaveType::Sine, WaveType::WhiteNoise];
    let selected = map_parameter_value_to_step(&choices[..], value);
    
    BooleanPicker {
        title: "WAVE".to_string(),
        parameter_index,
        choices,
        selected,
        format_value: format_wave_type
    }
}


pub fn bpm_sync<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> BooleanPicker<bool> {
    let value = sync_handle.get_parameter(parameter_index);
    
    let choices = vec![true, false];
    let selected = map_parameter_value_to_step(&choices[..], value);

    fn format_value(on: bool) -> String {
        if on {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }
    
    BooleanPicker {
        title: "BPM SYNC".to_string(),
        parameter_index,
        choices,
        selected,
        format_value,
    }
}


#[derive(Debug, Clone)]
pub struct BooleanPicker<V> {
    title: String,
    parameter_index: usize,
    selected: V,
    choices: Vec<V>,
    format_value: fn(V) -> String,
}


impl<V: Copy + Eq + 'static> BooleanPicker<V> {
    pub fn set_value(&mut self, value: f64) {
        self.selected = map_parameter_value_to_step(&self.choices[..], value);
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);
        
        let mut radios = Column::new()
            .spacing(4);
        
        for choice in self.choices.clone().into_iter() {
            let parameter_index = self.parameter_index;
            let choices = self.choices.clone();
            let value_string = (self.format_value)(choice);

            let radio = Radio::new(
                choice,
                value_string,
                Some(self.selected),
                move |choice| {
                    let value = map_step_to_parameter_value(
                        &choices[..],
                        choice
                    );

                    Message::ParameterChange(parameter_index, value)
                }
            )
                .size(FONT_SIZE)
                .text_size(FONT_SIZE)
                .spacing(4);

            radios = radios.push(radio);
        }
            
        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Align::Center)
            .push(title)
            .push(
                Space::with_height(Length::Units(LINE_HEIGHT))
            )
            .push(radios)
            .into()
    }
}


fn format_wave_type(wave_type: WaveType) -> String {
    match wave_type {
        WaveType::Sine => "SINE".to_string(),
        WaveType::WhiteNoise => "NOISE".to_string(),
    }
}
