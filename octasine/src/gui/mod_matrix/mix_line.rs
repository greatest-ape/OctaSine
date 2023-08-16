use iced_baseview::core::{Color, Point};
use iced_baseview::widget::canvas::{Frame, Path, Stroke};
use palette::gradient::Gradient;
use palette::Srgba;

use crate::gui::style::Theme;
use crate::gui::SnapPoint;

use super::StyleSheet;

pub struct MixOutLine {
    path: Path,
    additive: f32,
}

impl MixOutLine {
    pub fn new(from: Point, to_y: f32, additive: f32) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from.snap(), to.snap());

        Self { path, additive }
    }

    pub fn update(&mut self, additive: f32) {
        self.additive = additive;
    }

    fn calculate_color(&self, additive: f32, theme: &Theme) -> Color {
        let bg = theme.appearance().background_color;
        let c = theme.appearance().line_max_color;
        let line_color = theme.appearance().mix_out_line_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            // Srgba::new(0.23, 0.69, 0.06, 1.0).into_linear(),
            Srgba::new(line_color.r, line_color.g, line_color.b, line_color.a).into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(additive);
        let color = Srgba::from_linear(color);

        Color::new(color.red, color.green, color.blue, color.alpha)
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let calculated_color = self.calculate_color(self.additive, theme);

        let stroke = Stroke::default()
            .with_width(3.0)
            .with_color(calculated_color);

        frame.stroke(&self.path, stroke);
    }
}
