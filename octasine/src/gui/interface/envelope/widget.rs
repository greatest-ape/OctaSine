use std::sync::atomic::{AtomicBool, Ordering};

use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Container, Element, Length, Point, Rectangle, Size, Vector};

use crate::audio::voices::envelopes::VoiceOperatorVolumeEnvelope;
use crate::audio::voices::log10_table::Log10Table;
use crate::parameters::operator_envelope::{
    OperatorAttackDurationValue, OperatorDecayDurationValue, OperatorEnvelopeGroupValue,
    OperatorReleaseDurationValue, OperatorSustainVolumeValue, ENVELOPE_MAX_DURATION,
    ENVELOPE_MIN_DURATION,
};
use crate::parameters::{OperatorParameter, Parameter, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::super::style::Theme;
use super::super::{Message, SnapPoint, FONT_SIZE, LINE_HEIGHT};

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
const MIN_VIEWPORT_FACTOR: f32 = 1.0 / 64.0;

const FIXED_VIEWPORT_FACTORS: &[f32] = &[
    1.0,
    1.0 / 2.0,
    1.0 / 4.0,
    1.0 / 8.0,
    1.0 / 16.0,
    1.0 / 32.0,
    1.0 / 64.0,
];

#[derive(Debug, Clone)]
pub struct Style {
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
    fn active(&self) -> Style;
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeValues {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub viewport_factor: f32,
    pub x_offset: f32,
}

struct EnvelopeStagePath {
    path: Path,
    end_point: Point,
}

impl EnvelopeStagePath {
    fn new(
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

    fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let size = frame.size();
        let style = style_sheet.active();

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

        let fill_color = match self.status {
            EnvelopeDraggerStatus::Normal => style_sheet.active().dragger_fill_color_active,
            EnvelopeDraggerStatus::Hover => style_sheet.active().dragger_fill_color_hover,
            EnvelopeDraggerStatus::Dragging { .. } => {
                style_sheet.active().dragger_fill_color_dragging
            }
        };

        frame.fill(&circle_path, fill_color);

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(style.dragger_border_color);

        frame.stroke(&circle_path, stroke);
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

#[derive(Clone, Copy)]
struct DraggingBackground {
    from_point: Point,
    original_visible_position: f32,
    original_x_offset: f32,
    viewport_factor: f32,
}

#[derive(Clone, Copy, Debug)]
struct DoubleClickData {
    point: Point,
    releases: usize,
}

#[derive(Default)]
pub struct CanvasState {
    attack_stage_path: EnvelopeStagePath,
    decay_stage_path: EnvelopeStagePath,
    release_stage_path: EnvelopeStagePath,
    attack_dragger: EnvelopeDragger,
    decay_dragger: EnvelopeDragger,
    release_dragger: EnvelopeDragger,
    last_cursor_position: Point,
    dragging_background_from: Option<DraggingBackground>,
    double_click_data: Option<DoubleClickData>,
}

impl CanvasState {
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
}

pub struct Envelope {
    log10table: Log10Table,
    cache: Cache,
    style: Theme,
    operator_index: u8,
    attack_duration: f32,
    decay_duration: f32,
    sustain_volume: f32,
    release_duration: f32,
    group: OperatorEnvelopeGroupValue,
    modified_by_automation: bool,
    size: Size,
    viewport_factor: f32,
    x_offset: f32,
    updates_available: AtomicBool,
}

impl Envelope {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let operator_index = operator_index as u8;

        let attack_duration =
            OperatorAttackDurationValue::new_from_patch(sync_handle.get_parameter(
                Parameter::Operator(operator_index, OperatorParameter::AttackDuration),
            ))
            .to_patch();
        let decay_duration = OperatorDecayDurationValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::DecayDuration),
        ))
        .to_patch();
        let release_duration =
            OperatorReleaseDurationValue::new_from_patch(sync_handle.get_parameter(
                Parameter::Operator(operator_index, OperatorParameter::ReleaseDuration),
            ))
            .to_patch();
        let sustain_volume = OperatorSustainVolumeValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::SustainVolume),
        ))
        .to_patch();
        let group = OperatorEnvelopeGroupValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::EnvelopeLockGroup),
        ));

        let mut envelope = Self {
            log10table: Default::default(),
            cache: Cache::default(),
            style,
            operator_index,
            attack_duration,
            decay_duration,
            sustain_volume,
            release_duration,
            group,
            modified_by_automation: true,
            size: SIZE,
            viewport_factor: 1.0,
            x_offset: 0.0,
            updates_available: AtomicBool::new(true),
        };

        let (viewport_factor, x_offset) = envelope.get_zoom_to_fit_data();

        envelope.set_viewport(viewport_factor, x_offset);

        envelope
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(
            Canvas::new(self)
                .width(Length::Units(WIDTH))
                .height(Length::Units(HEIGHT)),
        )
        .height(Length::Units(LINE_HEIGHT * 6))
        .into()
    }
}

/// Public style / viewport / parameter value setters
impl Envelope {
    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    pub fn set_viewport(&mut self, viewport_factor: f32, x_offset: f32) {
        self.viewport_factor = viewport_factor;
        self.x_offset = Self::process_x_offset(x_offset, viewport_factor);

        self.update_data();
    }

    pub fn set_attack_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorAttackDurationValue::new_from_patch(value).to_patch();

        if value != self.attack_duration {
            self.attack_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_decay_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorDecayDurationValue::new_from_patch(value).to_patch();

        if value != self.decay_duration {
            self.decay_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_sustain_volume(&mut self, value: f32, internal: bool) {
        let value = OperatorSustainVolumeValue::new_from_patch(value).to_patch();

        if value != self.sustain_volume {
            self.sustain_volume = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_release_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorReleaseDurationValue::new_from_patch(value).to_patch();

        if value != self.release_duration {
            self.release_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_group(&mut self, group: OperatorEnvelopeGroupValue, internal: bool) {
        if group != self.group {
            self.group = group;

            self.modified_by_automation = !internal;
        }
    }
}

/// Public value getters
impl Envelope {
    pub fn get_envelope_values(&self) -> EnvelopeValues {
        EnvelopeValues {
            attack: self.attack_duration,
            decay: self.decay_duration,
            sustain: self.sustain_volume,
            release: self.release_duration,
            viewport_factor: self.viewport_factor,
            x_offset: self.x_offset,
        }
    }
    pub fn get_viewport_factor(&self) -> f32 {
        self.viewport_factor
    }
    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }
    pub fn get_modified_by_automation(&self) -> bool {
        self.modified_by_automation
    }
}

/// Viewport change helpers
impl Envelope {
    pub(super) fn get_zoom_in_data(&self) -> (f32, f32) {
        for factor in FIXED_VIEWPORT_FACTORS.iter().copied() {
            if factor < self.viewport_factor {
                let mut new_x_offset = self.x_offset;

                // Zoom towards center of viewport unless envelope is takes up
                // less than half of it (in which case, implicitly zoom towards
                // the left)
                if self.get_current_duration() / TOTAL_DURATION > self.viewport_factor * 0.5 {
                    new_x_offset -= (self.viewport_factor - factor) / 2.0;
                }

                let new_viewport_factor = factor;
                let new_x_offset = Self::process_x_offset(new_x_offset, new_viewport_factor);

                return (new_viewport_factor, new_x_offset);
            }
        }

        (self.viewport_factor, self.x_offset)
    }

    pub(super) fn get_zoom_out_data(&self) -> (f32, f32) {
        for factor in FIXED_VIEWPORT_FACTORS.iter().rev().copied() {
            if factor > self.viewport_factor {
                let new_x_offset = self.x_offset + (factor - self.viewport_factor) / 2.0;

                let new_viewport_factor = factor;
                let new_x_offset = Self::process_x_offset(new_x_offset, new_viewport_factor);

                return (new_viewport_factor, new_x_offset);
            }
        }

        (self.viewport_factor, self.x_offset)
    }

    pub(super) fn get_zoom_to_fit_data(&self) -> (f32, f32) {
        let duration_ratio = self.get_current_duration() / TOTAL_DURATION;

        let mut new_viewport_factor = 1.0;

        for factor in FIXED_VIEWPORT_FACTORS.iter().copied() {
            if duration_ratio > factor {
                break;
            }

            new_viewport_factor = factor;
        }

        let new_x_offset = Self::process_x_offset(0.0, new_viewport_factor);

        (new_viewport_factor, new_x_offset)
    }

    fn update_data(&mut self) {
        self.updates_available.store(true, Ordering::SeqCst);

        self.cache.clear();
    }
}

/// Internal data update helpers
impl Envelope {
    fn update_stage_paths(&self, state: &mut CanvasState) {
        let total_duration = self.viewport_factor * TOTAL_DURATION;
        let x_offset = self.x_offset / self.viewport_factor;

        state.attack_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            0.0,
            0.0,
            self.attack_duration as f32,
            1.0,
        );

        state.decay_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration,
            1.0,
            self.decay_duration as f32,
            self.sustain_volume as f32,
        );

        state.release_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration + self.decay_duration,
            self.sustain_volume,
            self.release_duration as f32,
            0.0,
        );
    }
}

/// Utilities
impl Envelope {
    fn process_x_offset(x_offset: f32, viewport_factor: f32) -> f32 {
        x_offset.min(0.0).max(-1.0 + viewport_factor)
    }

    fn get_current_duration(&self) -> f32 {
        self.attack_duration + self.decay_duration + self.release_duration
    }
}

/// Event handlers
impl Envelope {
    fn handle_button_pressed(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
    ) -> (event::Status, Option<Message>) {
        if bounds.contains(state.last_cursor_position) {
            let relative_position = Point::new(
                state.last_cursor_position.x - bounds.x,
                state.last_cursor_position.y - bounds.y,
            );

            if state.release_dragger.hitbox.contains(relative_position)
                && !state.release_dragger.is_dragging()
            {
                state.release_dragger.status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.release_duration,
                    original_end_value: 0.0,
                };
            } else if state.decay_dragger.hitbox.contains(relative_position)
                && !state.decay_dragger.is_dragging()
            {
                state.decay_dragger.status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.decay_duration,
                    original_end_value: self.sustain_volume,
                };
            } else if state.attack_dragger.hitbox.contains(relative_position)
                && !state.attack_dragger.is_dragging()
            {
                state.attack_dragger.status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.attack_duration,
                    original_end_value: 1.0,
                };
            } else {
                let pos_in_bounds = state.last_cursor_position.x - bounds.x;
                let pos_in_viewport =
                    (pos_in_bounds - (WIDTH as f32 * (1.0 - ENVELOPE_PATH_SCALE_X)) / 2.0).max(0.0);
                let pos_in_viewport =
                    (pos_in_viewport / (WIDTH as f32 * ENVELOPE_PATH_SCALE_X)).min(1.0);

                state.dragging_background_from = Some(DraggingBackground {
                    from_point: state.last_cursor_position,
                    original_visible_position: pos_in_viewport,
                    original_x_offset: self.x_offset,
                    viewport_factor: self.viewport_factor,
                });

                if state.double_click_data.is_none() {
                    state.double_click_data = Some(DoubleClickData {
                        point: state.last_cursor_position,
                        releases: 0,
                    });
                }
            }

            self.cache.clear();

            (event::Status::Captured, None)
        } else {
            (event::Status::Ignored, None)
        }
    }

    /// Handle cursor moved event
    ///
    /// Updates display state and ADSR parameter values on self
    fn handle_cursor_moved(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        x: f32,
        y: f32,
    ) -> (event::Status, Option<Message>) {
        state.last_cursor_position = Point::new(x, y);

        if let Some(data) = state.double_click_data {
            if data.point != state.last_cursor_position {
                state.double_click_data = None;
            }
        }

        let relative_position = Point::new(x - bounds.x, y - bounds.y);

        let attack_hitbox_hit = state.attack_dragger.hitbox.contains(relative_position);

        match state.attack_dragger.status {
            EnvelopeDraggerStatus::Normal => {
                if attack_hitbox_hit {
                    state.attack_dragger.status = EnvelopeDraggerStatus::Hover;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !attack_hitbox_hit {
                    state.attack_dragger.status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                ..
            } => {
                let attack_duration =
                    dragging_to_duration(self.viewport_factor, x, from, original_duration);

                // FIXME
                // self.modified_by_automation = false;
                // self.update_data();

                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(self.operator_index, OperatorParameter::AttackDuration),
                        attack_duration as f32,
                    ),
                    parameter_2: None,
                };

                return (event::Status::Captured, Some(message));
            }
        }

        let decay_hitbox_hit = state.decay_dragger.hitbox.contains(relative_position);

        if decay_hitbox_hit {
            state.attack_dragger.set_to_normal_if_in_hover_state();
        }

        match state.decay_dragger.status {
            EnvelopeDraggerStatus::Normal => {
                if decay_hitbox_hit {
                    state.decay_dragger.status = EnvelopeDraggerStatus::Hover;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !decay_hitbox_hit {
                    state.decay_dragger.status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                original_end_value,
            } => {
                let decay_duration =
                    dragging_to_duration(self.viewport_factor, x, from, original_duration);
                let sustain_volume = dragging_to_end_value(y, from, original_end_value);

                // FIXME
                // self.modified_by_automation = false;
                // self.update_data();

                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(self.operator_index, OperatorParameter::DecayDuration),
                        decay_duration as f32,
                    ),
                    parameter_2: Some((
                        Parameter::Operator(self.operator_index, OperatorParameter::SustainVolume),
                        sustain_volume as f32,
                    )),
                };

                return (event::Status::Captured, Some(message));
            }
        }

        let release_hitbox_hit = state.release_dragger.hitbox.contains(relative_position);

        if release_hitbox_hit {
            state.attack_dragger.set_to_normal_if_in_hover_state();
            state.decay_dragger.set_to_normal_if_in_hover_state();
        }

        match state.release_dragger.status {
            EnvelopeDraggerStatus::Normal => {
                if release_hitbox_hit {
                    state.release_dragger.status = EnvelopeDraggerStatus::Hover;

                    state.attack_dragger.set_to_normal_if_in_hover_state();
                    state.decay_dragger.set_to_normal_if_in_hover_state();

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !release_hitbox_hit {
                    state.release_dragger.status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                ..
            } => {
                let release_duration =
                    dragging_to_duration(self.viewport_factor, x, from, original_duration);

                // FIXME
                // self.modified_by_automation = false;
                // self.update_data();

                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(
                            self.operator_index,
                            OperatorParameter::ReleaseDuration,
                        ),
                        release_duration as f32,
                    ),
                    parameter_2: None,
                };

                return (event::Status::Captured, Some(message));
            }
        }

        if let Some(dragging_from) = state.dragging_background_from {
            let zoom_factor = (dragging_from.from_point.y - y) / 50.0;

            let new_viewport_factor = (dragging_from.viewport_factor * zoom_factor.exp2())
                .min(1.0)
                .max(MIN_VIEWPORT_FACTOR);

            let x_offset_change_zoom = -dragging_from.original_visible_position
                * (dragging_from.viewport_factor - new_viewport_factor);

            let x_offset_change_drag =
                (x - dragging_from.from_point.x) / WIDTH as f32 * new_viewport_factor;

            let new_x_offset = Self::process_x_offset(
                dragging_from.original_x_offset + x_offset_change_zoom + x_offset_change_drag,
                new_viewport_factor,
            );

            let message = Message::EnvelopeChangeViewport {
                operator_index: self.operator_index,
                viewport_factor: new_viewport_factor,
                x_offset: new_x_offset,
            };

            return (event::Status::Captured, Some(message));
        }

        if bounds.contains(Point::new(x, y)) {
            (event::Status::Captured, None)
        } else {
            (event::Status::Ignored, None)
        }
    }

    fn handle_button_released(&self, state: &mut CanvasState) -> (event::Status, Option<Message>) {
        if state.release_dragger.is_dragging() {
            state.release_dragger.status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::ReleaseDuration),
                    self.release_duration,
                ),
                parameter_2: None,
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else if state.decay_dragger.is_dragging() {
            state.decay_dragger.status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::DecayDuration),
                    self.decay_duration,
                ),
                parameter_2: Some((
                    Parameter::Operator(self.operator_index, OperatorParameter::SustainVolume),
                    self.sustain_volume,
                )),
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else if state.attack_dragger.is_dragging() {
            state.attack_dragger.status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::AttackDuration),
                    self.attack_duration,
                ),
                parameter_2: None,
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else {
            let mut event_status = event::Status::Ignored;
            let mut opt_message = None;

            if state.dragging_background_from.is_some() {
                state.dragging_background_from = None;

                event_status = event::Status::Captured;
            }

            // Increment double click data release count if set
            if let Some(data) = state.double_click_data.as_mut() {
                data.releases += 1;

                event_status = event::Status::Captured;
            }

            // If this is second release without mouse movement in between,
            // send zoom to fit message
            if let Some(DoubleClickData { releases: 2, .. }) = state.double_click_data {
                state.double_click_data = None;

                self.cache.clear();

                event_status = event::Status::Captured;

                let (viewport_factor, x_offset) = self.get_zoom_to_fit_data();

                opt_message = Some(Message::EnvelopeChangeViewport {
                    operator_index: self.operator_index,
                    viewport_factor,
                    x_offset,
                });
            }

            (event_status, opt_message)
        }
    }
}

/// Display logic
impl Envelope {
    fn draw_time_markers(&self, frame: &mut Frame, style: Theme) {
        let font_regular = style.font_regular();
        let style = style.envelope().active();

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

    fn draw_viewport_indicator(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        const WIDTH: f32 = 60.0;
        const HEIGHT: f32 = 6.0;

        let style = style_sheet.active();
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

impl Program<Message> for Envelope {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        _theme: &iced_baseview::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_time_markers(frame, self.style);

            state.draw_stage_paths(frame, self.style.envelope());

            state.attack_dragger.draw(frame, self.style.envelope());
            state.decay_dragger.draw(frame, self.style.envelope());
            state.release_dragger.draw(frame, self.style.envelope());

            self.draw_viewport_indicator(frame, self.style.envelope());
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if self.updates_available.fetch_and(false, Ordering::SeqCst) {
            self.update_stage_paths(state);

            state
                .attack_dragger
                .set_center(state.attack_stage_path.end_point);
            state
                .decay_dragger
                .set_center(state.decay_stage_path.end_point);
            state
                .release_dragger
                .set_center(state.release_stage_path.end_point);

            self.cache.clear();
        }

        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => self.handle_cursor_moved(state, bounds, x, y),
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(
                iced_baseview::mouse::Button::Left,
            )) => self.handle_button_pressed(state, bounds),
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(
                iced_baseview::mouse::Button::Left,
            )) => self.handle_button_released(state),
            _ => (event::Status::Ignored, None),
        }
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
