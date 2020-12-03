use std::sync::Arc;

use iced_baseview::canvas::{Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, path};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length
};
use vst2_helpers::approximations::Log10Table;

use crate::SyncHandle;
use crate::voices::VoiceOperatorVolumeEnvelope;

use super::Message;


pub struct Envelope {
    log10_table: Log10Table,
    pub attack_duration: f32,
    pub attack_end_value: f32,
    pub decay_duration: f32,
    pub decay_end_value: f32,
    pub release_duration: f32,
}


impl Envelope {
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

    fn calculate_stage_path(
        log10_table: &Log10Table,
        bounds: Rectangle,
        total_duration: f32,
        start_duration: f32,
        start_value: f32,
        stage_duration: f32,
        stage_end_value: f32,
    ) -> Path {
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

        path.build()
    }
}


impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        let sustain_duration = 0.1 / 4.0;

        let total_duration = self.attack_duration + self.decay_duration +
            sustain_duration + self.release_duration;

        self.draw_time_markers(&mut frame, total_duration);

        let attack = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            0.0,
            0.0,
            self.attack_duration as f32,
            self.attack_end_value as f32,
        );

        let decay = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration,
            self.attack_end_value,
            self.decay_duration as f32,
            self.decay_end_value as f32,
        );

        let sustain = Self::calculate_stage_path(
            &self.log10_table,
            bounds,
            total_duration,
            self.attack_duration + self.decay_duration,
            self.decay_end_value,
            sustain_duration as f32,
            self.decay_end_value,
        );

        let release = Self::calculate_stage_path(
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

        frame.stroke(&attack, stroke);
        frame.stroke(&decay, stroke);
        frame.stroke(&sustain, sustain_stroke);
        frame.stroke(&release, stroke);

        vec![frame.into_geometry()]
    }
}


impl Envelope {
    pub fn new<H: SyncHandle>(
        sync_handle: &Arc<H>,
        operator_index: usize,
    ) -> Self {
        let attack_duration = 10;
        let attack_end_value = 11;
        let decay_duration = 12;
        let decay_end_value = 13;
        let release_duration = 14;

        Self {
            log10_table: Log10Table::default(),
            attack_duration: sync_handle.get_presets().get_parameter_value_float(attack_duration) as f32,
            attack_end_value: sync_handle.get_presets().get_parameter_value_float(attack_end_value) as f32,
            decay_duration: sync_handle.get_presets().get_parameter_value_float(decay_duration) as f32,
            decay_end_value: sync_handle.get_presets().get_parameter_value_float(decay_end_value) as f32,
            release_duration: sync_handle.get_presets().get_parameter_value_float(release_duration) as f32,
        }
    }

    pub fn view<H: SyncHandle>(&mut self, sync_handle: &Arc<H>) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(256))
            .height(Length::Units(64))
            .into()
    }
}