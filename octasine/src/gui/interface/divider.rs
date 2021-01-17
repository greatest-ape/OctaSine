use iced_baseview::canvas::{
    Canvas, Cursor, Geometry, Program, Frame
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Container, Align
};

use super::Message;


/// Vertical rule that fills whole height
#[derive(Clone)]
pub struct VerticalRule {
    width: Length,
    height: Length,
    color: Color,
}


impl VerticalRule {
    pub fn new(width: Length, height: Length) -> Self {
        let color = Color::from_rgb(0.8, 0.8, 0.8);

        Self {
            width,
            height,
            color,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let width = self.width;
        let height = self.height;

        let canvas = Canvas::new(self)
            .width(Length::Units(1))
            .height(height);

        Container::new(canvas)
            .width(width)
            .align_x(Align::Center)
            .into()
    }
}


impl Program<Message> for VerticalRule {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        frame.fill_rectangle(Point::ORIGIN, frame.size(), self.color);

        vec![frame.into_geometry()]
    }
}
