use std::sync::Arc;

use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length
};

use vst2_helpers::approximations::Log10Table;

use crate::SyncHandle;
use crate::voices::VoiceOperatorVolumeEnvelope;
use crate::constants::{ENVELOPE_MIN_DURATION, ENVELOPE_MAX_DURATION};

use super::Message;


const SUSTAIN_DURATION: f32 = 0.1 / 4.0;
const DRAGGER_RADIUS: f32 = 4.0;


struct EnvelopeStagePath {
    path: Path,
    end_point: Point,
}


impl EnvelopeStagePath {
    fn new(
        log10_table: &Log10Table,
        bounds: Rectangle,
        total_duration: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
    ) -> Self {
        let mut path = path::Builder::new();

        let start = Self::calculate_stage_progress_point(
            log10_table,
            bounds,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            0.0
        );
        let control_a = Self::calculate_stage_progress_point(
            log10_table,
            bounds,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0 / 3.0
        );
        let control_b = Self::calculate_stage_progress_point(
            log10_table,
            bounds,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            2.0 / 3.0
        );
        let to = Self::calculate_stage_progress_point(
            log10_table,
            bounds,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0
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
        bounds: Rectangle,
        total_duration: f32,
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
        Point::new(
            ((start_duration + duration) / total_duration) * bounds.width,
            bounds.height * (1.0 - value)
        )
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
    Hover
}


struct EnvelopeDragger {
    center: Point,
    radius: f32,
    hitbox: Rectangle,
    status: EnvelopeDraggerStatus,
}


impl EnvelopeDragger {
    fn set_center(&mut self, center: Point){
        self.center = center;

        self.hitbox.width = self.radius * 2.0;
        self.hitbox.height = self.radius * 2.0;
        self.hitbox.x = (center.x - self.radius / 2.0).max(0.0);
        self.hitbox.y = (center.y - self.radius / 2.0).max(0.0);
    }

    fn update(&mut self, cursor_position: Point) -> bool {
        match (self.hitbox.contains(cursor_position), self.status){
            (true, EnvelopeDraggerStatus::Normal) => {
                self.status = EnvelopeDraggerStatus::Hover;

                true
            },
            (false, EnvelopeDraggerStatus::Hover) => {
                self.status = EnvelopeDraggerStatus::Normal;

                true
            },
            _ => false,
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
    attack_duration: f32,
    attack_end_value: f32,
    decay_duration: f32,
    decay_end_value: f32,
    release_duration: f32,
    bounds: Option<Rectangle>,
    attack_stage_path: EnvelopeStagePath,
    decay_stage_path: EnvelopeStagePath,
    sustain_stage_path: EnvelopeStagePath,
    release_stage_path: EnvelopeStagePath,
    attack_dragger: EnvelopeDragger,
    decay_dragger: EnvelopeDragger,
    release_dragger: EnvelopeDragger,
}


impl Envelope {
    pub fn new<H: SyncHandle>(
        sync_handle: &Arc<H>,
        operator_index: usize,
    ) -> Self {
        let (attack_dur, attack_val, decay_dur, decay_val, release_dur) = match operator_index {
            0 => (10, 11, 12, 13, 14),
            1 => (24, 25, 26, 27, 28),
            2 => (39, 40, 41, 42, 43),
            3 => (54, 55, 56, 57, 58),
            _ => unreachable!(),
        };

        Self {
            log10_table: Log10Table::default(),
            cache: Cache::default(),
            attack_duration: sync_handle.get_presets().get_parameter_value_float(attack_dur) as f32,
            attack_end_value: sync_handle.get_presets().get_parameter_value_float(attack_val) as f32,
            decay_duration: sync_handle.get_presets().get_parameter_value_float(decay_dur) as f32,
            decay_end_value: sync_handle.get_presets().get_parameter_value_float(decay_val) as f32,
            release_duration: sync_handle.get_presets().get_parameter_value_float(release_dur) as f32,
            bounds: None,
            attack_stage_path: EnvelopeStagePath::default(),
            decay_stage_path: EnvelopeStagePath::default(),
            sustain_stage_path: EnvelopeStagePath::default(),
            release_stage_path: EnvelopeStagePath::default(),
            attack_dragger: EnvelopeDragger::default(),
            decay_dragger: EnvelopeDragger::default(),
            release_dragger: EnvelopeDragger::default(),
        }
    }

    fn process_envelope_duration(sync_value: f64) -> f32 {
        sync_value.max(ENVELOPE_MIN_DURATION / ENVELOPE_MAX_DURATION) as f32
    }

    pub fn set_attack_duration(&mut self, value: f64){
        self.attack_duration = Self::process_envelope_duration(value);

        self.update_data(None);
    }

    pub fn set_attack_end_value(&mut self, value: f64){
        self.attack_end_value = value as f32;

        self.update_data(None);
    }

    pub fn set_decay_duration(&mut self, value: f64){
        self.decay_duration = Self::process_envelope_duration(value);

        self.update_data(None);
    }

    pub fn set_decay_end_value(&mut self, value: f64){
        self.decay_end_value = value as f32;

        self.update_data(None);
    }

    pub fn set_release_duration(&mut self, value: f64){
        self.release_duration = Self::process_envelope_duration(value);

        self.update_data(None);
    }

    fn get_total_duration(&self) -> f32 {
        self.attack_duration + self.decay_duration + SUSTAIN_DURATION + self.release_duration
    }

    fn update_data(&mut self, bounds: Option<Rectangle>){
        if let Some(bounds) = bounds {
            self.bounds = Some(bounds);
        }

        self.update_stage_paths();

        self.attack_dragger.set_center(self.attack_stage_path.end_point);
        self.decay_dragger.set_center(self.decay_stage_path.end_point);
        self.release_dragger.set_center(self.release_stage_path.end_point);

        self.cache.clear();
    }

    fn update_stage_paths(&mut self){
        let bounds = match self.bounds {
            Some(bounds) => bounds,
            None => return, // FIXME
        };

        let total_duration = self.get_total_duration();
        let sustain_duration = SUSTAIN_DURATION;

        self.attack_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            bounds,
            total_duration,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        self.decay_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        self.sustain_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            sustain_duration as f32,
            self.decay_end_value,
        );

        self.release_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration + self.decay_duration + sustain_duration,
            self.decay_end_value,
            self.release_duration as f32,
            0.0
        );
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(256))
            .height(Length::Units(64))
            .into()
    }

    fn draw_time_markers(&self, frame: &mut Frame){
        let total_duration = self.get_total_duration();

        let total_width = frame.width();
        let max_height = frame.height();

        let mut time_marker_interval = 0.01 / 4.0;

        let num_markers = loop {
            let num_markers = (total_duration / time_marker_interval) as usize;

            if num_markers <= 110 {
                break num_markers;
            } else {
                time_marker_interval *= 10.0;
            }
        };

        for i in 0..num_markers {
            let x = ((time_marker_interval * i as f32) / total_duration) * total_width;

            let path = Path::line(
                Point::new(x, 0.0),
                Point::new(x, max_height),
            );

            if i % 10 == 0 && i != 0 {
                let text = Text {
                    content: format!("{:.1}s", time_marker_interval * 4.0 * i as f32),
                    position: Point::new(x - 10.0, max_height),
                    size: 12.0,
                    ..Default::default()
                };
        
                frame.fill_text(text);

                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.7, 0.7, 0.7));

                frame.stroke(&path, stroke);
            } else {
                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.9, 0.9, 0.9));

                frame.stroke(&path, stroke);
            }
        }
    }

    fn draw_stage_paths(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::BLACK);
        let sustain_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.5, 0.5, 0.5));

        frame.stroke(&self.attack_stage_path.path, stroke);
        frame.stroke(&self.decay_stage_path.path, stroke);
        frame.stroke(&self.sustain_stage_path.path, sustain_stroke);
        frame.stroke(&self.release_stage_path.path, stroke);
    }

    fn draw_dragger(frame: &mut Frame, dragger: &EnvelopeDragger){
        let circle_path = {
            let mut builder = path::Builder::new();

            builder.move_to(dragger.center);
            builder.circle(dragger.center, dragger.radius);

            builder.build()
        };

        let fill_color = match dragger.status {
            EnvelopeDraggerStatus::Normal => Color::from_rgb(1.0, 1.0, 1.0),
            EnvelopeDraggerStatus::Hover => Color::from_rgb(0.0, 0.0, 0.0),
        };

        frame.fill(&circle_path, fill_color);

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.5, 0.5, 0.5));

        frame.stroke(&circle_path, stroke);
    }
}


impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_time_markers(frame);
            self.draw_stage_paths(frame);

            Self::draw_dragger(frame, &self.attack_dragger);
            Self::draw_dragger(frame, &self.decay_dragger);
            Self::draw_dragger(frame, &self.release_dragger);
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if self.bounds.is_none(){
            self.update_data(Some(bounds))
        }

        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {x, y}) => {
                if bounds.contains(Point::new(x, y)){
                    let cursor_position = Point::new(
                        x - bounds.x,
                        y - bounds.y,
                    );

                    let mut changed = false;

                    changed |= self.attack_dragger.update(cursor_position);
                    changed |= self.decay_dragger.update(cursor_position);
                    changed |= self.release_dragger.update(cursor_position);

                    if changed {
                        self.cache.clear();
                    }
                }
            },
            _ => (),
        };

        (event::Status::Ignored, None)
    }
}
