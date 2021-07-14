use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size, Vector};

use crate::approximations::Log10Table;

use crate::constants::{ENVELOPE_MAX_DURATION, ENVELOPE_MIN_DURATION};
use crate::voices::envelopes::VoiceOperatorVolumeEnvelope;
use crate::GuiSyncHandle;

use super::style::Theme;
use super::{Message, SnapPoint, FONT_SIZE, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT * 19;
const HEIGHT: u16 = LINE_HEIGHT * 5;
const SIZE: Size = Size {
    width: WIDTH as f32,
    height: HEIGHT as f32,
};

const DRAGGER_RADIUS: f32 = 4.0;

const ENVELOPE_PATH_SCALE_X: f32 = (WIDTH as f32 - 2.0 * LINE_HEIGHT as f32) / WIDTH as f32;
const ENVELOPE_PATH_SCALE_Y: f32 = 1.0 - (1.0 / 8.0) - (1.0 / 16.0);

const TOTAL_DURATION: f32 = 3.0;
const MIN_VIEWPORT_FACTOR: f32 = 1.0 / 128.0;

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub time_marker_minor_color: Color,
    pub time_marker_color_major: Color,
    pub path_color: Color,
    pub dragger_fill_color_active: Color,
    pub dragger_fill_color_hover: Color,
    pub dragger_border_color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
}

struct EnvelopeStagePath {
    path: Path,
    end_point: Point,
}

impl EnvelopeStagePath {
    fn new(
        log10_table: &Log10Table,
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
            log10_table,
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
            log10_table,
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
            log10_table,
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
            log10_table,
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
        log10_table: &Log10Table,
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
            log10_table,
            start_value as f64,
            stage_end_value as f64,
            duration as f64,
            stage_duration as f64,
        ) as f32;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum EnvelopeDraggerStatus {
    Normal,
    Hover,
    Dragging {
        from: Point,
        original_duration: f32,
        original_end_value: f32,
    },
}

struct EnvelopeDragger {
    center: Point,
    radius: f32,
    pub hitbox: Rectangle,
    pub status: EnvelopeDraggerStatus,
}

impl EnvelopeDragger {
    fn set_center(&mut self, center: Point) {
        self.center = center;

        self.hitbox.width = self.radius * 2.0;
        self.hitbox.height = self.radius * 2.0;
        self.hitbox.x = (center.x - self.radius / 2.0).max(0.0);
        self.hitbox.y = (center.y - self.radius / 2.0).max(0.0);
    }

    fn is_dragging(&self) -> bool {
        matches!(self.status, EnvelopeDraggerStatus::Dragging { .. })
    }

    fn set_to_normal_if_in_hover_state(&mut self) {
        if let EnvelopeDraggerStatus::Hover = self.status {
            self.status = EnvelopeDraggerStatus::Normal;
        }
    }
}

impl Default for EnvelopeDragger {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: DRAGGER_RADIUS,
            hitbox: Rectangle::default(),
            status: EnvelopeDraggerStatus::Normal,
        }
    }
}

pub struct Envelope {
    log10_table: Log10Table,
    cache: Cache,
    style: Theme,
    operator_index: usize,
    attack_duration: f32,
    attack_end_value: f32,
    decay_duration: f32,
    decay_end_value: f32,
    release_duration: f32,
    size: Size,
    viewport_factor: f32,
    x_offset: f32,
    attack_stage_path: EnvelopeStagePath,
    decay_stage_path: EnvelopeStagePath,
    release_stage_path: EnvelopeStagePath,
    attack_dragger: EnvelopeDragger,
    decay_dragger: EnvelopeDragger,
    release_dragger: EnvelopeDragger,
    last_cursor_position: Point,
    dragging_background_from: Option<(Point, f32)>,
}

impl Envelope {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let (attack_dur, attack_val, decay_dur, decay_val, release_dur) = match operator_index {
            0 => (10, 11, 12, 13, 14),
            1 => (24, 25, 26, 27, 28),
            2 => (39, 40, 41, 42, 43),
            3 => (54, 55, 56, 57, 58),
            _ => unreachable!(),
        };

        let attack_duration =
            Self::process_envelope_duration(sync_handle.get_parameter(attack_dur));
        let decay_duration = Self::process_envelope_duration(sync_handle.get_parameter(decay_dur));
        let release_duration =
            Self::process_envelope_duration(sync_handle.get_parameter(release_dur));

        let mut envelope = Self {
            log10_table: Log10Table::default(),
            cache: Cache::default(),
            style,
            operator_index,
            attack_duration,
            attack_end_value: sync_handle.get_parameter(attack_val) as f32,
            decay_duration,
            decay_end_value: sync_handle.get_parameter(decay_val) as f32,
            release_duration,
            size: SIZE,
            viewport_factor: 1.0,
            x_offset: 0.0,
            attack_stage_path: EnvelopeStagePath::default(),
            decay_stage_path: EnvelopeStagePath::default(),
            release_stage_path: EnvelopeStagePath::default(),
            attack_dragger: EnvelopeDragger::default(),
            decay_dragger: EnvelopeDragger::default(),
            release_dragger: EnvelopeDragger::default(),
            last_cursor_position: Point::new(-1.0, -1.0),
            dragging_background_from: None,
        };

        envelope.zoom_in_to_fit();
        envelope.update_data();

        envelope
    }

    fn zoom_in_to_fit(&mut self) {
        let duration_ratio = self.get_current_duration() / TOTAL_DURATION;

        loop {
            let next_viewport_factor = self.viewport_factor / 2.0;

            if duration_ratio > next_viewport_factor || next_viewport_factor <= MIN_VIEWPORT_FACTOR
            {
                break;
            }

            self.viewport_factor = next_viewport_factor;
        }
    }

    pub fn get_viewport_factor(&self) -> f32 {
        self.viewport_factor
    }

    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }

    pub fn set_viewport(&mut self, viewport_factor: f32, x_offset: f32) {
        self.viewport_factor = viewport_factor;
        self.x_offset = Self::process_x_offset(x_offset, viewport_factor);

        self.update_data();
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    fn process_x_offset(x_offset: f32, viewport_factor: f32) -> f32 {
        x_offset.min(0.0).max(-1.0 + viewport_factor)
    }

    fn process_envelope_duration(sync_value: f64) -> f32 {
        sync_value.max(ENVELOPE_MIN_DURATION / ENVELOPE_MAX_DURATION) as f32
    }

    fn get_current_duration(&self) -> f32 {
        self.attack_duration + self.decay_duration + self.release_duration
    }

    pub fn zoom_in(&mut self) {
        if self.viewport_factor > MIN_VIEWPORT_FACTOR {
            self.viewport_factor = (self.viewport_factor * 0.5).max(MIN_VIEWPORT_FACTOR);

            let duration = self.get_current_duration();

            // Zoom towards center of viewport unless envelope is really short
            // compared to it (in which case, implicitly zoom towards the left)
            if duration / TOTAL_DURATION >= self.viewport_factor {
                self.x_offset -= self.viewport_factor / 2.0;
            }

            self.x_offset = Self::process_x_offset(self.x_offset, self.viewport_factor);
        }

        self.update_data();
    }

    pub fn zoom_out(&mut self) {
        if self.viewport_factor < 1.0 {
            self.x_offset += self.viewport_factor / 2.0;

            self.viewport_factor = (self.viewport_factor * 2.0).min(1.0);

            self.x_offset = Self::process_x_offset(self.x_offset, self.viewport_factor);
        }

        self.update_data();
    }

    pub fn zoom_to_fit(&mut self) {
        self.viewport_factor = 1.0;
        self.x_offset = 0.0;

        self.zoom_in_to_fit();
        self.update_data();
    }

    pub fn set_attack_duration(&mut self, value: f64) {
        if !self.attack_dragger.is_dragging() {
            self.attack_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    pub fn set_attack_end_value(&mut self, value: f64) {
        self.attack_end_value = value as f32;

        self.update_data();
    }

    pub fn set_decay_duration(&mut self, value: f64) {
        if !self.decay_dragger.is_dragging() {
            self.decay_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    pub fn set_decay_end_value(&mut self, value: f64) {
        self.decay_end_value = value as f32;

        self.update_data();
    }

    pub fn set_release_duration(&mut self, value: f64) {
        if !self.release_dragger.is_dragging() {
            self.release_duration = Self::process_envelope_duration(value);

            self.update_data();
        }
    }

    fn update_data(&mut self) {
        self.update_stage_paths();

        self.attack_dragger
            .set_center(self.attack_stage_path.end_point);
        self.decay_dragger
            .set_center(self.decay_stage_path.end_point);
        self.release_dragger
            .set_center(self.release_stage_path.end_point);

        self.cache.clear();
    }

    fn update_stage_paths(&mut self) {
        let total_duration = self.viewport_factor * TOTAL_DURATION;
        let x_offset = self.x_offset / self.viewport_factor;

        self.attack_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            x_offset,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        self.decay_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        self.release_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            self.release_duration as f32,
            0.0,
        );
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_time_markers(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

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

    fn draw_stage_paths(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();
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
            .with_color(style.time_marker_color_major);

        frame.stroke(&top_drag_border, drag_border_stroke);
        frame.stroke(&bottom_drag_border, drag_border_stroke);

        let stage_path_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.path_color);

        frame.stroke(&self.attack_stage_path.path, stage_path_stroke);
        frame.stroke(&self.decay_stage_path.path, stage_path_stroke);
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

        frame.stroke(&top_border, border_stroke);
        frame.stroke(&bottom_border, border_stroke);
        frame.stroke(&left_border, border_stroke);
        frame.stroke(&right_border, border_stroke);
    }

    fn draw_dragger(
        frame: &mut Frame,
        style_sheet: Box<dyn StyleSheet>,
        dragger: &EnvelopeDragger,
    ) {
        let size = frame.size();
        let style = style_sheet.active();

        let left_end_x = scale_point(size, Point::ORIGIN).snap().x;
        let right_end_x = scale_point(size, Point::new(size.width, 0.0)).snap().x;

        if dragger.center.x < left_end_x || dragger.center.x > right_end_x {
            return;
        }

        let circle_path = {
            let mut builder = path::Builder::new();

            builder.move_to(dragger.center);
            builder.circle(dragger.center, dragger.radius);

            builder.build()
        };

        let fill_color = match dragger.status {
            EnvelopeDraggerStatus::Normal => style_sheet.active().dragger_fill_color_active,
            EnvelopeDraggerStatus::Hover => style_sheet.active().dragger_fill_color_hover,
            EnvelopeDraggerStatus::Dragging { .. } => style_sheet.active().dragger_fill_color_hover,
        };

        frame.fill(&circle_path, fill_color);

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.dragger_border_color);

        frame.stroke(&circle_path, stroke);
    }
}

impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_time_markers(frame, self.style.into());
            self.draw_stage_paths(frame, self.style.into());

            Self::draw_dragger(frame, self.style.into(), &self.attack_dragger);
            Self::draw_dragger(frame, self.style.into(), &self.decay_dragger);
            Self::draw_dragger(frame, self.style.into(), &self.release_dragger);
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                self.last_cursor_position = Point::new(x, y);

                let relative_position = Point::new(x - bounds.x, y - bounds.y);

                let attack_hitbox_hit = self.attack_dragger.hitbox.contains(relative_position);

                match self.attack_dragger.status {
                    EnvelopeDraggerStatus::Normal => {
                        if attack_hitbox_hit {
                            self.attack_dragger.status = EnvelopeDraggerStatus::Hover;

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Hover => {
                        if !attack_hitbox_hit {
                            self.attack_dragger.status = EnvelopeDraggerStatus::Normal;

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Dragging {
                        from,
                        original_duration,
                        original_end_value,
                    } => {
                        self.attack_duration =
                            dragging_to_duration(self.viewport_factor, x, from, original_duration);
                        self.attack_end_value = dragging_to_end_value(y, from, original_end_value);

                        self.update_data();

                        let (dur, val) = match self.operator_index {
                            0 => (10, 11),
                            1 => (24, 25),
                            2 => (39, 40),
                            3 => (54, 55),
                            _ => unreachable!(),
                        };

                        let changes = vec![
                            (dur, self.attack_duration as f64),
                            (val, self.attack_end_value as f64),
                        ];

                        return (
                            event::Status::Captured,
                            Some(Message::ParameterChanges(changes)),
                        );
                    }
                }

                let decay_hitbox_hit = self.decay_dragger.hitbox.contains(relative_position);

                if decay_hitbox_hit {
                    self.attack_dragger.set_to_normal_if_in_hover_state();
                }

                match self.decay_dragger.status {
                    EnvelopeDraggerStatus::Normal => {
                        if decay_hitbox_hit {
                            self.decay_dragger.status = EnvelopeDraggerStatus::Hover;

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Hover => {
                        if !decay_hitbox_hit {
                            self.decay_dragger.status = EnvelopeDraggerStatus::Normal;

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Dragging {
                        from,
                        original_duration,
                        original_end_value,
                    } => {
                        self.decay_duration =
                            dragging_to_duration(self.viewport_factor, x, from, original_duration);
                        self.decay_end_value = dragging_to_end_value(y, from, original_end_value);

                        self.update_data();

                        let (dur, val) = match self.operator_index {
                            0 => (12, 13),
                            1 => (26, 27),
                            2 => (41, 42),
                            3 => (56, 57),
                            _ => unreachable!(),
                        };

                        let changes = vec![
                            (dur, self.decay_duration as f64),
                            (val, self.decay_end_value as f64),
                        ];

                        return (
                            event::Status::Captured,
                            Some(Message::ParameterChanges(changes)),
                        );
                    }
                }

                let release_hitbox_hit = self.release_dragger.hitbox.contains(relative_position);

                if release_hitbox_hit {
                    self.attack_dragger.set_to_normal_if_in_hover_state();
                    self.decay_dragger.set_to_normal_if_in_hover_state();
                }

                match self.release_dragger.status {
                    EnvelopeDraggerStatus::Normal => {
                        if release_hitbox_hit {
                            self.release_dragger.status = EnvelopeDraggerStatus::Hover;

                            self.attack_dragger.set_to_normal_if_in_hover_state();
                            self.decay_dragger.set_to_normal_if_in_hover_state();

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Hover => {
                        if !release_hitbox_hit {
                            self.release_dragger.status = EnvelopeDraggerStatus::Normal;

                            self.cache.clear();
                        }
                    }
                    EnvelopeDraggerStatus::Dragging {
                        from,
                        original_duration,
                        ..
                    } => {
                        self.release_duration =
                            dragging_to_duration(self.viewport_factor, x, from, original_duration);

                        self.update_data();

                        let parameter_index = match self.operator_index {
                            0 => 14,
                            1 => 28,
                            2 => 43,
                            3 => 58,
                            _ => unreachable!(),
                        };

                        return (
                            event::Status::Captured,
                            Some(Message::ParameterChange(
                                parameter_index,
                                self.release_duration as f64,
                            )),
                        );
                    }
                }

                if let Some((from, original_offset)) = self.dragging_background_from {
                    let change = (x - from.x) / WIDTH as f32 * self.viewport_factor;

                    self.x_offset =
                        Self::process_x_offset(original_offset + change, self.viewport_factor);

                    self.update_data();
                }

                if bounds.contains(Point::new(x, y)) {
                    return (event::Status::Captured, None);
                }
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(
                iced_baseview::mouse::Button::Left,
            )) => {
                if bounds.contains(self.last_cursor_position) {
                    let relative_position = Point::new(
                        self.last_cursor_position.x - bounds.x,
                        self.last_cursor_position.y - bounds.y,
                    );

                    if self.release_dragger.hitbox.contains(relative_position)
                        && !self.release_dragger.is_dragging()
                    {
                        self.release_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.release_duration,
                            original_end_value: 0.0,
                        };
                    } else if self.decay_dragger.hitbox.contains(relative_position)
                        && !self.decay_dragger.is_dragging()
                    {
                        self.decay_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.decay_duration,
                            original_end_value: self.decay_end_value,
                        };
                    } else if self.attack_dragger.hitbox.contains(relative_position)
                        && !self.attack_dragger.is_dragging()
                    {
                        self.attack_dragger.status = EnvelopeDraggerStatus::Dragging {
                            from: self.last_cursor_position,
                            original_duration: self.attack_duration,
                            original_end_value: self.attack_end_value,
                        };
                    } else {
                        self.dragging_background_from =
                            Some((self.last_cursor_position, self.x_offset));
                    }

                    self.cache.clear();

                    return (event::Status::Captured, None);
                }
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(
                iced_baseview::mouse::Button::Left,
            )) => {
                let mut captured = false;

                if self.release_dragger.is_dragging() {
                    self.release_dragger.status = EnvelopeDraggerStatus::Normal;

                    captured = true;
                }
                if self.decay_dragger.is_dragging() {
                    self.decay_dragger.status = EnvelopeDraggerStatus::Normal;

                    captured = true;
                }
                if self.attack_dragger.is_dragging() {
                    self.attack_dragger.status = EnvelopeDraggerStatus::Normal;

                    captured = true;
                }

                if self.dragging_background_from.is_some() {
                    self.dragging_background_from = None;

                    captured = true;
                }

                if captured {
                    self.cache.clear();

                    return (event::Status::Captured, None);
                }
            }
            _ => (),
        };

        (event::Status::Ignored, None)
    }
}

fn scale_point(size: Size, point: Point) -> Point {
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

// Almost-correct reverse transformation for envelope dragger to duration
fn dragging_to_duration(
    viewport_factor: f32,
    cursor_x: f32,
    from: Point,
    original_value: f32,
) -> f32 {
    let change = (cursor_x - from.x) / WIDTH as f32;
    let change = change / ENVELOPE_PATH_SCALE_X;
    let change = change * viewport_factor * TOTAL_DURATION;

    (original_value + change)
        .min(1.0)
        .max(ENVELOPE_MIN_DURATION as f32 / ENVELOPE_MAX_DURATION as f32)
}

fn dragging_to_end_value(cursor_y: f32, from: Point, original_value: f32) -> f32 {
    let change = -(cursor_y - from.y) / HEIGHT as f32;
    let change = change / ENVELOPE_PATH_SCALE_Y;

    (original_value + change).min(1.0).max(0.0)
}
