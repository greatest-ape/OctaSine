use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Vector, Size
};

use vst2_helpers::approximations::Log10Table;

use crate::SyncHandle;
use crate::voices::VoiceOperatorVolumeEnvelope;
use crate::constants::{ENVELOPE_MIN_DURATION, ENVELOPE_MAX_DURATION};

use super::Message;


const WIDTH: u16 = 256;
const HEIGHT: u16 = 64;
const SIZE: Size = Size { width: WIDTH as f32, height: HEIGHT as f32 };

const SUSTAIN_DURATION: f32 = 0.1 / 4.0;
const DRAGGER_RADIUS: f32 = 4.0;

const ENVELOPE_PATH_SCALE_X: f32 = 1.0 - (1.0 / 16.0);
const ENVELOPE_PATH_SCALE_Y: f32 = 1.0 - (1.0 / 8.0) - (1.0 / 16.0);


struct EnvelopeStagePath {
    path: Path,
    end_point: Point,
}


impl EnvelopeStagePath {
    fn new(
        log10_table: &Log10Table,
        size: Size,
        total_duration: f32,
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
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            0.0
        );
        let control_a = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            1.0 / 3.0
        );
        let control_b = Self::calculate_stage_progress_point(
            log10_table,
            size,
            total_duration,
            start_duration,
            start_value,
            stage_duration,
            stage_end_value,
            2.0 / 3.0
        );
        let to = Self::calculate_stage_progress_point(
            log10_table,
            size,
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
        size: Size,
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
        let point = Point::new(
            ((start_duration + duration) / total_duration) * size.width,
            size.height * (1.0 - value)
        );

        scale_point(size, point)
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
    size: Size,
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
        sync_handle: &H,
        operator_index: usize,
    ) -> Self {
        let (attack_dur, attack_val, decay_dur, decay_val, release_dur) = match operator_index {
            0 => (10, 11, 12, 13, 14),
            1 => (24, 25, 26, 27, 28),
            2 => (39, 40, 41, 42, 43),
            3 => (54, 55, 56, 57, 58),
            _ => unreachable!(),
        };

        let mut envelope = Self {
            log10_table: Log10Table::default(),
            cache: Cache::default(),
            attack_duration: sync_handle.get_presets().get_parameter_value_float(attack_dur) as f32,
            attack_end_value: sync_handle.get_presets().get_parameter_value_float(attack_val) as f32,
            decay_duration: sync_handle.get_presets().get_parameter_value_float(decay_dur) as f32,
            decay_end_value: sync_handle.get_presets().get_parameter_value_float(decay_val) as f32,
            release_duration: sync_handle.get_presets().get_parameter_value_float(release_dur) as f32,
            size: SIZE,
            attack_stage_path: EnvelopeStagePath::default(),
            decay_stage_path: EnvelopeStagePath::default(),
            sustain_stage_path: EnvelopeStagePath::default(),
            release_stage_path: EnvelopeStagePath::default(),
            attack_dragger: EnvelopeDragger::default(),
            decay_dragger: EnvelopeDragger::default(),
            release_dragger: EnvelopeDragger::default(),
        };

        envelope.update_data();

        envelope
    }

    fn process_envelope_duration(sync_value: f64) -> f32 {
        sync_value.max(ENVELOPE_MIN_DURATION / ENVELOPE_MAX_DURATION) as f32
    }

    pub fn set_attack_duration(&mut self, value: f64){
        self.attack_duration = Self::process_envelope_duration(value);

        self.update_data();
    }

    pub fn set_attack_end_value(&mut self, value: f64){
        self.attack_end_value = value as f32;

        self.update_data();
    }

    pub fn set_decay_duration(&mut self, value: f64){
        self.decay_duration = Self::process_envelope_duration(value);

        self.update_data();
    }

    pub fn set_decay_end_value(&mut self, value: f64){
        self.decay_end_value = value as f32;

        self.update_data();
    }

    pub fn set_release_duration(&mut self, value: f64){
        self.release_duration = Self::process_envelope_duration(value);

        self.update_data();
    }

    fn get_total_duration(&self) -> f32 {
        self.attack_duration + self.decay_duration + SUSTAIN_DURATION + self.release_duration
    }

    fn update_data(&mut self){
        self.update_stage_paths();

        self.attack_dragger.set_center(self.attack_stage_path.end_point);
        self.decay_dragger.set_center(self.decay_stage_path.end_point);
        self.release_dragger.set_center(self.release_stage_path.end_point);

        self.cache.clear();
    }

    fn update_stage_paths(&mut self){
        let total_duration = self.get_total_duration();
        let sustain_duration = SUSTAIN_DURATION;

        self.attack_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        self.decay_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        self.sustain_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            sustain_duration as f32,
            self.decay_end_value,
        );

        self.release_stage_path = EnvelopeStagePath::new(
            &self.log10_table,
            self.size,
            total_duration,
            self.attack_duration + self.decay_duration + sustain_duration,
            self.decay_end_value,
            self.release_duration as f32,
            0.0
        );
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_time_markers(&self, frame: &mut Frame){
        let total_duration = self.get_total_duration();

        let mut time_marker_interval = 0.01 / 4.0;

        let mut num_markers = loop {
            let num_markers = (total_duration / time_marker_interval) as usize;

            if num_markers <= 110 {
                break num_markers;
            } else {
                time_marker_interval *= 10.0;
            }
        };

        // End marker
        num_markers += 1;

        for i in 0..num_markers {
            let x = ((time_marker_interval * i as f32) / total_duration) * self.size.width;

            let top_point = Point::new(x, 0.0);
            let bottom_point = Point::new(x, self.size.height);

            let path = Path::line(
                scale_point_x(self.size, top_point),
                scale_point_x(self.size, bottom_point),
            );

            if i % 10 == 0 && i != 0 {
                let text_point = Point::new(x - 10.0, self.size.height);

                let text = Text {
                    content: format!("{:.1}s", time_marker_interval * 4.0 * i as f32),
                    position: scale_point_x(self.size, text_point),
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
        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {x, y}) => {
                if bounds.contains(Point::new(x, y)){
                    let relative_position = Point::new(
                        x - bounds.x,
                        y - bounds.y,
                    );

                    let mut changed = false;

                    changed |= self.attack_dragger.update(relative_position);
                    changed |= self.decay_dragger.update(relative_position);
                    changed |= self.release_dragger.update(relative_position);

                    if changed {
                        self.cache.clear();
                    }

                    return (event::Status::Captured, None);
                }
            },
            _ => (),
        };

        (event::Status::Ignored, None)
    }
}


fn scale_point(size: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - ENVELOPE_PATH_SCALE_X) * size.width / 2.0,
        y: (1.0 - ENVELOPE_PATH_SCALE_Y) * size.height / 2.0
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