use iced_baseview::canvas::{path, Frame, Stroke};
use iced_baseview::{Color, Point};

use super::StyleSheet;

pub struct ModOutLines {
    from: Point,
    lines: Vec<[Point; 2]>,
    color: Color,
}

impl ModOutLines {
    pub fn new(from: Point, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut line = Self {
            from,
            lines: Vec::new(),
            color: Color::TRANSPARENT,
        };

        line.update(Vec::new(), style_sheet);

        line
    }

    pub fn update(&mut self, lines: Vec<[Point; 2]>, style_sheet: Box<dyn StyleSheet>) {
        self.lines = lines;
        self.color = style_sheet.active().mod_out_line_color;
    }

    pub fn draw(&self, frame: &mut Frame) {
        for points in self.lines.iter() {
            let stroke = Stroke::default().with_width(3.0).with_color(self.color);

            let mut builder = path::Builder::new();

            builder.move_to(self.from);

            for point in points.iter() {
                builder.line_to(*point);
            }

            let path = builder.build();

            frame.stroke(&path, stroke);
        }
    }
}
