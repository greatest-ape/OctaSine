use iced_baseview::canvas::{Cache, Canvas, Cursor, Geometry, Program};
use iced_baseview::{Align, Color, Container, Element, Length, Point, Rectangle};

use super::style::Theme;
use super::Message;

#[derive(Debug, Clone)]
pub struct Style {
    pub color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
}

/// Vertical rule that fills whole height
pub struct VerticalRule {
    style: Theme,
    cache: Cache,
    width: Length,
    height: Length,
}

impl VerticalRule {
    pub fn new(style: Theme, width: Length, height: Length) -> Self {
        Self {
            style,
            cache: Cache::default(),
            width,
            height,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let width = self.width;
        let height = self.height;

        let canvas = Canvas::new(self).width(Length::Units(1)).height(height);

        Container::new(canvas)
            .width(width)
            .align_x(Align::Center)
            .into()
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.cache.clear();
    }
}

impl Program<Message> for VerticalRule {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let style: Box<dyn StyleSheet> = self.style.into();
            let color = style.active().color;

            frame.fill_rectangle(Point::ORIGIN, frame.size(), color);
        });

        vec![geometry]
    }
}
