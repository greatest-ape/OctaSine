use iced_baseview::core::{Color, Point, Size, Vector};
use iced_baseview::widget::canvas::{path, Frame, Path, Stroke};

use crate::audio::voices::envelopes::VoiceOperatorVolumeEnvelope;
use crate::audio::voices::log10_table::Log10Table;
use crate::gui::style::Theme;
use crate::gui::{SnapPoint, LINE_HEIGHT};

pub const WIDTH: u16 = LINE_HEIGHT * 20;
pub const HEIGHT: u16 = LINE_HEIGHT * 5;

pub const SIZE: Size = Size {
    width: WIDTH as f32,
    height: HEIGHT as f32,
};

pub const DRAGGER_RADIUS: f32 = 4.0;

pub const ENVELOPE_PATH_SCALE_X: f32 = (WIDTH as f32 - 2.0 * LINE_HEIGHT as f32) / WIDTH as f32;
pub const ENVELOPE_PATH_SCALE_Y: f32 = 1.0 - (1.0 / 8.0) - (1.0 / 16.0);

pub const TOTAL_DURATION: f32 = 3.0;
pub const MIN_VIEWPORT_FACTOR: f32 = 1.0 / 64.0;

pub const FIXED_VIEWPORT_FACTORS: &[f32] = &[
    1.0,
    1.0 / 2.0,
    1.0 / 4.0,
    1.0 / 8.0,
    1.0 / 16.0,
    1.0 / 32.0,
    1.0 / 64.0,
];

#[derive(Debug, Clone)]
pub struct Appearance {
    pub background_color: Color,
    pub border_color: Color,
    pub drag_border_color: Color,
    pub text_color: Color,
    pub time_marker_minor_color: Color,
    pub time_marker_color_major: Color,
    pub path_color: Color,
    pub dragger_fill_color_active: Color,
    pub dragger_fill_color_hover: Color,
    pub dragger_fill_color_dragging: Color,
    pub dragger_border_color: Color,
    pub viewport_indicator_border: Color,
    pub viewport_indicator_border_active: Color,
}

pub trait StyleSheet {
    fn appearance(&self) -> Appearance;
}

pub struct EnvelopeStagePath {
    pub path: Path,
    pub end_point: Point,
}

impl EnvelopeStagePath {
    pub fn new(
        log10table: &Log10Table,
        size: Size,
        total_duration: f32,
        x_offset: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
    ) -> Self {
        let mut path = path::Builder::new();

        let start = Self::calculate_stage_progress_point(
            log10table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            0.0,
        );
        let control_a = Self::calculate_stage_progress_point(
            log10table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0 / 3.0,
        );
        let control_b = Self::calculate_stage_progress_point(
            log10table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            2.0 / 3.0,
        );
        let to = Self::calculate_stage_progress_point(
            log10table,
            size,
            total_duration,
            x_offset,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0,
        );

        path.move_to(start);
        path.bezier_curve_to(control_a, control_b, to);

        Self {
            path: path.build(),
            end_point: to,
        }
    }

    fn calculate_stage_progress_point(
        log10table: &Log10Table,
        size: Size,
        total_duration: f32,
        x_offset: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
        progress: f32,
    ) -> Point {
        let duration = stage_duration * progress;

        let value = VoiceOperatorVolumeEnvelope::calculate_curve(
            log10table,
            start_value,
            stage_end_value,
            duration as f64,
            stage_duration as f64,
        );

        // Watch out for point.y.is_nan() when duration = 0.0 here
        let point = Point::new(
            (x_offset + (start_duration + duration) / total_duration) * size.width,
            size.height * (1.0 - value),
        );

        scale_point(size, point).snap()
    }
}

impl Default for EnvelopeStagePath {
    fn default() -> Self {
        Self {
            path: Path::line(Point::default(), Point::default()),
            end_point: Point::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum EnvelopeDraggerStatus {
    #[default]
    Normal,
    Hover,
    Dragging {
        from: Point,
        original_duration: f32,
        original_end_value: f32,
    },
}

impl EnvelopeDraggerStatus {
    pub fn is_dragging(&self) -> bool {
        matches!(self, Self::Dragging { .. })
    }

    pub fn set_to_normal_if_in_hover_state(&mut self) {
        if let Self::Hover = self {
            *self = Self::Normal;
        }
    }
}

pub struct EnvelopeDragger {
    center: Point,
    radius: f32,
}

impl EnvelopeDragger {
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme, status: &EnvelopeDraggerStatus) {
        let size = frame.size();
        let appearance = theme.appearance();

        let left_end_x = scale_point(size, Point::ORIGIN).snap().x;
        let right_end_x = scale_point(size, Point::new(size.width, 0.0)).snap().x;

        if self.center.x < left_end_x || self.center.x > right_end_x {
            return;
        }

        let circle_path = {
            let mut builder = path::Builder::new();

            builder.move_to(self.center);
            builder.circle(self.center, self.radius);

            builder.build()
        };

        let fill_color = match status {
            EnvelopeDraggerStatus::Normal => appearance.dragger_fill_color_active,
            EnvelopeDraggerStatus::Hover => appearance.dragger_fill_color_hover,
            EnvelopeDraggerStatus::Dragging { .. } => appearance.dragger_fill_color_dragging,
        };

        frame.fill(&circle_path, fill_color);

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(appearance.dragger_border_color);

        frame.stroke(&circle_path, stroke);
    }

    pub fn cursor_overlaps(&self, cursor_position: Point) -> bool {
        let diff = cursor_position - self.center;

        (diff.x.abs() <= self.radius) & (diff.y.abs() <= self.radius)
    }
}

impl Default for EnvelopeDragger {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: DRAGGER_RADIUS,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DraggingBackground {
    pub from_point: Point,
    pub original_visible_position: f32,
    pub original_x_offset: f32,
    pub viewport_factor: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DoubleClickData {
    pub point: Point,
    pub releases: usize,
}

#[derive(Default)]
pub struct EnvelopeCanvasState {
    pub last_cursor_position: Point,
    pub dragging_background_from: Option<DraggingBackground>,
    pub double_click_data: Option<DoubleClickData>,
    pub attack_dragger_status: EnvelopeDraggerStatus,
    pub decay_dragger_status: EnvelopeDraggerStatus,
    pub release_dragger_status: EnvelopeDraggerStatus,
}

pub fn scale_point(size: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - ENVELOPE_PATH_SCALE_X) * size.width / 2.0,
        y: (1.0 - ENVELOPE_PATH_SCALE_Y) * size.height / 2.0,
    };

    let scaled = Point {
        x: point.x * ENVELOPE_PATH_SCALE_X,
        y: point.y * ENVELOPE_PATH_SCALE_Y,
    };

    scaled + translation
}
