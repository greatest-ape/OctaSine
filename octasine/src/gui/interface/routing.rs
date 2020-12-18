use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Vector, Size
};

use vst2_helpers::processing_parameters::ParameterValueConversion;
use crate::GuiSyncHandle;
use crate::processing_parameters::{
    ProcessingParameterOperatorModulationTarget2,
    ProcessingParameterOperatorModulationTarget3,
};

use super::Message;


const WIDTH: u16 = 84;
const HEIGHT: u16 = 108;
const SIZE: Size = Size { width: WIDTH as f32, height: HEIGHT as f32 };


enum Box {
    Operator {
        index: usize,
    },
    Modulation {
        active: bool
    }
}


pub struct ModulationMatrix {
    cache: Cache,
    size: Size,
    operator_3_target: usize,
    operator_4_target: usize,
    operator_2_additive: f64,
    operator_3_additive: f64,
    operator_4_additive: f64,
}


impl ModulationMatrix {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let operator_3_target = Self::convert_operator_3_target(
            sync_handle.get_presets().get_parameter_value_float(33)
        );

        let operator_4_target = Self::convert_operator_4_target(
            sync_handle.get_presets().get_parameter_value_float(48)
        );

        let mut matrix = Self {
            cache: Cache::default(),
            size: SIZE,
            operator_3_target,
            operator_4_target,
            operator_2_additive: sync_handle.get_presets().get_parameter_value_float(18),
            operator_3_additive: sync_handle.get_presets().get_parameter_value_float(32),
            operator_4_additive: sync_handle.get_presets().get_parameter_value_float(47),
        };


        matrix.update_data();

        matrix
    }

    fn convert_operator_3_target(value: f64) -> usize {
        ProcessingParameterOperatorModulationTarget2::to_processing(value)
    }

    fn convert_operator_4_target(value: f64) -> usize {
        ProcessingParameterOperatorModulationTarget3::to_processing(value)
    }

    pub fn set_operator_3_target(&mut self, value: f64){
        self.operator_3_target = Self::convert_operator_3_target(value);

        self.update_data();
    }

    pub fn set_operator_4_target(&mut self, value: f64){
        self.operator_4_target = Self::convert_operator_4_target(value);

        self.update_data();
    }

    fn update_data(&mut self){
        // FIXME: update stuff

        self.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_boxes(&self, frame: &mut Frame){
        self.draw_box(frame, 0, 0, Box::Operator { index: 3 });
        self.draw_box(frame, 2, 2, Box::Operator { index: 2 });
        self.draw_box(frame, 4, 4, Box::Operator { index: 1 });
        self.draw_box(frame, 6, 6, Box::Operator { index: 0 });

        self.draw_box(frame, 2, 0, Box::Modulation { active: self.operator_4_target == 2 });
        self.draw_box(frame, 4, 0, Box::Modulation { active: self.operator_4_target == 1 });
        self.draw_box(frame, 6, 0, Box::Modulation { active: self.operator_4_target == 0 });

        self.draw_box(frame, 4, 2, Box::Modulation { active: self.operator_3_target == 1 });
        self.draw_box(frame, 6, 2, Box::Modulation { active: self.operator_3_target == 0 });

        self.draw_box(frame, 6, 4, Box::Modulation { active: true });

        self.draw_output_box(frame);
    }

    fn draw_output_box(&self, frame: &mut Frame){
        let bounds = frame.size();

        let x_bla = bounds.width / 7.0;
        let y_bla = bounds.height / 9.0;

        let base_top_left = Point::new(
            0.0,
            8.0 * y_bla,
        );
        let base_size = Size::new(x_bla, y_bla);

        let size_multiplier = 1.5;

        let size = Size {
            width: base_size.width * 6.0 + base_size.width * size_multiplier,
            height: base_size.height * size_multiplier,
        };
        let top_left = Point {
            x: base_top_left.x - (size_multiplier - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (size_multiplier - 1.0) * base_size.height / 2.0,
        };

        let rect = Path::rectangle(top_left, size);

        frame.fill(&rect, Color::from_rgb(0.8, 0.8, 0.8));

        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        frame.stroke(&rect, stroke);
    }

    fn draw_box(&self, frame: &mut Frame, x: usize, y: usize, box_type: Box) -> Point {
        let bounds = frame.size();

        let x_bla = bounds.width / 7.0;
        let y_bla = bounds.height / 9.0;

        let base_top_left = Point::new(
            x as f32 * x_bla,
            y as f32 * y_bla,
        );
        let base_size = Size::new(x_bla, y_bla);

        match box_type {
            Box::Operator { index } => {
                let size_multiplier = 1.5;

                let size = Size {
                    width: base_size.width * size_multiplier,
                    height: base_size.height * size_multiplier,
                };
                let top_left = Point {
                    x: base_top_left.x - (size_multiplier - 1.0) * base_size.width / 2.0,
                    y: base_top_left.y - (size_multiplier - 1.0) * base_size.height / 2.0,
                };

                let rect = Path::rectangle(top_left, size);

                let stroke = Stroke::default()
                    .with_color(Color::BLACK)
                    .with_width(1.0);

                frame.stroke(&rect, stroke);

                let text_position = Point {
                    x: base_top_left.x + 2.45,
                    y: base_top_left.y + 0.45,
                };

                let text = Text {
                    content: format!("{}", index + 1),
                    position: text_position,
                    size: 12.0,
                    ..Default::default()
                };

                frame.fill_text(text);
            },
            Box::Modulation { active } => {
                let rect = Path::rectangle(base_top_left, base_size);

                if active {
                    frame.fill(&rect, Color::from_rgb8(27, 159, 31));
                }

                let stroke = Stroke::default()
                    .with_color(Color::BLACK)
                    .with_width(1.0);

                frame.stroke(&rect, stroke);

            },
        }

        Point::new(
            base_top_left.x + base_size.width / 2.0,
            base_top_left.y + base_size.height / 2.0,
        )
    }
}


impl Program<Message> for ModulationMatrix {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_boxes(frame)
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
                    /*
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
                    */

                    return (event::Status::Captured, None);
                }
            },
            _ => (),
        };

        (event::Status::Ignored, None)
    }
}
