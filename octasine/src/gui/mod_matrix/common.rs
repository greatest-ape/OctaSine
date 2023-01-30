use iced_baseview::{Point, Size, Vector};

use super::SCALE;

#[derive(Default)]
pub enum BoxStatus {
    #[default]
    Normal,
    Hover,
    Dragging {
        from: Point,
        original_value: f32,
    },
}

impl BoxStatus {
    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging { .. })
    }
}

pub fn get_box_base_point_and_size(bounds: Size, x: usize, y: usize) -> (Point, Size) {
    let x_bla = bounds.width / 7.0;
    let y_bla = bounds.height / 8.0;

    let base_top_left = Point::new(x as f32 * x_bla, y as f32 * y_bla);

    let base_size = Size::new(x_bla, y_bla);

    (base_top_left, base_size)
}

pub fn scale_point(bounds: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - SCALE) * bounds.width / 2.0,
        y: (1.0 - SCALE) * bounds.height / 2.0,
    };

    let scaled = Point {
        x: point.x * SCALE,
        y: point.y * SCALE,
    };

    scaled + translation
}

pub fn scale_size(size: Size) -> Size {
    Size::new(size.width * SCALE, size.height * SCALE)
}
