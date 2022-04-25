mod common;
mod mix_line;
mod mod_box;
mod mod_line;
mod operator_box;
mod output_box;

use iced_baseview::canvas::{event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::common::{ModTarget, ModTargetStorage};
use crate::parameters::values::{
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
    ParameterValue,
};
use crate::GuiSyncHandle;

use self::mix_line::MixOutLine;
use self::mod_box::{ModulationBox, ModulationBoxChange, ModulationBoxUpdate};
use self::mod_line::ModOutLine;
use self::operator_box::{OperatorBox, OperatorBoxChange};
use self::output_box::OutputBox;

use super::style::Theme;
use super::{Message, LINE_HEIGHT};

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
    pub modulation_box_color_hover: Color,
    pub line_max_color: Color,
    pub mod_out_line_color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
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
    operator_4_mod_3_box: ModulationBox<Operator4ModulationTargetValue>,
    operator_4_mod_2_box: ModulationBox<Operator4ModulationTargetValue>,
    operator_4_mod_1_box: ModulationBox<Operator4ModulationTargetValue>,
    operator_3_mod_2_box: ModulationBox<Operator3ModulationTargetValue>,
    operator_3_mod_1_box: ModulationBox<Operator3ModulationTargetValue>,
    operator_2_mod_1_box: ModulationBox<Operator2ModulationTargetValue>,
    output_box: OutputBox,
    operator_4_mix_out_line: MixOutLine,
    operator_3_mix_out_line: MixOutLine,
    operator_2_mix_out_line: MixOutLine,
    operator_1_mix_out_line: MixOutLine,
    operator_4_mod_out_line: ModOutLine,
    operator_3_mod_out_line: ModOutLine,
    operator_2_mod_out_line: ModOutLine,
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

        let operator_4_mix_out_line = MixOutLine::new(
            operator_4_box.center,
            output_box.y,
            parameters.operator_4_mix,
            style.into(),
        );
        let operator_3_mix_out_line = MixOutLine::new(
            operator_3_box.center,
            output_box.y,
            parameters.operator_3_mix,
            style.into(),
        );
        let operator_2_mix_out_line = MixOutLine::new(
            operator_2_box.center,
            output_box.y,
            parameters.operator_2_mix,
            style.into(),
        );
        let operator_1_mix_out_line = MixOutLine::new(
            operator_1_box.center,
            output_box.y,
            parameters.operator_1_mix,
            style.into(),
        );

        let operator_4_mod_out_line = ModOutLine::new(operator_4_box.center, style.into());
        let operator_3_mod_out_line = ModOutLine::new(operator_3_box.center, style.into());
        let operator_2_mod_out_line = ModOutLine::new(operator_2_box.center, style.into());

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
            operator_4_mix_out_line,
            operator_3_mix_out_line,
            operator_2_mix_out_line,
            operator_1_mix_out_line,
            operator_4_mod_out_line,
            operator_3_mod_out_line,
            operator_2_mod_out_line,
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

        self.operator_4_mix_out_line
            .update(parameters.operator_4_mix, style.into());

        self.operator_3_mix_out_line
            .update(parameters.operator_3_mix, style.into());
        self.operator_2_mix_out_line
            .update(parameters.operator_2_mix, style.into());
        self.operator_1_mix_out_line
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

            self.operator_4_mod_out_line.update(points, style.into());
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

            self.operator_3_mod_out_line.update(points, style.into());
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

            self.operator_2_mod_out_line.update(points, style.into());
        }
    }

    fn draw_lines(&self, frame: &mut Frame) {
        self.operator_4_mix_out_line.draw(frame);
        self.operator_3_mix_out_line.draw(frame);
        self.operator_2_mix_out_line.draw(frame);
        self.operator_1_mix_out_line.draw(frame);

        self.operator_4_mod_out_line.draw(frame);
        self.operator_3_mod_out_line.draw(frame);
        self.operator_2_mod_out_line.draw(frame);
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
