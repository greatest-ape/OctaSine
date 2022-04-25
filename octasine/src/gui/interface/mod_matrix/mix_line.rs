use iced_baseview::canvas::{Frame, Path, Stroke};
use iced_baseview::{Color, Point};
use palette::gradient::Gradient;
use palette::Srgba;

use crate::gui::interface::SnapPoint;

use super::StyleSheet;

pub struct AdditiveLine {
    path: Path,
    color: Color,
}

impl AdditiveLine {
    pub fn new(from: Point, to_y: f32, additive: f64, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from.snap(), to.snap());

        let mut line = Self {
            path,
            color: style_sheet.active().line_max_color,
        };

        line.update(additive, style_sheet);

        line
    }

    pub fn update(&mut self, additive: f64, style_sheet: Box<dyn StyleSheet>) {
        self.color = Self::calculate_color(additive, style_sheet);
    }

    fn calculate_color(additive: f64, style_sheet: Box<dyn StyleSheet>) -> Color {
        let bg = style_sheet.active().background_color;
        let c = style_sheet.active().line_max_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            Srgba::new(0.23, 0.69, 0.06, 1.0).into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(additive as f32);

        Color::from(Srgba::from_linear(color))
    }

    pub fn draw(&self, frame: &mut Frame) {
        let stroke = Stroke::default().with_width(3.0).with_color(self.color);

        frame.stroke(&self.path, stroke);
    }
}
