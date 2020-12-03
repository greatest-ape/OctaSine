use std::sync::Arc;

use iced_baseview::canvas::{Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, path, Text};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length
};
use vst2_helpers::approximations::Log10Table;

use crate::SyncHandle;
use crate::voices::VoiceOperatorVolumeEnvelope;

use super::Message;


pub struct Envelope {
    log10_table: Log10Table,
    attack_duration: f32,
    attack_end_value: f32,
    decay_duration: f32,
    decay_end_value: f32,
    release_duration: f32,
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
            attack_duration: sync_handle.get_presets().get_parameter_value_float(attack_dur) as f32,
            attack_end_value: sync_handle.get_presets().get_parameter_value_float(attack_val) as f32,
            decay_duration: sync_handle.get_presets().get_parameter_value_float(decay_dur) as f32,
            decay_end_value: sync_handle.get_presets().get_parameter_value_float(decay_val) as f32,
            release_duration: sync_handle.get_presets().get_parameter_value_float(release_dur) as f32,
        }
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(256))
            .height(Length::Units(64))
            .into()
    }

    pub fn set_attack_duration(&mut self, value: f64){
        self.attack_duration = value as f32;
    }

    pub fn set_attack_end_value(&mut self, value: f64){
        self.attack_end_value = value as f32;
    }

    pub fn set_decay_duration(&mut self, value: f64){
        self.decay_duration = value as f32;
    }

    pub fn set_decay_end_value(&mut self, value: f64){
        self.decay_end_value = value as f32;
    }

    pub fn set_release_duration(&mut self, value: f64){
        self.release_duration = value as f32;
    }

    fn draw_time_markers(&self, frame: &mut Frame, total_duration: f32){
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
                let text = iced_baseview::canvas::Text {
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

    fn draw_stage_paths(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        total_duration: f32,
        sustain_duration: f32,
    ){
        let (attack_path, attack_end_point) = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        let (decay_path, decay_end_point) = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        let (sustain_path, _) = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            sustain_duration as f32,
            self.decay_end_value,
        );

        let (release_path, release_end_point) = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration + self.decay_duration + sustain_duration,
            self.decay_end_value,
            self.release_duration as f32,
            0.0
        );

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::BLACK);
        let sustain_stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.5, 0.5, 0.5));

        frame.stroke(&attack_path, stroke);
        frame.stroke(&decay_path, stroke);
        frame.stroke(&sustain_path, sustain_stroke);
        frame.stroke(&release_path, stroke);

        Self::draw_circle(frame, attack_end_point, "A");
        Self::draw_circle(frame, decay_end_point, "D");
        Self::draw_circle(frame, release_end_point, "R");
    }

    fn draw_circle(
        frame: &mut Frame,
        center: Point,
        content: &str,
    ){
        let radius = 6.0;
        let text_size = 10.0;

        let mut path_builder = path::Builder::new();

        path_builder.move_to(center);
        path_builder.circle(center, radius);

        let path = path_builder.build();

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgb(0.7, 0.7, 0.7));

        frame.fill(&path, Color::WHITE);
        frame.stroke(&path, stroke);

        let text_position = Point::new(
            center.x - text_size * 0.33,
            center.y - text_size * 0.5,
        );

        let text = Text {
            content: content.to_string(),
            position: text_position,
            size: text_size,
            color: Color::BLACK,
            ..Default::default()
        };

        frame.fill_text(text)
    }

    fn calculate_stage_path(
        log10_table: &Log10Table,
        bounds: Rectangle,
        total_duration: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
    ) -> (Path, Point) {
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

        (path.build(), to)
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

        Point::new(
            ((start_duration + duration) / total_duration) * bounds.width,
            bounds.height * (1.0 - value)
        )
    }
}


impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        let sustain_duration = 0.1 / 4.0;

        let total_duration = self.attack_duration + self.decay_duration +
            sustain_duration + self.release_duration;

        self.draw_time_markers(&mut frame, total_duration);

        self.draw_stage_paths(
            &mut frame,
            bounds,
            total_duration,
            sustain_duration
        );

        vec![frame.into_geometry()]
    }
}
