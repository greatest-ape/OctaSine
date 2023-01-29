mod common;
mod mix_line;
mod mod_box;
mod mod_lines;
mod operator_box;
mod output_box;

use iced_baseview::widget::canvas::{
    event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::parameters::{
    ModTargetStorage, Operator2ModulationTargetValue, Operator3ModulationTargetValue,
    Operator4ModulationTargetValue, OperatorParameter, Parameter, ParameterValue,
};
use crate::sync::GuiSyncHandle;

use self::mix_line::MixOutLine;
use self::mod_box::{
    ModulationBox, ModulationBoxCanvasState, ModulationBoxCanvasUpdate,
    ModulationBoxCanvasUpdateResult,
};
use self::mod_lines::ModOutLines;
use self::operator_box::{OperatorBox, OperatorBoxCanvasState, OperatorBoxChange};
use self::output_box::OutputBox;

use super::style::Theme;
use super::{Message, SnapPoint, LINE_HEIGHT};

/// Canvas width
const WIDTH: u16 = LINE_HEIGHT * 5 + 2;
pub const HEIGHT: u16 = LINE_HEIGHT * 6;
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

#[derive(Debug, Clone)]
pub struct Appearance {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub box_border_color: Color,
    pub operator_box_border_color: Option<Color>,
    pub operator_box_color_active: Color,
    pub operator_box_color_hover: Color,
    pub operator_box_color_dragging: Color,
    pub modulation_box_color_active: Color,
    pub modulation_box_color_inactive: Color,
    pub modulation_box_color_hover: Color,
    pub line_max_color: Color,
    pub mod_out_line_color: Color,
    pub mix_out_line_color: Color,
}

pub trait StyleSheet {
    fn appearance(&self) -> Appearance;
}

struct ModulationMatrixParameters {
    operator_2_targets: ModTargetStorage,
    operator_3_targets: ModTargetStorage,
    operator_4_targets: ModTargetStorage,
    operator_1_mix: f32,
    operator_2_mix: f32,
    operator_3_mix: f32,
    operator_4_mix: f32,
    operator_2_mod: f32,
    operator_3_mod: f32,
    operator_4_mod: f32,
}

impl ModulationMatrixParameters {
    fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let operator_2_targets = Operator2ModulationTargetValue::new_from_patch(
            sync_handle.get_parameter(Parameter::Operator(1, OperatorParameter::ModTargets).into()),
        )
        .get();
        let operator_3_targets = Operator3ModulationTargetValue::new_from_patch(
            sync_handle.get_parameter(Parameter::Operator(2, OperatorParameter::ModTargets).into()),
        )
        .get();
        let operator_4_targets = Operator4ModulationTargetValue::new_from_patch(
            sync_handle.get_parameter(Parameter::Operator(3, OperatorParameter::ModTargets).into()),
        )
        .get();

        let operator_1_mix =
            sync_handle.get_parameter(Parameter::Operator(0, OperatorParameter::MixOut).into());
        let operator_2_mix =
            sync_handle.get_parameter(Parameter::Operator(1, OperatorParameter::MixOut).into());
        let operator_3_mix =
            sync_handle.get_parameter(Parameter::Operator(2, OperatorParameter::MixOut).into());
        let operator_4_mix =
            sync_handle.get_parameter(Parameter::Operator(3, OperatorParameter::MixOut).into());

        let operator_2_mod =
            sync_handle.get_parameter(Parameter::Operator(1, OperatorParameter::ModOut).into());
        let operator_3_mod =
            sync_handle.get_parameter(Parameter::Operator(2, OperatorParameter::ModOut).into());
        let operator_4_mod =
            sync_handle.get_parameter(Parameter::Operator(3, OperatorParameter::ModOut).into());

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
    operator_4_mod_out_lines: ModOutLines,
    operator_3_mod_out_lines: ModOutLines,
    operator_2_mod_out_lines: ModOutLines,
}

impl ModulationMatrixComponents {
    fn new(parameters: &ModulationMatrixParameters, bounds: Size) -> Self {
        let operator_1_box = OperatorBox::new(bounds, 0);
        let operator_2_box = OperatorBox::new(bounds, 1);
        let operator_3_box = OperatorBox::new(bounds, 2);
        let operator_4_box = OperatorBox::new(bounds, 3);

        let operator_4_mod_3_box = ModulationBox::new(
            bounds,
            3,
            2,
            Parameter::Operator(3, OperatorParameter::ModTargets).into(),
            2,
            parameters.operator_4_targets,
        );
        let operator_4_mod_2_box = ModulationBox::new(
            bounds,
            3,
            1,
            Parameter::Operator(3, OperatorParameter::ModTargets).into(),
            1,
            parameters.operator_4_targets,
        );
        let operator_4_mod_1_box = ModulationBox::new(
            bounds,
            3,
            0,
            Parameter::Operator(3, OperatorParameter::ModTargets).into(),
            0,
            parameters.operator_4_targets,
        );
        let operator_3_mod_2_box = ModulationBox::new(
            bounds,
            2,
            1,
            Parameter::Operator(2, OperatorParameter::ModTargets).into(),
            1,
            parameters.operator_3_targets,
        );
        let operator_3_mod_1_box = ModulationBox::new(
            bounds,
            2,
            0,
            Parameter::Operator(2, OperatorParameter::ModTargets).into(),
            0,
            parameters.operator_3_targets,
        );
        let operator_2_mod_1_box = ModulationBox::new(
            bounds,
            1,
            0,
            Parameter::Operator(1, OperatorParameter::ModTargets).into(),
            0,
            parameters.operator_2_targets,
        );

        let output_box = OutputBox::new(bounds);

        let operator_4_mix_out_line = MixOutLine::new(
            operator_4_box.get_center(),
            output_box.y,
            parameters.operator_4_mix,
        );
        let operator_3_mix_out_line = MixOutLine::new(
            operator_3_box.get_center(),
            output_box.y,
            parameters.operator_3_mix,
        );
        let operator_2_mix_out_line = MixOutLine::new(
            operator_2_box.get_center(),
            output_box.y,
            parameters.operator_2_mix,
        );
        let operator_1_mix_out_line = MixOutLine::new(
            operator_1_box.get_center(),
            output_box.y,
            parameters.operator_1_mix,
        );

        let operator_4_mod_out_lines = ModOutLines::new(operator_4_box.get_center());
        let operator_3_mod_out_lines = ModOutLines::new(operator_3_box.get_center());
        let operator_2_mod_out_lines = ModOutLines::new(operator_2_box.get_center());

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
            operator_4_mod_out_lines,
            operator_3_mod_out_lines,
            operator_2_mod_out_lines,
        };

        components.update(parameters);

        components
    }

    fn update(&mut self, parameters: &ModulationMatrixParameters) {
        self.operator_4_mod_3_box.v = parameters.operator_4_targets;
        self.operator_4_mod_2_box.v = parameters.operator_4_targets;
        self.operator_4_mod_1_box.v = parameters.operator_4_targets;
        self.operator_3_mod_2_box.v = parameters.operator_3_targets;
        self.operator_3_mod_1_box.v = parameters.operator_3_targets;

        self.operator_4_mix_out_line
            .update(parameters.operator_4_mix);

        self.operator_3_mix_out_line
            .update(parameters.operator_3_mix);
        self.operator_2_mix_out_line
            .update(parameters.operator_2_mix);
        self.operator_1_mix_out_line
            .update(parameters.operator_1_mix);

        {
            let lines = parameters
                .operator_4_targets
                .active_indices()
                .map(|mod_target| match mod_target {
                    0 => [
                        self.operator_4_mod_1_box.get_center().snap(),
                        self.operator_1_box.get_center().snap(),
                    ],
                    1 => [
                        self.operator_4_mod_2_box.get_center().snap(),
                        self.operator_2_box.get_center().snap(),
                    ],
                    2 => [
                        self.operator_4_mod_3_box.get_center().snap(),
                        self.operator_3_box.get_center().snap(),
                    ],
                    _ => unreachable!(),
                });

            self.operator_4_mod_out_lines.update(lines);
        }

        {
            let lines = parameters
                .operator_3_targets
                .active_indices()
                .map(|mod_target| match mod_target {
                    0 => [
                        self.operator_3_mod_1_box.get_center().snap(),
                        self.operator_1_box.get_center().snap(),
                    ],
                    1 => [
                        self.operator_3_mod_2_box.get_center().snap(),
                        self.operator_2_box.get_center().snap(),
                    ],
                    _ => unreachable!(),
                });

            self.operator_3_mod_out_lines.update(lines);
        };

        {
            let lines = parameters
                .operator_2_targets
                .active_indices()
                .map(|mod_target| match mod_target {
                    0 => [
                        self.operator_2_mod_1_box.get_center().snap(),
                        self.operator_1_box.get_center().snap(),
                    ],
                    _ => unreachable!(),
                });

            self.operator_2_mod_out_lines.update(lines);
        }
    }

    fn draw_lines(&self, frame: &mut Frame, theme: &Theme) {
        self.operator_4_mix_out_line.draw(frame, theme);
        self.operator_3_mix_out_line.draw(frame, theme);
        self.operator_2_mix_out_line.draw(frame, theme);
        self.operator_1_mix_out_line.draw(frame, theme);

        self.operator_4_mod_out_lines.draw(frame, theme);
        self.operator_3_mod_out_lines.draw(frame, theme);
        self.operator_2_mod_out_lines.draw(frame, theme);
    }

    fn draw_boxes(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        self.operator_1_box
            .draw(&state.operator_1_box, frame, theme);
        self.operator_2_box
            .draw(&state.operator_2_box, frame, theme);
        self.operator_3_box
            .draw(&state.operator_3_box, frame, theme);
        self.operator_4_box
            .draw(&state.operator_4_box, frame, theme);

        self.operator_4_mod_3_box
            .draw(&state.operator_4_mod_3_box, frame, theme);
        self.operator_4_mod_2_box
            .draw(&state.operator_4_mod_2_box, frame, theme);
        self.operator_4_mod_1_box
            .draw(&state.operator_4_mod_1_box, frame, theme);
        self.operator_3_mod_2_box
            .draw(&state.operator_3_mod_2_box, frame, theme);
        self.operator_3_mod_1_box
            .draw(&state.operator_3_mod_1_box, frame, theme);
        self.operator_2_mod_1_box
            .draw(&state.operator_2_mod_1_box, frame, theme);

        self.output_box.draw(frame, theme);
    }
}

pub struct ModulationMatrix {
    cache: Cache,
    parameters: ModulationMatrixParameters,
    components: ModulationMatrixComponents,
}

impl ModulationMatrix {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H) -> Self {
        let parameters = ModulationMatrixParameters::new(sync_handle);
        let components = ModulationMatrixComponents::new(&parameters, SIZE);

        Self {
            cache: Cache::default(),
            parameters,
            components,
        }
    }

    pub fn theme_changed(&mut self) {
        self.cache.clear();
    }

    pub fn set_operator_2_target(&mut self, value: f32) {
        self.parameters.operator_2_targets =
            Operator2ModulationTargetValue::new_from_patch(value).get();

        self.update_components();
    }

    pub fn set_operator_3_target(&mut self, value: f32) {
        self.parameters.operator_3_targets =
            Operator3ModulationTargetValue::new_from_patch(value).get();

        self.update_components();
    }

    pub fn set_operator_4_target(&mut self, value: f32) {
        self.parameters.operator_4_targets =
            Operator4ModulationTargetValue::new_from_patch(value).get();

        self.update_components();
    }

    pub fn set_operator_4_mod(&mut self, value: f32) {
        self.parameters.operator_4_mod = value;

        self.update_components();
    }

    pub fn set_operator_3_mod(&mut self, value: f32) {
        self.parameters.operator_3_mod = value;

        self.update_components();
    }

    pub fn set_operator_2_mod(&mut self, value: f32) {
        self.parameters.operator_2_mod = value;

        self.update_components();
    }

    pub fn set_operator_4_mix(&mut self, value: f32) {
        self.parameters.operator_4_mix = value;

        self.update_components();
    }

    pub fn set_operator_3_mix(&mut self, value: f32) {
        self.parameters.operator_3_mix = value;

        self.update_components();
    }

    pub fn set_operator_2_mix(&mut self, value: f32) {
        self.parameters.operator_2_mix = value;

        self.update_components();
    }

    pub fn set_operator_1_mix(&mut self, value: f32) {
        self.parameters.operator_1_mix = value;

        self.update_components();
    }

    fn update_components(&mut self) {
        self.components.update(&self.parameters);

        self.cache.clear();
    }

    pub fn view(&self) -> Element<Message, Theme> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_background(&self, frame: &mut Frame, theme: &Theme) {
        let mut size = frame.size();
        let appearance = theme.appearance();

        size.width -= 1.0;
        size.height -= 1.0;

        let background = Path::rectangle(Point::new(0.5, 0.5), size);

        let stroke = Stroke::default()
            .with_color(appearance.border_color)
            .with_width(1.0);

        frame.fill(&background, appearance.background_color);
        frame.stroke(&background, stroke);
    }
}

#[derive(Default)]
pub struct CanvasState {
    operator_1_box: OperatorBoxCanvasState,
    operator_2_box: OperatorBoxCanvasState,
    operator_3_box: OperatorBoxCanvasState,
    operator_4_box: OperatorBoxCanvasState,
    operator_4_mod_3_box: ModulationBoxCanvasState,
    operator_4_mod_2_box: ModulationBoxCanvasState,
    operator_4_mod_1_box: ModulationBoxCanvasState,
    operator_3_mod_2_box: ModulationBoxCanvasState,
    operator_3_mod_1_box: ModulationBoxCanvasState,
    operator_2_mod_1_box: ModulationBoxCanvasState,
}

impl Program<Message, Theme> for ModulationMatrix {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, theme);

            self.components.draw_lines(frame, theme);
            self.components.draw_boxes(state, frame, theme);
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let operator_boxes = [
            (
                &self.components.operator_1_box,
                &mut state.operator_1_box,
                self.parameters.operator_1_mix,
            ),
            (
                &self.components.operator_2_box,
                &mut state.operator_2_box,
                self.parameters.operator_2_mix,
            ),
            (
                &self.components.operator_3_box,
                &mut state.operator_3_box,
                self.parameters.operator_3_mix,
            ),
            (
                &self.components.operator_4_box,
                &mut state.operator_4_box,
                self.parameters.operator_4_mix,
            ),
        ];

        for (operator_box, state, value) in operator_boxes.into_iter() {
            match operator_box.update(state, bounds, event, value) {
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

        macro_rules! update_mod_box {
            ($mod_box:expr, $state:expr) => {
                match $mod_box.update($state, bounds, event) {
                    ModulationBoxCanvasUpdateResult::Update(message) => {
                        return (event::Status::Captured, Some(message));
                    }
                    ModulationBoxCanvasUpdateResult::ClearCache(opt_message) => {
                        self.cache.clear();

                        return (event::Status::Ignored, opt_message);
                    }
                    ModulationBoxCanvasUpdateResult::None => (),
                }
            };
        }

        update_mod_box!(
            self.components.operator_4_mod_3_box,
            &mut state.operator_4_mod_3_box
        );
        update_mod_box!(
            self.components.operator_4_mod_2_box,
            &mut state.operator_4_mod_2_box
        );
        update_mod_box!(
            self.components.operator_4_mod_1_box,
            &mut state.operator_4_mod_1_box
        );
        update_mod_box!(
            self.components.operator_3_mod_2_box,
            &mut state.operator_3_mod_2_box
        );
        update_mod_box!(
            self.components.operator_3_mod_1_box,
            &mut state.operator_3_mod_1_box
        );
        update_mod_box!(
            self.components.operator_2_mod_1_box,
            &mut state.operator_2_mod_1_box
        );

        (event::Status::Ignored, None)
    }
}
