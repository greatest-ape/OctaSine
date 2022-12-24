use arrayvec::ArrayVec;
use iced_baseview::widget::canvas::{path, Frame, Path, Stroke};
use iced_baseview::Point;

use crate::gui::style::Theme;

use super::StyleSheet;

pub struct ModOutLines {
    from: Point,
    paths: ArrayVec<Path, 3>,
}

impl ModOutLines {
    pub fn new(from: Point) -> Self {
        Self {
            from,
            paths: Default::default(),
        }
    }

    pub fn update<I: Iterator<Item = [Point; 2]>>(&mut self, lines: I) {
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

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let color = theme.appearance().mod_out_line_color;

        for path in self.paths.iter() {
            let stroke = Stroke::default().with_width(3.0).with_color(color);

            frame.stroke(&path, stroke);
        }
    }
}
