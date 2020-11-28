use iced_native::{button, Align, Button, slider, Slider, Column, Command, Element, Row, Text, pick_list, PickList};
use once_cell::sync::Lazy;


type Renderer = super::bridge::Renderer;


pub struct Application {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    slider_value: f32,
    slider: slider::State,
    pick_list: pick_list::State<Step<f64>>,
    operator_ratio_step: Step<f64>,
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    SliderChanged(f32),
    RatioStep(Step<f64>),
}


#[derive(Debug, Clone, Copy)]
pub struct Step<T> {
    index: usize,
    value: T,
}


impl <T>PartialEq for Step<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}


impl <T>Eq for Step<T> {}


impl ::std::fmt::Display for Step<f64> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.value.fmt(f)
    }
}


static OPERATOR_RATIO_STEPS: Lazy<Vec<Step<f64>>> = Lazy::new(|| {
    crate::constants::OPERATOR_RATIO_STEPS.iter()
        .enumerate()
        .map(|(index, value)| {
            Step {
                index,
                value: *value
            }
        })
        .collect()
});


impl super::bridge::Application for Application {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: Default::default(),
            increment_button: Default::default(),
            decrement_button: Default::default(),
            slider_value: Default::default(),
            slider: Default::default(),
            pick_list: Default::default(),
            operator_ratio_step: *OPERATOR_RATIO_STEPS.first().unwrap(),
        }
    }

    fn title(&self) -> String {
        "Test".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::SliderChanged(value) => {
                self.slider_value = value;
            },
            Message::RatioStep(value) => {
                self.operator_ratio_step = value;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Renderer> {
        let buttons = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            );
        
        let slider = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Slider::new(&mut self.slider, 0.0..=100.0, self.slider_value, |value| Message::SliderChanged(value)))
            .push(Text::new(self.slider_value.to_string()).size(14));
        
        let pick_list = {
            let pick_list = PickList::new(
                &mut self.pick_list,
                OPERATOR_RATIO_STEPS.as_slice(),
                Some(self.operator_ratio_step),
                |step| Message::RatioStep(step)
            );

            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(pick_list)
                .push(Text::new(self.operator_ratio_step.value.to_string()))
        };
        
        Column::new()
            .push(
                Row::new()
                    .padding(20)
                    .push(buttons)
                    .push(slider)
            ).push(
                Row::new()
                    .padding(20)
                    .push(pick_list)
            )
            .into()
    }
}