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
const ACTIVE_MOD_BOX_COLOR: (u8, u8, u8) = (27, 159, 31);

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


impl OperatorBox {
    fn new(bounds: Size, index: usize) -> Self {
        let (x, y) = match index {
            3 => (0, 0),
            2 => (2, 2),
            1 => (4, 4),
            0 => (6, 6),
            _ => unreachable!(),
        };

        let (base_top_left, base_size) = get_box_base_point_and_size(
            bounds, x, y
        );

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

        let (top_left, size) = get_box_base_point_and_size(bounds,x, y);

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
            let (r, g, b) = ACTIVE_MOD_BOX_COLOR;

            frame.fill(&self.path, Color::from_rgb8(r, g, b));
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


impl OutputBox {
    fn new(bounds: Size) -> Self {
        let (base_top_left, base_size) = get_box_base_point_and_size(
            bounds,
            0,
            7
        );

        let height = base_size.height * OPERATOR_BOX_SCALE;
        let width = base_size.width * 6.0 + base_size.width * OPERATOR_BOX_SCALE;

        let left = Point {
            x: base_top_left.x - (OPERATOR_BOX_SCALE - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (OPERATOR_BOX_SCALE - 1.0) * base_size.height / 2.0 + height,
        };
        let right = Point {
            x: left.x + width,
            y: left.y,
        };

        let left = scale_point(bounds, left);
        let right = scale_point(bounds, right);

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


struct ModulationMatrixParameters {
    operator_3_target: usize,
    operator_4_target: usize,
    operator_2_additive: f64,
    operator_3_additive: f64,
    operator_4_additive: f64,
}


impl ModulationMatrixParameters {
    fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let operator_3_target = Self::convert_operator_3_target(
            sync_handle.get_parameter(33)
        );
        let operator_4_target = Self::convert_operator_4_target(
            sync_handle.get_parameter(48)
        );
        let operator_2_additive = sync_handle.get_parameter(18);
        let operator_3_additive = sync_handle.get_parameter(32);
        let operator_4_additive = sync_handle.get_parameter(47);

        Self {
            operator_3_target,
            operator_4_target,
            operator_2_additive,
            operator_3_additive,
            operator_4_additive,
        }
    }

    fn convert_operator_3_target(value: f64) -> usize {
        ProcessingParameterOperatorModulationTarget2::to_processing(value)
    }

    fn convert_operator_4_target(value: f64) -> usize {
        ProcessingParameterOperatorModulationTarget3::to_processing(value)
    }
}


struct ModulationMatrixComponents {
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


impl ModulationMatrixComponents {
    fn new(
        parameters: &ModulationMatrixParameters,
        bounds: Size
    ) -> Self {
        let operator_1_box = OperatorBox::new(bounds, 0);
        let operator_2_box = OperatorBox::new(bounds, 1);
        let operator_3_box = OperatorBox::new(bounds, 2);
        let operator_4_box = OperatorBox::new(bounds, 3);

        let operator_4_mod_3_box = ModulationBox::new(
            bounds,
            3,
            2,
            parameters.operator_4_target == 2,
            Some(Message::ParameterChange(48, 1.0)),
        );
        let operator_4_mod_2_box = ModulationBox::new(
            bounds,
            3,
            1,
            parameters.operator_4_target == 1,
            Some(Message::ParameterChange(48, 0.5)),
        );
        let operator_4_mod_1_box = ModulationBox::new(
            bounds,
            3,
            0,
            parameters.operator_4_target == 0,
            Some(Message::ParameterChange(48, 0.0)),
        );
        let operator_3_mod_2_box = ModulationBox::new(
            bounds,
            2,
            1,
            parameters.operator_3_target == 1,
            Some(Message::ParameterChange(33, 1.0)),
        );
        let operator_3_mod_1_box = ModulationBox::new(
            bounds,
            2,
            0,
            parameters.operator_3_target == 0,
            Some(Message::ParameterChange(33, 0.0)),
        );
        let operator_2_mod_1_box = ModulationBox::new(
            bounds,
            1,
            0,
            true,
            None,
        );

        let output_box = OutputBox::new(bounds);

        let operator_4_additive_line = OperatorLine::additive(
            operator_4_box.center,
            output_box.y,
            parameters.operator_4_additive as f32,
        );
        let operator_3_additive_line = OperatorLine::additive(
            operator_3_box.center,
            output_box.y,
            parameters.operator_3_additive as f32,
        );
        let operator_2_additive_line = OperatorLine::additive(
            operator_2_box.center,
            output_box.y,
            parameters.operator_2_additive as f32,
        );
        let operator_1_additive_line = OperatorLine::additive(
            operator_1_box.center,
            output_box.y,
            1.0
        );

        let operator_4_modulation_line = {
            let (through, to) = match parameters.operator_4_target {
                0 => (operator_4_mod_1_box.center, operator_1_box.center),
                1 => (operator_4_mod_2_box.center, operator_2_box.center),
                2 => (operator_4_mod_3_box.center, operator_3_box.center),
                _ => unreachable!(),
            };
    
            OperatorLine::modulation(
                operator_4_box.center,
                through,
                to,
                1.0 - parameters.operator_4_additive as f32 
            )
        };

        let operator_3_modulation_line = {
            let (through, to) = match parameters.operator_3_target {
                0 => (operator_3_mod_1_box.center, operator_1_box.center),
                1 => (operator_3_mod_2_box.center, operator_2_box.center),
                _ => unreachable!(),
            };
    
            OperatorLine::modulation(
                operator_3_box.center,
                through,
                to,
                1.0 - parameters.operator_3_additive as f32 
            )
        };

        let operator_2_modulation_line = OperatorLine::modulation(
            operator_2_box.center,
            operator_2_mod_1_box.center,
            operator_1_box.center,
            1.0 - parameters.operator_2_additive as f32 
        );

        Self {
            operator_1_box,
            operator_2_box,
            operator_3_box,
            operator_4_box,
            operator_4_mod_3_box,
            operator_4_mod_2_box,
            operator_4_mod_1_box,
            operator_3_mod_2_box,
            operator_3_mod_1_box,
            operator_2_mod_1_box,
            output_box,
            operator_4_additive_line,
            operator_3_additive_line,
            operator_2_additive_line,
            operator_1_additive_line,
            operator_4_modulation_line,
            operator_3_modulation_line,
            operator_2_modulation_line,
        }
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


pub struct ModulationMatrix {
    cache: Cache,
    size: Size,
    parameters: ModulationMatrixParameters,
    components: ModulationMatrixComponents,
}


impl ModulationMatrix {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let parameters = ModulationMatrixParameters::new(sync_handle);
        let components = ModulationMatrixComponents::new(&parameters, SIZE);

        Self {
            cache: Cache::default(),
            size: SIZE,
            parameters,
            components,
        }
    }

    pub fn set_operator_3_target(&mut self, value: f64){
        self.parameters.operator_3_target =
            ModulationMatrixParameters::convert_operator_3_target(value);

        self.update_components();
    }

    pub fn set_operator_4_target(&mut self, value: f64){
        self.parameters.operator_4_target =
            ModulationMatrixParameters::convert_operator_4_target(value);

        self.update_components();
    }

    pub fn set_operator_4_additive(&mut self, value: f64){
        self.parameters.operator_4_additive = value;

        self.update_components();
    }

    pub fn set_operator_3_additive(&mut self, value: f64){
        self.parameters.operator_3_additive = value;

        self.update_components();
    }

    pub fn set_operator_2_additive(&mut self, value: f64){
        self.parameters.operator_2_additive = value;

        self.update_components();
    }

    fn update_components(&mut self){
        self.components = ModulationMatrixComponents::new(
            &self.parameters,
            self.size
        );

        self.cache.clear();
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
}


impl Program<Message> for ModulationMatrix {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame);

            self.components.draw_lines(frame);
            self.components.draw_boxes(frame);
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
            &mut self.components.operator_4_mod_3_box,
            &mut self.components.operator_4_mod_2_box,
            &mut self.components.operator_4_mod_1_box,
            &mut self.components.operator_3_mod_2_box,
            &mut self.components.operator_3_mod_1_box,
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


fn get_box_base_point_and_size(
    bounds: Size,
    x: usize,
    y: usize
) -> (Point, Size) {
    let x_bla = bounds.width / 7.0;
    let y_bla = bounds.height / 8.0;

    let base_top_left = Point::new(
        x as f32 * x_bla,
        y as f32 * y_bla,
    );

    let base_size = Size::new(x_bla, y_bla);

    (base_top_left, base_size)
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
