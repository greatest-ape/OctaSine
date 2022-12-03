use iced_baseview::canvas::{Frame, Path, Stroke, Text};
use iced_baseview::{Point, Size, Vector};

use crate::gui::interface::style::Theme;
use crate::gui::interface::{SnapPoint, FONT_SIZE};

use super::common::*;
use super::EnvelopeCanvas;

/// Canvas display logic
impl EnvelopeCanvas {
    pub fn draw_time_markers(&self, frame: &mut Frame, style: Theme) {
        let font_regular = style.font_regular();
        let style = style.envelope().appearance();

        let total_duration = self.viewport_factor * TOTAL_DURATION;
        let x_offset = self.x_offset / self.viewport_factor;

        let mut time_marker_interval = 0.01 / 4.0;

        loop {
            let num_markers = (total_duration / time_marker_interval) as usize;

            if num_markers <= 110 {
                break;
            } else {
                time_marker_interval *= 10.0;
            }
        }

        let iterations = (TOTAL_DURATION / time_marker_interval) as usize + 1;

        for i in 0..iterations {
            let x =
                (x_offset + (time_marker_interval * i as f32) / total_duration) * self.size.width;

            if x < 0.0 || x > self.size.width {
                continue;
            }

            let top_point = Point::new(x, 0.0);
            let bottom_point = Point::new(x, self.size.height);

            let path = Path::line(
                scale_point_x(self.size, top_point).snap(),
                scale_point_x(self.size, bottom_point).snap(),
            );

            if i % 10 == 0 && i != 0 {
                let text_point = Point::new(x - 10.0, self.size.height);

                let text = Text {
                    content: format!("{:.1}s", time_marker_interval * 4.0 * i as f32),
                    position: scale_point_x(self.size, text_point),
                    font: font_regular,
                    size: FONT_SIZE as f32,
                    color: style.text_color,
                    ..Default::default()
                };

                frame.fill_text(text);

                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(style.time_marker_color_major);

                frame.stroke(&path, stroke);
            } else {
                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(style.time_marker_minor_color);

                frame.stroke(&path, stroke);
            }
        }
    }

    pub fn draw_stage_paths(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.appearance();
        let size = frame.size();

        let top_drag_border = Path::line(
            scale_point(size, Point::ORIGIN).snap(),
            scale_point(size, Point::new(size.width, 0.0)).snap(),
        );
        let bottom_drag_border = Path::line(
            scale_point(size, Point::new(0.0, size.height)).snap(),
            scale_point(size, Point::new(size.width, size.height)).snap(),
        );

        let drag_border_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.drag_border_color);

        frame.stroke(&top_drag_border, drag_border_stroke.clone());
        frame.stroke(&bottom_drag_border, drag_border_stroke);

        let stage_path_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.path_color);

        frame.stroke(&self.attack_stage_path.path, stage_path_stroke.clone());
        frame.stroke(&self.decay_stage_path.path, stage_path_stroke.clone());
        frame.stroke(&self.release_stage_path.path, stage_path_stroke);

        // Hide stage path parts that extend beyond scaled bounds, draw borders

        let left_bg_x = scale_point_x(size, Point::ORIGIN).snap().x - 1.0;
        let left_bg = Path::rectangle(Point::ORIGIN, Size::new(left_bg_x, size.height));
        frame.fill(&left_bg, style.background_color);
        frame.stroke(
            &left_bg,
            Stroke::default().with_color(style.background_color),
        );

        let right_bg_x = scale_point_x(size, Point::new(size.width, 0.0)).snap().x + 1.0;
        let right_bg = Path::rectangle(
            Point::new(right_bg_x, 0.0),
            Size::new(size.width, size.height),
        );
        frame.fill(&right_bg, style.background_color);
        frame.stroke(
            &right_bg,
            Stroke::default().with_color(style.background_color),
        );

        let top_border = Path::line(
            scale_point_x(size, Point::ORIGIN).snap(),
            scale_point_x(size, Point::new(size.width, 0.0)).snap(),
        );
        let bottom_border = {
            let left = scale_point_x(size, Point::new(0.0, size.height)).snap().x;
            let right = scale_point_x(size, Point::new(size.width, size.height))
                .snap()
                .x;

            Path::line(
                Point::new(left, size.height - 1.0).snap(),
                Point::new(right, size.height - 1.0).snap(),
            )
        };
        let left_border = Path::line(
            scale_point_x(size, Point::new(0.0, 0.0)).snap(),
            scale_point_x(size, Point::new(0.0, size.height)).snap(),
        );
        let right_border = Path::line(
            scale_point_x(size, Point::new(size.width, 0.0)).snap(),
            scale_point_x(size, Point::new(size.width, size.height)).snap(),
        );
        let border_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.border_color);

        frame.stroke(&top_border, border_stroke.clone());
        frame.stroke(&bottom_border, border_stroke.clone());
        frame.stroke(&left_border, border_stroke.clone());
        frame.stroke(&right_border, border_stroke);
    }

    pub fn draw_viewport_indicator(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        const WIDTH: f32 = 60.0;
        const HEIGHT: f32 = 6.0;

        let style = style_sheet.appearance();
        let size = frame.size();

        let top_right = scale_point_x(size, Point::new(size.width, 0.0)).snap();
        let top_left = Point::new(top_right.x - WIDTH, top_right.y);

        let full_rect = Path::rectangle(top_left, Size::new(WIDTH, HEIGHT));

        let border_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.viewport_indicator_border);

        frame.fill(&full_rect, style.background_color);
        frame.stroke(&full_rect, border_stroke);

        let viewport_top_left = Point::new(
            (top_left.x + -self.x_offset * WIDTH).floor() + 0.5 + 1.0,
            top_left.y + 1.0,
        );
        let viewport_rect = Path::rectangle(
            viewport_top_left,
            Size::new(
                (WIDTH * self.viewport_factor).floor().max(2.0) - 2.0,
                HEIGHT - 2.0,
            ),
        );

        let border_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.viewport_indicator_border_active);

        frame.fill(&viewport_rect, style.background_color);
        frame.stroke(&viewport_rect, border_stroke);
    }
}

fn scale_point_x(size: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - ENVELOPE_PATH_SCALE_X) * size.width / 2.0,
        y: 0.0,
    };

    let scaled = Point {
        x: point.x * ENVELOPE_PATH_SCALE_X,
        y: point.y,
    };

    scaled + translation
}
