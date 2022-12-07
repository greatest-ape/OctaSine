use iced_baseview::widget::canvas::{Frame, Path, Stroke};
use iced_baseview::{Color, Point};
use palette::gradient::Gradient;
use palette::Srgba;

use crate::gui::interface::style::Theme;
use crate::gui::interface::SnapPoint;

use super::StyleSheet;

pub struct MixOutLine {
    path: Path,
    line_color: Color,
    calculated_color: Color,
}

impl MixOutLine {
    pub fn new(from: Point, to_y: f32, additive: f32, theme: &Theme) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from.snap(), to.snap());

        let mut line = Self {
            path,
            line_color: theme.appearance().mix_out_line_color,
            calculated_color: Color::TRANSPARENT,
        };

        line.update(additive, theme);

        line
    }

    pub fn update(&mut self, additive: f32, theme: &Theme) {
        self.calculated_color = self.calculate_color(additive, theme);
    }

    fn calculate_color(&self, additive: f32, theme: &Theme) -> Color {
        let bg = theme.appearance().background_color;
        let c = theme.appearance().line_max_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            // Srgba::new(0.23, 0.69, 0.06, 1.0).into_linear(),
            Srgba::new(
                self.line_color.r,
                self.line_color.g,
                self.line_color.b,
                self.line_color.a,
            )
            .into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(additive as f32);
        let color = Srgba::from_linear(color);

        Color::new(color.red, color.green, color.blue, color.alpha)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let stroke = Stroke::default()
            .with_width(3.0)
            .with_color(self.calculated_color);

        frame.stroke(&self.path, stroke);
    }
}
