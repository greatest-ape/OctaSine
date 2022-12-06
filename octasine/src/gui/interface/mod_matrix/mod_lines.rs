use arrayvec::ArrayVec;
use iced_baseview::widget::canvas::{path, Frame, Path, Stroke};
use iced_baseview::{Color, Point};

use super::StyleSheet;

pub struct ModOutLines {
    from: Point,
    color: Color,
    paths: ArrayVec<Path, 3>,
}

impl ModOutLines {
    pub fn new(from: Point, style_sheet: Box<dyn StyleSheet>) -> Self {
        Self {
            from,
            color: style_sheet.appearance().mod_out_line_color,
            paths: Default::default(),
        }
    }

    pub fn update<I: Iterator<Item = [Point; 2]>>(
        &mut self,
        lines: I,
        style_sheet: Box<dyn StyleSheet>,
    ) {
        self.color = style_sheet.appearance().mod_out_line_color;

        self.paths = lines
            .map(|points| {
                let mut builder = path::Builder::new();

                builder.move_to(self.from);

                for point in points.iter() {
                    builder.line_to(*point);
                }

                builder.build()
            })
            .collect();
    }

    pub fn draw(&self, frame: &mut Frame) {
        for path in self.paths.iter() {
            let stroke = Stroke::default().with_width(3.0).with_color(self.color);

            frame.stroke(&path, stroke);
        }
    }
}
