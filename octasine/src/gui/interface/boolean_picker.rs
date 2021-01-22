use iced_baseview::{
    Column, Element, Text, Length, HorizontalAlignment, Align, Radio, Space
};

use crate::common::*;
use crate::GuiSyncHandle;
use crate::parameters::values::{
    ParameterValue,
    LfoBpmSyncValue,
    LfoModeValue,
    OperatorWaveTypeValue
};

use super::{FONT_BOLD, FONT_SIZE, LINE_HEIGHT, Message};


pub fn wave_type<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> BooleanPicker<OperatorWaveTypeValue> {
    BooleanPicker::new(
        sync_handle,
        parameter_index,
        "WAVE",
        vec![WaveType::Sine, WaveType::WhiteNoise],
    )
}


pub fn bpm_sync<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> BooleanPicker<LfoBpmSyncValue> {
    BooleanPicker::new(
        sync_handle,
        parameter_index,
        "BPM SYNC",
        vec![true, false]
    )
}


pub fn lfo_mode<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
) -> BooleanPicker<LfoModeValue> {
    BooleanPicker::new(
        sync_handle,
        parameter_index,
        "MODE",
        vec![LfoMode::Forever, LfoMode::Once]
    )
}


#[derive(Debug, Clone)]
pub struct BooleanPicker<P: ParameterValue> {
    title: String,
    parameter_index: usize,
    selected: P::Value,
    choices: Vec<P::Value>,
}


impl<P: ParameterValue> BooleanPicker<P> where P::Value: Eq + Copy + 'static {
    fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        title: &str,
        choices: Vec<P::Value>,
    ) -> Self {
        let sync_value = sync_handle.get_parameter(parameter_index);
        let selected = P::from_sync(sync_value).get();

        Self {
            title: title.into(),
            parameter_index,
            choices,
            selected
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.selected = P::from_sync(value).get();
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new(self.title.clone())
            .horizontal_alignment(HorizontalAlignment::Center)
            .font(FONT_BOLD);
        
        let mut radios = Column::new()
            .spacing(4);
        
        for choice in self.choices.clone().into_iter() {
            let parameter_index = self.parameter_index;

            let radio = Radio::new(
                choice,
                P::from_processing(choice).format().to_uppercase(),
                Some(self.selected),
                move |choice| {
                    Message::ParameterChange(
                        parameter_index,
                        P::from_processing(choice).to_sync()
                    )
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
