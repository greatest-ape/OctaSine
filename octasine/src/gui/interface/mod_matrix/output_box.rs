use iced_baseview::widget::canvas::{Frame, Path, Stroke};
use iced_baseview::{Point, Size};

use crate::gui::interface::style::Theme;
use crate::gui::interface::SnapPoint;

use super::{common::*, StyleSheet, OPERATOR_BOX_SCALE};

pub struct OutputBox {
    path: Path,
    pub y: f32,
}

impl OutputBox {
    pub fn new(bounds: Size) -> Self {
        let (base_top_left, base_size) = get_box_base_point_and_size(bounds, 0, 7);

        let height = base_size.height * OPERATOR_BOX_SCALE;
        let width = base_size.width * 6.0 + base_size.width * OPERATOR_BOX_SCALE;

        let left = Point {
            x: base_top_left.x - (OPERATOR_BOX_SCALE - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (OPERATOR_BOX_SCALE - 1.0) * base_size.height / 2.0 + height,
        };
        let right = Point {
            x: left.x + width,
            y: left.y,
        };

        let mut left = scale_point(bounds, left);
        let mut right = scale_point(bounds, right);

        // left.x += 1.0;
        // right.x += 1.0;

        left = left.snap();
        right = right.snap();

        let path = Path::line(left, right);

        Self { path, y: left.y }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let stroke = Stroke::default()
            .with_color(theme.appearance().box_border_color)
            .with_width(1.0);

        frame.stroke(&self.path, stroke);
    }
}
