use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Vector, Size, mouse
};

use vst2_helpers::processing_parameters::ParameterValueConversion;
use crate::GuiSyncHandle;
use crate::processing_parameters::{
    ProcessingParameterOperatorModulationTarget2,
    ProcessingParameterOperatorModulationTarget3,
};

use super::Message;


const BACKGROUND_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);

pub const HEIGHT: u16 = 16 * 7;
const SMALL_BOX_SIZE: u16 = 12;
const BIG_BOX_SIZE: u16 = 16;

// Calculated from the constants above
const SCALE: f32 = SMALL_BOX_SIZE as f32 / (HEIGHT as f32 / 8.0);
const WIDTH_FLOAT: f32 = ((HEIGHT as f64 / 8.0) * 7.0) as f32;
const SIZE: Size = Size { width: WIDTH_FLOAT, height: HEIGHT as f32 };
const OPERATOR_BOX_SCALE: f32 = BIG_BOX_SIZE as f32 / SMALL_BOX_SIZE as f32;
const WIDTH: u16 = WIDTH_FLOAT as u16;


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
        let y_bla = bounds.height / 8.0;

        let (x, y) = match index {
            3 => (0, 0),
            2 => (2, 2),
            1 => (4, 4),
            0 => (6, 6),
            _ => unreachable!(),
        };

        let base_top_left = Point::new(
            x as f32 * x_bla,
            y as f32 * y_bla,
        );
        let base_size = Size::new(x_bla, y_bla);

        let size = Size {
            width: base_size.width * OPERATOR_BOX_SCALE,
            height: base_size.height * OPERATOR_BOX_SCALE,
        };
        let top_left = Point {
            x: base_top_left.x - (OPERATOR_BOX_SCALE - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (OPERATOR_BOX_SCALE - 1.0) * base_size.height / 2.0,
        };

        let top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        let path = Path::rectangle(top_left, size);
        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let text_position = Point {
            x: base_top_left.x + 2.5,
            y: base_top_left.y,
        };

        let text_position = scale_point(bounds, text_position);

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

        frame.fill(&self.path, Color::WHITE);
        frame.stroke(&self.path, stroke);
        frame.fill_text(self.text.clone());
    }
}


enum ModulationBoxChange {
    Update(Message),
    ClearCache,
    None
}


struct ModulationBox {
    path: Path,
    center: Point,
    rect: Rectangle,
    active: bool,
    hover: bool,
    click_started: bool,
    message: Option<Message>,
}


impl Default for ModulationBox {
    fn default() -> Self {
        Self {
            path: Path::rectangle(Point::default(), Size::new(0.0, 0.0)),
            center: Point::default(),
            rect: Rectangle::new(Point::default(), Size::new(0.0, 0.0)),
            active: false,
            hover: false,
            click_started: false,
            message: None,
        }
    }
}


impl ModulationBox {
    fn new(
        bounds: Size,
        from: usize,
        to: usize,
        active: bool,
        message: Option<Message>
    ) -> Self {
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
        let y_bla = bounds.height / 8.0;

        let top_left = Point::new(
            x as f32 * x_bla,
            y as f32 * y_bla,
        );
        let size = Size::new(x_bla, y_bla);

        let top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        let path = Path::rectangle(top_left, size);
        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        Self {
            path,
            center,
            rect,
            active,
            hover: false,
            click_started: false,
            message
        }
    }

    fn update(
        &mut self,
        bounds: Rectangle,
        event: event::Event
    ) -> ModulationBoxChange {
        if let Some(message) = self.message.as_ref() {
            match event {
                event::Event::Mouse(mouse::Event::CursorMoved {x, y}) => {
                    let cursor = Point::new(
                        x - bounds.x,
                        y - bounds.y,
                    );

                    match (self.hover, self.rect.contains(cursor)){
                        (false, true) => {
                            self.hover = true;
        
                            return ModulationBoxChange::ClearCache;
                        },
                        (true, false) => {
                            self.hover = false;
        
                            return ModulationBoxChange::ClearCache;
                        },
                        _ => (),
                    }
                },
                event::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    if self.hover {
                        self.click_started = true;
                    }
                },
                event::Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                    if self.hover && self.click_started {
                        self.click_started = false;

                        return ModulationBoxChange::Update(message.clone());
                    }
                },
                _ => (),
            }
        }

        ModulationBoxChange::None
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        if self.active || self.hover {
            frame.fill(&self.path, Color::from_rgb8(27, 159, 31));
        } else {
            frame.fill(&self.path, Color::WHITE);
        }

        frame.stroke(&self.path, stroke);
    }
}


struct OutputBox {
    path: Path,
    y: f32,
}


impl Default for OutputBox {
    fn default() -> Self {
        Self {
            path: Path::rectangle(Point::default(), Size::new(0.0, 0.0)),
            y: 0.0,
        }
    }
}


impl OutputBox {
    fn new(bounds: Size) -> Self {
        let x_bla = bounds.width / 7.0;
        let y_bla = bounds.height / 8.0;

        let base_top_left = Point::new(
            0.0,
            7.0 * y_bla,
        );
        let base_size = Size::new(x_bla, y_bla);

        let size = Size {
            width: base_size.width * 6.0 + base_size.width * OPERATOR_BOX_SCALE,
            height: base_size.height * OPERATOR_BOX_SCALE,
        };
        let left = Point {
            x: base_top_left.x - (OPERATOR_BOX_SCALE - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (OPERATOR_BOX_SCALE - 1.0) * base_size.height / 2.0,
        };
        let right = Point {
            x: left.x + size.width,
            y: left.y,
        };

        let mut left = scale_point(bounds, left);
        let mut right = scale_point(bounds, right);

        let size = scale_size(size);

        left.y += size.height;
        right.y += size.height;

        // let path = Path::rectangle(top_left, size);
        let path = Path::line(left, right);

        Self {
            path,
            y: left.y,
        }
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_color(Color::BLACK)
            .with_width(1.0);

        frame.stroke(&self.path, stroke);
    }
}


struct OperatorLine {
    path: Path,
    opacity: f32,
}


impl Default for OperatorLine {
    fn default() -> Self {
        Self {
            path: Path::line(Point::default(), Point::default()),
            opacity: 0.0,
        }
    }
}


impl OperatorLine {
    fn additive(from: Point, to_y: f32, opacity: f32) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from, to);

        Self {
            path,
            opacity,
        }
    }

    fn modulation(
        from: Point,
        through: Point,
        to: Point,
        opacity: f32,
    ) -> Self {
        let mut builder = path::Builder::new();

        builder.move_to(from);
        builder.line_to(through);
        builder.line_to(to);

        let path = builder.build();

        Self {
            path,
            opacity,
        }
    }

    fn draw(&self, frame: &mut Frame){
        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgba(0.0, 0.0, 0.0, self.opacity));

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
    operator_4_additive_line: OperatorLine,
    operator_3_additive_line: OperatorLine,
    operator_2_additive_line: OperatorLine,
    operator_1_additive_line: OperatorLine,
    operator_4_modulation_line: OperatorLine,
    operator_3_modulation_line: OperatorLine,
    operator_2_modulation_line: OperatorLine,
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
            operator_4_additive_line: Default::default(),
            operator_3_additive_line: Default::default(),
            operator_2_additive_line: Default::default(),
            operator_1_additive_line: Default::default(),
            operator_4_modulation_line: Default::default(),
            operator_3_modulation_line: Default::default(),
            operator_2_modulation_line: Default::default(),
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

    pub fn set_operator_4_additive(&mut self, value: f64){
        self.operator_4_additive = value;

        self.update_data();
    }

    pub fn set_operator_3_additive(&mut self, value: f64){
        self.operator_3_additive = value;

        self.update_data();
    }

    pub fn set_operator_2_additive(&mut self, value: f64){
        self.operator_2_additive = value;

        self.update_data();
    }

    fn update_data(&mut self){
        let bounds = self.size;

        self.operator_1_box = OperatorBox::new(bounds, 0);
        self.operator_2_box = OperatorBox::new(bounds, 1);
        self.operator_3_box = OperatorBox::new(bounds, 2);
        self.operator_4_box = OperatorBox::new(bounds, 3);

        self.operator_4_mod_3_box = ModulationBox::new(
            bounds,
            3,
            2,
            self.operator_4_target == 2,
            Some(Message::ParameterChange(48, iced_audio::Normal::new(1.0))),
        );
        self.operator_4_mod_2_box = ModulationBox::new(
            bounds,
            3,
            1,
            self.operator_4_target == 1,
            Some(Message::ParameterChange(48, iced_audio::Normal::new(0.5))),
        );
        self.operator_4_mod_1_box = ModulationBox::new(
            bounds,
            3,
            0,
            self.operator_4_target == 0,
            Some(Message::ParameterChange(48, iced_audio::Normal::new(0.0))),
        );
        self.operator_3_mod_2_box = ModulationBox::new(
            bounds,
            2,
            1,
            self.operator_3_target == 1,
            Some(Message::ParameterChange(33, iced_audio::Normal::new(1.0))),
        );
        self.operator_3_mod_1_box = ModulationBox::new(
            bounds,
            2,
            0,
            self.operator_3_target == 0,
            Some(Message::ParameterChange(33, iced_audio::Normal::new(0.0))),
        );
        self.operator_2_mod_1_box = ModulationBox::new(
            bounds,
            1,
            0,
            true,
            None,
        );

        self.output_box = OutputBox::new(bounds);

        self.update_additive_lines();

        self.update_operator_4_modulation_line();
        self.update_operator_3_modulation_line();
        self.update_operator_2_modulation_line();

        self.cache.clear();
    }

    fn update_additive_lines(&mut self){
        self.operator_4_additive_line = OperatorLine::additive(
            self.operator_4_box.center,
            self.output_box.y,
            self.operator_4_additive as f32,
        );
        self.operator_3_additive_line = OperatorLine::additive(
            self.operator_3_box.center,
            self.output_box.y,
            self.operator_3_additive as f32,
        );
        self.operator_2_additive_line = OperatorLine::additive(
            self.operator_2_box.center,
            self.output_box.y,
            self.operator_2_additive as f32,
        );
        self.operator_1_additive_line = OperatorLine::additive(
            self.operator_1_box.center,
            self.output_box.y,
            1.0
        );
    }

    fn update_operator_4_modulation_line(&mut self){
        let (through, to) = match self.operator_4_target {
            0 => (self.operator_4_mod_1_box.center, self.operator_1_box.center),
            1 => (self.operator_4_mod_2_box.center, self.operator_2_box.center),
            2 => (self.operator_4_mod_3_box.center, self.operator_3_box.center),
            _ => unreachable!(),
        };

        self.operator_4_modulation_line = OperatorLine::modulation(
            self.operator_4_box.center,
            through,
            to,
            1.0 - self.operator_4_additive as f32 
        );
    }

    fn update_operator_3_modulation_line(&mut self){
        let (through, to) = match self.operator_3_target {
            0 => (self.operator_3_mod_1_box.center, self.operator_1_box.center),
            1 => (self.operator_3_mod_2_box.center, self.operator_2_box.center),
            _ => unreachable!(),
        };

        self.operator_3_modulation_line = OperatorLine::modulation(
            self.operator_3_box.center,
            through,
            to,
            1.0 - self.operator_3_additive as f32 
        );
    }

    fn update_operator_2_modulation_line(&mut self){
        self.operator_2_modulation_line = OperatorLine::modulation(
            self.operator_2_box.center,
            self.operator_2_mod_1_box.center,
            self.operator_1_box.center,
            1.0 - self.operator_2_additive as f32 
        );
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_background(&self, frame: &mut Frame){
        let mut size = frame.size();

        size.width -= 2.0;
        size.height -= 2.0;

        let background = Path::rectangle(
            Point::new(1.0, 1.0),
            size
        );

        let stroke = Stroke::default()
            .with_width(1.0);

        frame.fill(&background, BACKGROUND_COLOR);
        frame.stroke(&background, stroke);
    }

    fn draw_lines(&self, frame: &mut Frame){
        self.operator_4_additive_line.draw(frame);
        self.operator_3_additive_line.draw(frame);
        self.operator_2_additive_line.draw(frame);
        self.operator_1_additive_line.draw(frame);

        self.operator_4_modulation_line.draw(frame);
        self.operator_3_modulation_line.draw(frame);
        self.operator_2_modulation_line.draw(frame);
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
            self.draw_background(frame);
            self.draw_lines(frame);
            self.draw_boxes(frame);
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let mod_boxes = vec![
            &mut self.operator_4_mod_3_box,
            &mut self.operator_4_mod_2_box,
            &mut self.operator_4_mod_1_box,
            &mut self.operator_3_mod_2_box,
            &mut self.operator_3_mod_1_box,
        ];

        for mod_box in mod_boxes.into_iter(){
            match mod_box.update(bounds, event){
                ModulationBoxChange::Update(message) => {
                    return (event::Status::Captured, Some(message));
                },
                ModulationBoxChange::ClearCache => {
                    self.cache.clear();

                    return (event::Status::Ignored, None);
                },
                _ => (),
            }
        }

        (event::Status::Ignored, None)
    }
}


fn scale_point(bounds: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - SCALE) * bounds.width / 2.0,
        y: (1.0 - SCALE) * bounds.height / 2.0
    };

    let scaled = Point {
        x: point.x * SCALE,
        y: point.y * SCALE,
    };

    scaled + translation
}


fn scale_size(size: Size) -> Size {
    Size::new(
        size.width * SCALE,
        size.height * SCALE,
    )
}