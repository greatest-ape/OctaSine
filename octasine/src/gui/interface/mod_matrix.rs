use iced_baseview::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text, path, event
};
use iced_baseview::{
    Element, Color, Rectangle, Point, Length, Vector, Size, mouse
};

use crate::GuiSyncHandle;
use crate::parameters::values::{
    ParameterValue,
    Operator3ModulationTargetValue,
    Operator4ModulationTargetValue,
};

use super::{FONT_BOLD, FONT_SIZE, LINE_HEIGHT, Message, SnapPoint};


const BACKGROUND_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);
const ACTIVE_MOD_BOX_COLOR: (u8, u8, u8) = (0, 0, 0);

pub const HEIGHT: u16 = LINE_HEIGHT * 7;
const SMALL_BOX_SIZE: u16 = 10;
const BIG_BOX_SIZE: u16 = LINE_HEIGHT;

// Calculated from the constants above
const SCALE: f32 = SMALL_BOX_SIZE as f32 / (HEIGHT as f32 / 8.0);
const WIDTH_FLOAT: f32 = ((HEIGHT as f64 / 8.0) * 7.0) as f32;
const SIZE: Size = Size { width: WIDTH_FLOAT, height: HEIGHT as f32 };
const OPERATOR_BOX_SCALE: f32 = BIG_BOX_SIZE as f32 / SMALL_BOX_SIZE as f32;
const WIDTH: u16 = WIDTH_FLOAT as u16 + 2;


struct OperatorBox {
    text: Text,
    path: Path,
    center: Point,
    feedback_center: Point,
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

        let mut top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        top_left.x += 1.0;
        top_left = top_left.snap();

        let path = Path::rectangle(top_left, size);
        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let half_box_size = (BIG_BOX_SIZE as f32) / 2.0;

        let feedback_center = Point {
            x: center.x + half_box_size - 1.0,
            y: center.y - half_box_size,
        };

        let text_position = Point {
            x: base_top_left.x,
            y: base_top_left.y,
        };

        let mut text_position = scale_point(bounds, text_position);

        text_position = text_position.snap();

        text_position.x += 2.0;
        text_position.y -= 2.0;

        let text = Text {
            content: format!("{}", index + 1),
            position: text_position,
            font: FONT_BOLD,
            size: FONT_SIZE as f32,
            ..Default::default()
        };

        Self {
            text,
            path,
            center,
            feedback_center,
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

        let mut top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        top_left.x += 1.0;

        top_left = top_left.snap();

        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let path = Path::circle(center, size.width / 2.0);

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

        let mut left = scale_point(bounds, left);
        let mut right = scale_point(bounds, right);

        left.x += 1.0;
        right.x += 1.0;

        left = left.snap();
        right = right.snap();

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
    stroke: Stroke,
}


impl OperatorLine {
    fn additive(from: Point, to_y: f32, opacity: f32) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from.snap(), to.snap());
        let stroke = Stroke::default()
            .with_width(3.0)
            .with_color(Color::from_rgba(0.0, 0.0, 0.0, opacity));

        Self {
            path,
            stroke,
        }
    }

    fn modulation(
        from: Point,
        through: Point,
        to: Point,
        opacity: f32,
    ) -> Self {
        let mut builder = path::Builder::new();

        builder.move_to(from.snap());
        builder.line_to(through.snap());
        builder.line_to(to.snap());

        let path = builder.build();
        let stroke = Stroke::default()
            .with_width(3.0)
            .with_color(Color::from_rgba(0.0, 0.0, 0.0, opacity));

        Self {
            path,
            stroke,
        }
    }

    fn feedback(
        center: Point,
        opacity: f32
    ) -> Self {
        let path = Path::circle(center, 4.0);
        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgba(0.0, 0.0, 0.0, opacity));

        Self {
            path,
            stroke,
        }
    }

    fn draw(&self, frame: &mut Frame){
        frame.stroke(&self.path, self.stroke);
    }
}


struct ModulationMatrixParameters {
    operator_3_target: usize,
    operator_4_target: usize,
    operator_2_additive: f64,
    operator_3_additive: f64,
    operator_4_additive: f64,
    operator_1_feedback: f64,
    operator_2_feedback: f64,
    operator_3_feedback: f64,
    operator_4_feedback: f64,
    operator_1_volume: f64,
    operator_2_volume: f64,
    operator_3_volume: f64,
    operator_4_volume: f64,
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

        let operator_1_feedback = sync_handle.get_parameter(6);
        let operator_2_feedback = sync_handle.get_parameter(20);
        let operator_3_feedback = sync_handle.get_parameter(35);
        let operator_4_feedback = sync_handle.get_parameter(50);

        let operator_1_volume = Self::convert_volume(sync_handle.get_parameter(2));
        let operator_2_volume = Self::convert_volume(sync_handle.get_parameter(15));
        let operator_3_volume = Self::convert_volume(sync_handle.get_parameter(29));
        let operator_4_volume = Self::convert_volume(sync_handle.get_parameter(44));

        Self {
            operator_3_target,
            operator_4_target,
            operator_2_additive,
            operator_3_additive,
            operator_4_additive,
            operator_1_feedback,
            operator_2_feedback,
            operator_3_feedback,
            operator_4_feedback,
            operator_1_volume,
            operator_2_volume,
            operator_3_volume,
            operator_4_volume,
        }
    }

    fn convert_operator_3_target(value: f64) -> usize {
        Operator3ModulationTargetValue::from_sync(value).0
    }

    fn convert_operator_4_target(value: f64) -> usize {
        Operator4ModulationTargetValue::from_sync(value).get()
    }

    fn convert_volume(value: f64) -> f64 {
        value.min(0.5) * 2.0
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
    operator_1_feedback_line: OperatorLine,
    operator_2_feedback_line: OperatorLine,
    operator_3_feedback_line: OperatorLine,
    operator_4_feedback_line: OperatorLine,
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
            (parameters.operator_4_additive * parameters.operator_4_volume) as f32,
        );
        let operator_3_additive_line = OperatorLine::additive(
            operator_3_box.center,
            output_box.y,
            (parameters.operator_3_additive * parameters.operator_3_volume) as f32,
        );
        let operator_2_additive_line = OperatorLine::additive(
            operator_2_box.center,
            output_box.y,
            (parameters.operator_2_additive * parameters.operator_2_volume) as f32,
        );
        let operator_1_additive_line = OperatorLine::additive(
            operator_1_box.center,
            output_box.y,
            (1.0 * parameters.operator_1_volume) as f32
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
                ((1.0 - parameters.operator_4_additive) * parameters.operator_4_volume) as f32
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
                ((1.0 - parameters.operator_3_additive) * parameters.operator_3_volume) as f32
            )
        };

        let operator_2_modulation_line = OperatorLine::modulation(
            operator_2_box.center,
            operator_2_mod_1_box.center,
            operator_1_box.center,
            ((1.0 - parameters.operator_2_additive) * parameters.operator_2_volume) as f32
        );

        let operator_4_feedback_line = OperatorLine::feedback(
            operator_4_box.feedback_center,
            parameters.operator_4_feedback as f32,
        );
        let operator_3_feedback_line = OperatorLine::feedback(
            operator_3_box.feedback_center,
            parameters.operator_3_feedback as f32,
        );
        let operator_2_feedback_line = OperatorLine::feedback(
            operator_2_box.feedback_center,
            parameters.operator_2_feedback as f32,
        );
        let operator_1_feedback_line = OperatorLine::feedback(
            operator_1_box.feedback_center,
            parameters.operator_1_feedback as f32,
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
            operator_4_feedback_line,
            operator_3_feedback_line,
            operator_2_feedback_line,
            operator_1_feedback_line,
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

        // self.operator_4_feedback_line.draw(frame);
        // self.operator_3_feedback_line.draw(frame);
        // self.operator_2_feedback_line.draw(frame);
        // self.operator_1_feedback_line.draw(frame);
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

    pub fn set_operator_4_feedback(&mut self, value: f64){
        self.parameters.operator_4_feedback = value;

        self.update_components();
    }

    pub fn set_operator_3_feedback(&mut self, value: f64){
        self.parameters.operator_3_feedback = value;

        self.update_components();
    }

    pub fn set_operator_2_feedback(&mut self, value: f64){
        self.parameters.operator_2_feedback = value;

        self.update_components();
    }

    pub fn set_operator_1_feedback(&mut self, value: f64){
        self.parameters.operator_1_feedback = value;

        self.update_components();
    }

    pub fn set_operator_4_volume(&mut self, value: f64){
        let converted = ModulationMatrixParameters::convert_volume(value);

        self.parameters.operator_4_volume = converted;

        self.update_components();
    }

    pub fn set_operator_3_volume(&mut self, value: f64){
        let converted = ModulationMatrixParameters::convert_volume(value);

        self.parameters.operator_3_volume = converted;

        self.update_components();
    }

    pub fn set_operator_2_volume(&mut self, value: f64){
        let converted = ModulationMatrixParameters::convert_volume(value);

        self.parameters.operator_2_volume = converted;

        self.update_components();
    }

    pub fn set_operator_1_volume(&mut self, value: f64){
        let converted = ModulationMatrixParameters::convert_volume(value);

        self.parameters.operator_1_volume = converted;

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

        size.width -= 1.0;
        size.height -= 1.0;

        let background = Path::rectangle(
            Point::new(0.5, 0.5),
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
