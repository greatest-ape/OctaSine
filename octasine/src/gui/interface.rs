use iced_native::{button, Align, Button, slider, Slider, Column, Command, Element, Row, Text};


type Renderer = super::bridge::Renderer;


#[derive(Default)]
pub struct Application {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    slider_value: f32,
    slider: slider::State,
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    SliderChanged(f32),
}


impl super::bridge::Application for Application {
    type Message = Message;

    fn new() -> Self {
        Self::default()
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
        
        Row::new()
            .push(buttons)
            .push(slider)
            .into()
    }
}