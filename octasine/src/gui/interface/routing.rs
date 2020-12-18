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


struct OperatorBox {
    text: Text,
    path: Path,
    center: Point,
}


impl Default for OperatorBox {
    fn default() -> Self {
        Self {
            text: Text::default(),
            path: Path::rectangle(Point::default(), Size::new(0.0, 0.0)),
            center: Point::default(),
        }
    }
}


impl OperatorBox {
    fn new(bounds: Size, index: usize) -> Self {
        let x_bla = bounds.width / 7.0;
        let y_bla = bounds.height / 9.0;

        let (x, y) = match index {
            0 => (0, 0),
            1 => (2, 2),
            2 => (4, 4),
            3 => (6, 6),
            _ => unreachable!(),
        };

        let base_top_left = Point::new(
            x as f32 * x_bla,
            y as f32 * y_bla,
        );
        let base_size = Size::new(x_bla, y_bla);

        let size_multiplier = 1.5;

        let size = Size {
            width: base_size.width * size_multiplier,
            height: base_size.height * size_multiplier,
        };
        let top_left = Point {
            x: base_top_left.x - (size_multiplier - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (size_multiplier - 1.0) * base_size.height / 2.0,
        };

        let path = Path::rectangle(top_left, size);
        let center = Rectangle::new(top_left, size).center();

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


        Self {
            text,
            path,
            center,
        }
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        frame.stroke(&self.path, stroke);
        frame.fill_text(self.text.clone());
    }
}


struct ModulationBox {
    path: Path,
    center: Point,
    active: bool
}


impl Default for ModulationBox {
    fn default() -> Self {
        Self {
            path: Path::rectangle(Point::default(), Size::new(0.0, 0.0)),
            center: Point::default(),
            active: false,
        }
    }
}


impl ModulationBox {
    fn new(bounds: Size, from: usize, to: usize, active: bool) -> Self {
        let (x, y) = match (from, to) {
            (3, 2) => (2, 0),
            (3, 1) => (4, 0),
            (3, 0) => (6, 0),
            (2, 1) => (4, 2),
            (2, 0) => (6, 2),
            (1, 0) => (6, 4),
            _ => unreachable!(),
        };

        let x_bla = bounds.width / 7.0;
        let y_bla = bounds.height / 9.0;

        let top_left = Point::new(
            x as f32 * x_bla,
            y as f32 * y_bla,
        );
        let size = Size::new(x_bla, y_bla);

        let path = Path::rectangle(top_left, size);
        let center = Rectangle::new(top_left, size).center();

        Self {
            path,
            center,
            active,
        }
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        if self.active {
            frame.fill(&self.path, Color::from_rgb8(27, 159, 31));
        }

        frame.stroke(&self.path, stroke);
    }
}


struct OutputBox {
    path: Path,
    center: Point,
}


impl Default for OutputBox {
    fn default() -> Self {
        Self {
            path: Path::rectangle(Point::default(), Size::new(0.0, 0.0)),
            center: Point::default(),
        }
    }
}


impl OutputBox {
    fn new(bounds: Size) -> Self {
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

        let path = Path::rectangle(top_left, size);

        Self {
            path,
            center: Rectangle::new(top_left, size).center()
        }
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        frame.stroke(&self.path, stroke);
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
    operator_1_box: OperatorBox,
    operator_2_box: OperatorBox,
    operator_3_box: OperatorBox,
    operator_4_box: OperatorBox,
    operator_4_mod_3_box: ModulationBox,
    operator_4_mod_2_box: ModulationBox,
    operator_4_mod_1_box: ModulationBox,
    operator_3_mod_2_box: ModulationBox,
    operator_3_mod_1_box: ModulationBox,
    operator_2_mod_1_box: ModulationBox,
    output_box: OutputBox,
}


impl ModulationMatrix {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let operator_3_target = Self::convert_operator_3_target(
            sync_handle.get_presets().get_parameter_value_float(33)
        );
        let operator_4_target = Self::convert_operator_4_target(
            sync_handle.get_presets().get_parameter_value_float(48)
        );
        let operator_2_additive = sync_handle.get_presets()
            .get_parameter_value_float(18);
        let operator_3_additive = sync_handle.get_presets()
            .get_parameter_value_float(32);
        let operator_4_additive = sync_handle.get_presets()
            .get_parameter_value_float(47);

        let mut matrix = Self {
            cache: Cache::default(),
            size: SIZE,
            operator_3_target,
            operator_4_target,
            operator_2_additive,
            operator_3_additive,
            operator_4_additive,
            operator_1_box: Default::default(),
            operator_2_box: Default::default(),
            operator_3_box: Default::default(),
            operator_4_box: Default::default(),
            operator_4_mod_3_box: Default::default(),
            operator_4_mod_2_box: Default::default(),
            operator_4_mod_1_box: Default::default(),
            operator_3_mod_2_box: Default::default(),
            operator_3_mod_1_box: Default::default(),
            operator_2_mod_1_box: Default::default(),
            output_box: Default::default(),
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
        let bounds = self.size;

        self.operator_1_box = OperatorBox::new(bounds, 0);
        self.operator_2_box = OperatorBox::new(bounds, 1);
        self.operator_3_box = OperatorBox::new(bounds, 2);
        self.operator_4_box = OperatorBox::new(bounds, 3);

        self.operator_4_mod_3_box = ModulationBox::new(bounds, 3, 2, self.operator_4_target == 2);
        self.operator_4_mod_2_box = ModulationBox::new(bounds, 3, 1, self.operator_4_target == 1);
        self.operator_4_mod_1_box = ModulationBox::new(bounds, 3, 0, self.operator_4_target == 0);
        self.operator_3_mod_2_box = ModulationBox::new(bounds, 2, 1, self.operator_3_target == 1);
        self.operator_3_mod_1_box = ModulationBox::new(bounds, 2, 0, self.operator_3_target == 0);
        self.operator_2_mod_1_box = ModulationBox::new(bounds, 1, 0, true);

        self.output_box = OutputBox::new(bounds);

        self.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_boxes(&self, frame: &mut Frame){
        self.operator_1_box.draw(frame);
        self.operator_2_box.draw(frame);
        self.operator_3_box.draw(frame);
        self.operator_4_box.draw(frame);

        self.operator_4_mod_3_box.draw(frame);
        self.operator_4_mod_2_box.draw(frame);
        self.operator_4_mod_1_box.draw(frame);
        self.operator_3_mod_2_box.draw(frame);
        self.operator_3_mod_1_box.draw(frame);
        self.operator_2_mod_1_box.draw(frame);

        self.output_box.draw(frame);
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
