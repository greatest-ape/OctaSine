use iced_baseview::canvas::{path, Frame, Stroke};
use iced_baseview::{Color, Point};
use palette::gradient::Gradient;
use palette::Srgba;

use crate::gui::interface::SnapPoint;

use super::StyleSheet;

pub struct ModulationLine {
    from: Point,
    points: Vec<Point>,
    color: Color,
}

impl ModulationLine {
    pub fn new(from: Point, mod_index: f64, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut line = Self {
            from,
            points: vec![],
            color: Color::TRANSPARENT,
        };

        line.update(vec![], style_sheet, mod_index);

        line
    }

    pub fn update(&mut self, points: Vec<Point>, style_sheet: Box<dyn StyleSheet>, mod_index: f64) {
        let bg = style_sheet.active().background_color;
        let c = style_sheet.active().line_max_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            Srgba::new(0.25, 0.5, 1.0, 1.0).into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(mod_index as f32);
        let color = Color::from(Srgba::from_linear(color));

        self.points = points;
        self.color = color;
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
