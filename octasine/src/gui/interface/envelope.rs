use std::sync::Arc;

use iced_baseview::canvas::{Canvas, Cursor, Frame, Geometry, Path, Program, Stroke};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length
};

use crate::SyncHandle;

use super::Message;


pub struct Envelope {
    pub attack_duration: f32,
    pub attack_end_value: f32,
    pub decay_duration: f32,
    pub decay_end_value: f32,
    pub release_duration: f32,
}


impl Program<Message> for Envelope {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        let sustain_duration = 0.05;

        let total_duration = self.attack_duration + self.decay_duration + sustain_duration + self.release_duration;
        let total_width = bounds.width;

        let max_height = bounds.height * 1.0;

        // Draw time markers
        for i in 0..(total_duration / 0.01) as usize {
            let x = ((0.01 * i as f32) / total_duration) * total_width;

            let path = Path::line(
                Point::new(x, 0.0),
                Point::new(x, max_height),
            );

            if i % 10 == 0 && i != 0 {
                let text = iced_baseview::canvas::Text {
                    content: format!("{:.1}s", 0.01 * i as f32),
                    position: Point::new(x, max_height),
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

        let attack_from = Point::new(0.0, max_height);
        let attack_to = Point::new(
            (self.attack_duration / total_duration) * total_width,
            max_height * (1.0 - self.attack_end_value)
        );

        let decay_to = Point::new(
            attack_to.x + (self.decay_duration / total_duration) * total_width,
            max_height * (1.0 - self.decay_end_value)
        );

        let sustain_to = Point::new(
            decay_to.x + (sustain_duration / total_duration) * total_width,
            decay_to.y
        );

        let release_to = Point::new(
            sustain_to.x + (self.release_duration / total_duration) * total_width,
            max_height
        );

        let attack = Path::line(attack_from, attack_to);
        let decay = Path::line(attack_to, decay_to);
        let sustain = Path::line(decay_to, sustain_to);
        let release = Path::line(sustain_to, release_to);

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

        /*

        */

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