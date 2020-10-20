use iced_native::{Command, Element};


type Renderer = super::bridge::Renderer;


pub struct Application {

}


impl super::bridge::Application for Application {
    type Message = ();

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Test".into()
    }

    fn update(&mut self, _: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Renderer> {
        iced::Text::new("Hello").into()
    }
}