use iced_baseview::{
    Column, Element, Text, Length, HorizontalAlignment, Align, Radio, Space
};

use crate::parameters::utils::{
    map_parameter_value_to_step,
    map_step_to_parameter_value
};

use crate::GuiSyncHandle;
use crate::common::WaveType;

use super::{FONT_BOLD, FONT_SIZE, LINE_HEIGHT, Message};


#[derive(Debug, Clone)]
pub struct WaveTypePicker {
    title: String,
    parameter_index: usize,
    selected: WaveType,
    choices: Vec<WaveType>,
}


impl WaveTypePicker {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
    ) -> Self {
        let value = sync_handle.get_parameter(parameter_index);
        
        let choices = vec![WaveType::Sine, WaveType::WhiteNoise];
        let selected = map_parameter_value_to_step(&choices[..], value);
        
        Self {
            title: "WAVE".to_string(),
            parameter_index,
            choices,
            selected,
        }
    }

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

            let radio = Radio::new(
                choice,
                format_wave_type(choice),
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
