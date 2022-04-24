use std::marker::PhantomData;

use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{mouse, Color, Element, Length, Point, Rectangle, Size, Vector};
use palette::gradient::Gradient;
use palette::Srgba;

use crate::common::{ModTarget, ModTargetStorage};
use crate::parameters::values::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
    ParameterValue,
};
use crate::GuiSyncHandle;

use super::style::Theme;
use super::{Message, SnapPoint, FONT_BOLD, FONT_SIZE, LINE_HEIGHT};

pub const HEIGHT: u16 = LINE_HEIGHT * 7;
const SMALL_BOX_SIZE: u16 = 8;
const BIG_BOX_SIZE: u16 = LINE_HEIGHT;

// Calculated from the constants above
const SCALE: f32 = SMALL_BOX_SIZE as f32 / (HEIGHT as f32 / 8.0);
const WIDTH_FLOAT: f32 = ((HEIGHT as f64 / 8.0) * 7.0) as f32;
const SIZE: Size = Size {
    width: WIDTH_FLOAT,
    height: HEIGHT as f32,
};
const OPERATOR_BOX_SCALE: f32 = BIG_BOX_SIZE as f32 / SMALL_BOX_SIZE as f32;
const WIDTH: u16 = WIDTH_FLOAT as u16 + 2;

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub box_border_color: Color,
    pub operator_box_color_active: Color,
    pub operator_box_color_hover: Color,
    pub operator_box_color_dragging: Color,
    pub modulation_box_color_active: Color,
    pub modulation_box_color_inactive: Color,
    pub line_max_color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
}

enum BoxStatus {
    Normal,
    Hover,
    Dragging { from: Point, original_value: f64 },
}

impl BoxStatus {
    fn is_dragging(&self) -> bool {
        matches!(self, BoxStatus::Dragging { .. })
    }
}

struct OperatorBox {
    index: usize,
    text: Text,
    path: Path,
    center: Point,
    status: BoxStatus,
    last_cursor_position: Point,
    hitbox: Rectangle,
}

impl OperatorBox {
    fn new(bounds: Size, index: usize, style_sheet: Box<dyn StyleSheet>) -> Self {
        let (x, y) = match index {
            3 => (0, 0),
            2 => (2, 2),
            1 => (4, 4),
            0 => (6, 6),
            _ => unreachable!(),
        };

        let (base_top_left, base_size) = get_box_base_point_and_size(bounds, x, y);

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
            color: style_sheet.active().text_color,
            ..Default::default()
        };

        Self {
            index,
            text,
            path,
            center,
            status: BoxStatus::Normal,
            last_cursor_position: Point::new(-1.0, -1.0),
            hitbox: rect,
        }
    }

    fn get_parameter_index(&self) -> usize {
        match self.index {
            1 => 22,
            2 => 38,
            3 => 54,
            _ => unreachable!(),
        }
    }

    fn update(&mut self, bounds: Rectangle, event: event::Event, value: f64) -> OperatorBoxChange {
        if self.index == 0 {
            return OperatorBoxChange::None;
        }

        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                self.last_cursor_position = cursor;

                let hit = self.hitbox.contains(cursor);

                match self.status {
                    BoxStatus::Normal if hit => {
                        self.status = BoxStatus::Hover;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Hover if !hit => {
                        self.status = BoxStatus::Normal;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Dragging {
                        from,
                        original_value,
                    } => {
                        let change = -(cursor.y - from.y) as f64 / 100.0;

                        return OperatorBoxChange::Update(Message::ChangeSingleParameterSetValue(
                            self.get_parameter_index(),
                            (original_value + change).max(0.0).min(1.0),
                        ));
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if !self.status.is_dragging() && self.hitbox.contains(self.last_cursor_position) {
                    self.status = BoxStatus::Dragging {
                        from: self.last_cursor_position,
                        original_value: value,
                    };

                    return OperatorBoxChange::ClearCache(Some(
                        Message::ChangeSingleParameterBegin(self.get_parameter_index()),
                    ));
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if self.status.is_dragging() {
                    if self.hitbox.contains(self.last_cursor_position) {
                        self.status = BoxStatus::Hover;
                    } else {
                        self.status = BoxStatus::Normal;
                    }

                    return OperatorBoxChange::ClearCache(Some(Message::ChangeSingleParameterEnd(
                        self.get_parameter_index(),
                    )));
                }
            }
            _ => (),
        }

        OperatorBoxChange::None
    }

    fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let stroke = Stroke::default()
            .with_color(style.box_border_color)
            .with_width(1.0);

        let background_color = match self.status {
            BoxStatus::Normal => style.operator_box_color_active,
            BoxStatus::Hover => style.operator_box_color_hover,
            BoxStatus::Dragging { .. } => style.operator_box_color_dragging,
        };

        frame.fill(&self.path, background_color);
        frame.stroke(&self.path, stroke);
        frame.fill_text(self.text.clone());
    }
}

enum ModulationBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

enum OperatorBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

trait ModulationBoxUpdate {
    fn update(&mut self, bounds: Rectangle, event: event::Event) -> ModulationBoxChange;
}

struct ModulationBox<P, V> {
    path: Path,
    center: Point,
    rect: Rectangle,
    hover: bool,
    click_started: bool,
    parameter_index: usize,
    target_index: usize,
    v: V,
    _phantom_data: PhantomData<P>,
}

impl<P, V> ModulationBox<P, V>
where
    P: ParameterValue<Value = V>,
    V: ModTarget,
{
    fn new(
        bounds: Size,
        from: usize,
        to: usize,
        parameter_index: usize,
        target_index: usize,
        v: V,
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

        let (top_left, size) = get_box_base_point_and_size(bounds, x, y);

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
            hover: false,
            click_started: false,
            parameter_index,
            target_index,
            v,
            _phantom_data: Default::default(),
        }
    }

    fn active(&self) -> bool {
        self.v.index_active(self.target_index)
    }

    fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let stroke = Stroke::default()
            .with_color(style.box_border_color)
            .with_width(1.0);

        if self.active() || self.hover {
            frame.fill(&self.path, style.modulation_box_color_active);
        } else {
            frame.fill(&self.path, style.modulation_box_color_inactive);
        }

        frame.stroke(&self.path, stroke);
    }
}

impl<P, V> ModulationBoxUpdate for ModulationBox<P, V>
where
    P: ParameterValue<Value = V>,
    V: ModTarget,
{
    fn update(&mut self, bounds: Rectangle, event: event::Event) -> ModulationBoxChange {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                match (self.hover, self.rect.contains(cursor)) {
                    (false, true) => {
                        self.hover = true;

                        return ModulationBoxChange::ClearCache(None);
                    }
                    (true, false) => {
                        self.hover = false;

                        return ModulationBoxChange::ClearCache(None);
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if self.hover {
                    self.click_started = true;
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                if self.hover && self.click_started {
                    self.click_started = false;

                    self.v.set_index(self.target_index, !self.active());
                    let sync_value = P::from_processing(self.v).to_sync();

                    return ModulationBoxChange::Update(Message::ChangeSingleParameterImmediate(
                        self.parameter_index,
                        sync_value,
                    ));
                }
            }
            _ => (),
        }

        ModulationBoxChange::None
    }
}

struct OutputBox {
    path: Path,
    y: f32,
}

impl OutputBox {
    fn new(bounds: Size) -> Self {
        let (base_top_left, base_size) = get_box_base_point_and_size(bounds, 0, 7);

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

        Self { path, y: left.y }
    }

    fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let stroke = Stroke::default()
            .with_color(style_sheet.active().box_border_color)
            .with_width(1.0);

        frame.stroke(&self.path, stroke);
    }
}

struct AdditiveLine {
    path: Path,
    color: Color,
}

impl AdditiveLine {
    fn new(from: Point, to_y: f32, additive: f64, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut to = from;

        to.y = to_y;

        let path = Path::line(from.snap(), to.snap());

        let mut line = Self {
            path,
            color: style_sheet.active().line_max_color,
        };

        line.update(additive, style_sheet);

        line
    }

    fn update(&mut self, additive: f64, style_sheet: Box<dyn StyleSheet>) {
        self.color = Self::calculate_color(additive, style_sheet);
    }

    fn calculate_color(additive: f64, style_sheet: Box<dyn StyleSheet>) -> Color {
        let bg = style_sheet.active().background_color;
        let c = style_sheet.active().line_max_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            Srgba::new(0.23, 0.69, 0.06, 1.0).into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(additive as f32);

        Color::from(Srgba::from_linear(color))
    }

    fn draw(&self, frame: &mut Frame) {
        let stroke = Stroke::default().with_width(3.0).with_color(self.color);

        frame.stroke(&self.path, stroke);
    }
}

struct ModulationLine {
    from: Point,
    points: Vec<Point>,
    color: Color,
}

impl ModulationLine {
    fn new(from: Point, mod_index: f64, style_sheet: Box<dyn StyleSheet>) -> Self {
        let mut line = Self {
            from,
            points: vec![],
            color: Color::TRANSPARENT,
        };

        line.update(vec![], style_sheet, mod_index);

        line
    }

    fn update(&mut self, points: Vec<Point>, style_sheet: Box<dyn StyleSheet>, mod_index: f64) {
        let bg = style_sheet.active().background_color;
        let c = style_sheet.active().line_max_color;

        let gradient = Gradient::new(vec![
            Srgba::new(bg.r, bg.g, bg.b, 1.0).into_linear(),
            Srgba::new(0.25, 0.5, 1.0, 1.0).into_linear(),
            Srgba::new(c.r, c.g, c.b, 1.0).into_linear(),
        ]);

        let color = gradient.get(mod_index as f32);
        let color = Color::from(Srgba::from_linear(color));

        self.points = points;
        self.color = color;
    }

    fn draw(&self, frame: &mut Frame) {
        let stroke = Stroke::default().with_width(3.0).with_color(self.color);

        let mut builder = path::Builder::new();

        builder.move_to(self.from.snap());

        for point in self.points.iter() {
            builder.line_to(point.snap());
        }

        let path = builder.build();

        frame.stroke(&path, stroke);
    }
}

struct ModulationMatrixParameters {
    operator_2_targets: ModTargetStorage<1>,
    operator_3_targets: ModTargetStorage<2>,
    operator_4_targets: ModTargetStorage<3>,
    operator_1_mix: f64,
    operator_2_mix: f64,
    operator_3_mix: f64,
    operator_4_mix: f64,
    operator_2_mod: f64,
    operator_3_mod: f64,
    operator_4_mod: f64,
}

impl ModulationMatrixParameters {
    fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let operator_2_targets =
            Operator2ModulationTargetValue::from_sync(sync_handle.get_parameter(21)).get();
        let operator_3_targets =
            Operator3ModulationTargetValue::from_sync(sync_handle.get_parameter(37)).get();
        let operator_4_targets =
            Operator4ModulationTargetValue::from_sync(sync_handle.get_parameter(53)).get();

        let operator_1_mix = sync_handle.get_parameter(4);
        let operator_2_mix = sync_handle.get_parameter(18);
        let operator_3_mix = sync_handle.get_parameter(34);
        let operator_4_mix = sync_handle.get_parameter(50);

        let operator_2_mod = sync_handle.get_parameter(22);
        let operator_3_mod = sync_handle.get_parameter(38);
        let operator_4_mod = sync_handle.get_parameter(54);

        Self {
            operator_2_targets,
            operator_3_targets,
            operator_4_targets,
            operator_1_mix,
            operator_2_mix,
            operator_3_mix,
            operator_4_mix,
            operator_2_mod,
            operator_3_mod,
            operator_4_mod,
        }
    }
}

struct ModulationMatrixComponents {
    operator_1_box: OperatorBox,
    operator_2_box: OperatorBox,
    operator_3_box: OperatorBox,
    operator_4_box: OperatorBox,
    operator_4_mod_3_box: ModulationBox<Operator4ModulationTargetValue, ModTargetStorage<3>>,
    operator_4_mod_2_box: ModulationBox<Operator4ModulationTargetValue, ModTargetStorage<3>>,
    operator_4_mod_1_box: ModulationBox<Operator4ModulationTargetValue, ModTargetStorage<3>>,
    operator_3_mod_2_box: ModulationBox<Operator3ModulationTargetValue, ModTargetStorage<2>>,
    operator_3_mod_1_box: ModulationBox<Operator3ModulationTargetValue, ModTargetStorage<2>>,
    operator_2_mod_1_box: ModulationBox<Operator2ModulationTargetValue, ModTargetStorage<1>>,
    output_box: OutputBox,
    operator_4_additive_line: AdditiveLine,
    operator_3_additive_line: AdditiveLine,
    operator_2_additive_line: AdditiveLine,
    operator_1_additive_line: AdditiveLine,
    operator_4_modulation_line: ModulationLine,
    operator_3_modulation_line: ModulationLine,
    operator_2_modulation_line: ModulationLine,
}

impl ModulationMatrixComponents {
    fn new(parameters: &ModulationMatrixParameters, bounds: Size, style: Theme) -> Self {
        let operator_1_box = OperatorBox::new(bounds, 0, style.into());
        let operator_2_box = OperatorBox::new(bounds, 1, style.into());
        let operator_3_box = OperatorBox::new(bounds, 2, style.into());
        let operator_4_box = OperatorBox::new(bounds, 3, style.into());

        let operator_4_mod_3_box =
            ModulationBox::new(bounds, 3, 2, 53, 2, parameters.operator_4_targets);
        let operator_4_mod_2_box =
            ModulationBox::new(bounds, 3, 1, 53, 1, parameters.operator_4_targets);
        let operator_4_mod_1_box =
            ModulationBox::new(bounds, 3, 0, 53, 0, parameters.operator_4_targets);
        let operator_3_mod_2_box =
            ModulationBox::new(bounds, 2, 1, 37, 1, parameters.operator_3_targets);
        let operator_3_mod_1_box =
            ModulationBox::new(bounds, 2, 0, 37, 0, parameters.operator_3_targets);
        let operator_2_mod_1_box =
            ModulationBox::new(bounds, 1, 0, 21, 0, parameters.operator_2_targets);

        let output_box = OutputBox::new(bounds);

        let operator_4_additive_line = AdditiveLine::new(
            operator_4_box.center,
            output_box.y,
            parameters.operator_4_mix,
            style.into(),
        );
        let operator_3_additive_line = AdditiveLine::new(
            operator_3_box.center,
            output_box.y,
            parameters.operator_3_mix,
            style.into(),
        );
        let operator_2_additive_line = AdditiveLine::new(
            operator_2_box.center,
            output_box.y,
            parameters.operator_2_mix,
            style.into(),
        );
        let operator_1_additive_line = AdditiveLine::new(
            operator_1_box.center,
            output_box.y,
            parameters.operator_1_mix,
            style.into(),
        );

        let operator_4_modulation_line = ModulationLine::new(
            operator_4_box.center,
            parameters.operator_4_mod,
            style.into(),
        );
        let operator_3_modulation_line = ModulationLine::new(
            operator_3_box.center,
            parameters.operator_3_mod,
            style.into(),
        );
        let operator_2_modulation_line = ModulationLine::new(
            operator_2_box.center,
            parameters.operator_2_mod,
            style.into(),
        );

        let mut components = Self {
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
        };

        components.update(parameters, style);

        components
    }

    fn update(&mut self, parameters: &ModulationMatrixParameters, style: Theme) {
        self.operator_4_mod_3_box.v = parameters.operator_4_targets;
        self.operator_4_mod_2_box.v = parameters.operator_4_targets;
        self.operator_4_mod_1_box.v = parameters.operator_4_targets;
        self.operator_3_mod_2_box.v = parameters.operator_3_targets;
        self.operator_3_mod_1_box.v = parameters.operator_3_targets;

        self.operator_4_additive_line
            .update(parameters.operator_4_mix, style.into());

        self.operator_3_additive_line
            .update(parameters.operator_3_mix, style.into());
        self.operator_2_additive_line
            .update(parameters.operator_2_mix, style.into());
        self.operator_1_additive_line
            .update(parameters.operator_1_mix, style.into());

        {
            let mut points = Vec::new();

            for mod_target in parameters.operator_4_targets.active_indices() {
                let (mod_box, operator_box) = match mod_target {
                    0 => (self.operator_4_mod_1_box.center, self.operator_1_box.center),
                    1 => (self.operator_4_mod_2_box.center, self.operator_2_box.center),
                    2 => (self.operator_4_mod_3_box.center, self.operator_3_box.center),
                    _ => unreachable!(),
                };

                points.push(mod_box);
                points.push(operator_box);
                points.push(mod_box);
            }

            self.operator_4_modulation_line
                .update(points, style.into(), parameters.operator_4_mod);
        }

        {
            let mut points = Vec::new();

            for mod_target in parameters.operator_3_targets.active_indices() {
                let (mod_box, operator_box) = match mod_target {
                    0 => (self.operator_3_mod_1_box.center, self.operator_1_box.center),
                    1 => (self.operator_3_mod_2_box.center, self.operator_2_box.center),
                    _ => unreachable!(),
                };

                points.push(mod_box);
                points.push(operator_box);
                points.push(mod_box);
            }

            self.operator_3_modulation_line
                .update(points, style.into(), parameters.operator_3_mod);
        };

        {
            let mut points = Vec::new();

            for mod_target in parameters.operator_2_targets.active_indices() {
                let (mod_box, operator_box) = match mod_target {
                    0 => (self.operator_2_mod_1_box.center, self.operator_1_box.center),
                    _ => unreachable!(),
                };

                points.push(mod_box);
                points.push(operator_box);
                points.push(mod_box);
            }

            self.operator_2_modulation_line
                .update(points, style.into(), parameters.operator_2_mod);
        }
    }

    fn draw_lines(&self, frame: &mut Frame) {
        self.operator_4_additive_line.draw(frame);
        self.operator_3_additive_line.draw(frame);
        self.operator_2_additive_line.draw(frame);
        self.operator_1_additive_line.draw(frame);

        self.operator_4_modulation_line.draw(frame);
        self.operator_3_modulation_line.draw(frame);
        self.operator_2_modulation_line.draw(frame);
    }

    fn draw_boxes(&self, frame: &mut Frame, style: Theme) {
        self.operator_1_box.draw(frame, style.into());
        self.operator_2_box.draw(frame, style.into());
        self.operator_3_box.draw(frame, style.into());
        self.operator_4_box.draw(frame, style.into());

        self.operator_4_mod_3_box.draw(frame, style.into());
        self.operator_4_mod_2_box.draw(frame, style.into());
        self.operator_4_mod_1_box.draw(frame, style.into());
        self.operator_3_mod_2_box.draw(frame, style.into());
        self.operator_3_mod_1_box.draw(frame, style.into());
        self.operator_2_mod_1_box.draw(frame, style.into());

        self.output_box.draw(frame, style.into());
    }
}

pub struct ModulationMatrix {
    cache: Cache,
    style: Theme,
    parameters: ModulationMatrixParameters,
    components: ModulationMatrixComponents,
}

impl ModulationMatrix {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, style: Theme) -> Self {
        let parameters = ModulationMatrixParameters::new(sync_handle);
        let components = ModulationMatrixComponents::new(&parameters, SIZE, style);

        Self {
            cache: Cache::default(),
            style,
            parameters,
            components,
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.update_components();

        self.cache.clear();
    }

    pub fn set_operator_2_target(&mut self, value: f64) {
        self.parameters.operator_2_targets = Operator2ModulationTargetValue::from_sync(value).get();

        self.update_components();
    }

    pub fn set_operator_3_target(&mut self, value: f64) {
        self.parameters.operator_3_targets = Operator3ModulationTargetValue::from_sync(value).get();

        self.update_components();
    }

    pub fn set_operator_4_target(&mut self, value: f64) {
        self.parameters.operator_4_targets = Operator4ModulationTargetValue::from_sync(value).get();

        self.update_components();
    }

    pub fn set_operator_4_mod(&mut self, value: f64) {
        self.parameters.operator_4_mod = value;

        self.update_components();
    }

    pub fn set_operator_3_mod(&mut self, value: f64) {
        self.parameters.operator_3_mod = value;

        self.update_components();
    }

    pub fn set_operator_2_mod(&mut self, value: f64) {
        self.parameters.operator_2_mod = value;

        self.update_components();
    }

    pub fn set_operator_4_mix(&mut self, value: f64) {
        self.parameters.operator_4_mix = value;

        self.update_components();
    }

    pub fn set_operator_3_mix(&mut self, value: f64) {
        self.parameters.operator_3_mix = value;

        self.update_components();
    }

    pub fn set_operator_2_mix(&mut self, value: f64) {
        self.parameters.operator_2_mix = value;

        self.update_components();
    }

    pub fn set_operator_1_mix(&mut self, value: f64) {
        self.parameters.operator_1_mix = value;

        self.update_components();
    }

    fn update_components(&mut self) {
        self.components.update(&self.parameters, self.style);

        self.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_background(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let mut size = frame.size();
        let style = style_sheet.active();

        size.width -= 1.0;
        size.height -= 1.0;

        let background = Path::rectangle(Point::new(0.5, 0.5), size);

        let stroke = Stroke::default()
            .with_color(style.border_color)
            .with_width(1.0);

        frame.fill(&background, style.background_color);
        frame.stroke(&background, stroke);
    }
}

impl Program<Message> for ModulationMatrix {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, self.style.into());

            self.components.draw_lines(frame);
            self.components.draw_boxes(frame, self.style);
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        macro_rules! update_mod_box {
            ($mod_box:expr) => {
                match $mod_box.update(bounds, event) {
                    ModulationBoxChange::Update(message) => {
                        return (event::Status::Captured, Some(message));
                    }
                    ModulationBoxChange::ClearCache(opt_message) => {
                        self.cache.clear();

                        return (event::Status::Ignored, opt_message);
                    }
                    ModulationBoxChange::None => (),
                }
            };
        }

        update_mod_box!(self.components.operator_4_mod_3_box);
        update_mod_box!(self.components.operator_4_mod_2_box);
        update_mod_box!(self.components.operator_4_mod_1_box);
        update_mod_box!(self.components.operator_3_mod_2_box);
        update_mod_box!(self.components.operator_3_mod_1_box);
        update_mod_box!(self.components.operator_2_mod_1_box);

        let operator_boxes = vec![
            (&mut self.components.operator_1_box, 0.0),
            (
                &mut self.components.operator_2_box,
                self.parameters.operator_2_mod,
            ),
            (
                &mut self.components.operator_3_box,
                self.parameters.operator_3_mod,
            ),
            (
                &mut self.components.operator_4_box,
                self.parameters.operator_4_mod,
            ),
        ];

        for (operator_box, value) in operator_boxes.into_iter() {
            match operator_box.update(bounds, event, value) {
                OperatorBoxChange::Update(message) => {
                    return (event::Status::Captured, Some(message));
                }
                OperatorBoxChange::ClearCache(opt_message) => {
                    self.cache.clear();

                    return (event::Status::Ignored, opt_message);
                }
                _ => (),
            }
        }

        (event::Status::Ignored, None)
    }
}

fn get_box_base_point_and_size(bounds: Size, x: usize, y: usize) -> (Point, Size) {
    let x_bla = bounds.width / 7.0;
    let y_bla = bounds.height / 8.0;

    let base_top_left = Point::new(x as f32 * x_bla, y as f32 * y_bla);

    let base_size = Size::new(x_bla, y_bla);

    (base_top_left, base_size)
}

fn scale_point(bounds: Size, point: Point) -> Point {
    let translation = Vector {
        x: (1.0 - SCALE) * bounds.width / 2.0,
        y: (1.0 - SCALE) * bounds.height / 2.0,
    };

    let scaled = Point {
        x: point.x * SCALE,
        y: point.y * SCALE,
    };

    scaled + translation
}

fn scale_size(size: Size) -> Size {
    Size::new(size.width * SCALE, size.height * SCALE)
}
