use iced_baseview::canvas::{path, Frame, Stroke};
use iced_baseview::{Color, Point};

use crate::gui::interface::SnapPoint;

use super::StyleSheet;

pub struct ModOutLine {
    from: Point,
    points: Vec<Point>,
    color: Color,
}

impl ModOutLine {
    pub fn new(from: Point, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut line = Self {
            from,
            points: vec![],
            color: Color::TRANSPARENT,
        };

        line.update(vec![], style_sheet);

        line
    }

    pub fn update(&mut self, points: Vec<Point>, style_sheet: Box<dyn StyleSheet>) {
        self.points = points;
        self.color = style_sheet.active().mod_out_line_color;
    }

    pub fn draw(&self, frame: &mut Frame) {
        let stroke = Stroke::default().with_width(3.0).with_color(self.color);

        let mut builder = path::Builder::new();

        builder.move_to(self.from.snap());

        for point in self.points.iter() {
            builder.line_to(point.snap());
        }

        let path = builder.build();

        frame.stroke(&path, stroke);
    }
}
