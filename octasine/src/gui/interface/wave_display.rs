use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::parameters::list::OperatorParameter;
use crate::parameters::operator_feedback::OperatorFeedbackValue;
use crate::parameters::operator_frequency_fine::OperatorFrequencyFineValue;
use crate::parameters::operator_frequency_free::OperatorFrequencyFreeValue;
use crate::parameters::operator_frequency_ratio::OperatorFrequencyRatioValue;
use crate::parameters::operator_mod_out::OperatorModOutValue;
use crate::parameters::operator_mod_target::{
    ModTargetStorage, Operator2ModulationTargetValue, Operator3ModulationTargetValue,
    Operator4ModulationTargetValue,
};
use crate::parameters::operator_panning::OperatorPanningValue;
use crate::parameters::{Parameter, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::{Message, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT * 2;
const HEIGHT: u16 = LINE_HEIGHT * 2;

const HEIGHT_MIDDLE: f32 = HEIGHT as f32 / 2.0 - 0.5;
const SHAPE_HEIGHT_RANGE: f32 = HEIGHT as f32 / 4.0;

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub middle_line_color: Color,
    pub border_color_active: Color,
    pub border_color_hovered: Color,
    pub shape_line_color_active: Color,
    pub shape_line_color_hovered: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
}

enum OperatorModTargets {
    Two(Operator2ModulationTargetValue),
    Three(Operator3ModulationTargetValue),
    Four(Operator4ModulationTargetValue),
}

struct OperatorData {
    frequency_ratio: OperatorFrequencyRatioValue,
    frequency_free: OperatorFrequencyFreeValue,
    frequency_fine: OperatorFrequencyFineValue,
    feedback: OperatorFeedbackValue,
    pan: OperatorPanningValue,
    mod_out: OperatorModOutValue,
    mod_targets: Option<OperatorModTargets>,
}

impl OperatorData {
    fn new(operator_index: usize) -> Self {
        let mod_targets = match operator_index {
            1 => Some(OperatorModTargets::Two(
                Operator2ModulationTargetValue::default(),
            )),
            2 => Some(OperatorModTargets::Three(
                Operator3ModulationTargetValue::default(),
            )),
            3 => Some(OperatorModTargets::Four(
                Operator4ModulationTargetValue::default(),
            )),
            _ => None,
        };

        Self {
            frequency_free: Default::default(),
            frequency_ratio: Default::default(),
            frequency_fine: Default::default(),
            feedback: Default::default(),
            pan: Default::default(),
            mod_out: Default::default(),
            mod_targets,
        }
    }
}

pub struct WaveDisplay {
    operator_index: u8,
    style: Theme,
    canvas_cache: Cache,
    canvas_bounds_path: Path,
    operators: [OperatorData; 4],
}

impl WaveDisplay {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: u8, style: Theme) -> Self {
        let mut operators = ::std::array::from_fn(OperatorData::new);

        for (i, operator) in operators.iter_mut().enumerate() {
            let i = i as u8;

            operator.frequency_ratio.replace_from_patch(
                sync_handle
                    .get_parameter(Parameter::Operator(i, OperatorParameter::FrequencyRatio)),
            );
            operator.frequency_free.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::FrequencyFree)),
            );
            operator.frequency_fine.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::FrequencyFine)),
            );
            operator.feedback.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::Feedback)),
            );
            operator.pan.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::Panning)),
            );
            operator.mod_out.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::ModOut)),
            );

            let targets =
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::ModTargets));

            match operator.mod_targets.as_mut() {
                None => (),
                Some(OperatorModTargets::Two(v)) => v.replace_from_patch(targets),
                Some(OperatorModTargets::Three(v)) => v.replace_from_patch(targets),
                Some(OperatorModTargets::Four(v)) => v.replace_from_patch(targets),
            }
        }

        let canvas_bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );

        Self {
            operator_index,
            style,
            canvas_cache: Cache::new(),
            canvas_bounds_path,
            operators,
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.canvas_cache.clear();
    }

    pub fn set_value(&mut self, parameter: Parameter, value: f32) {
        match parameter {
            Parameter::Operator(i, _) if i < self.operator_index => return,
            Parameter::Operator(i, OperatorParameter::FrequencyRatio) => self.operators[i as usize]
                .frequency_ratio
                .replace_from_patch(value),
            Parameter::Operator(i, OperatorParameter::FrequencyFree) => self.operators[i as usize]
                .frequency_free
                .replace_from_patch(value),
            Parameter::Operator(i, OperatorParameter::FrequencyFine) => self.operators[i as usize]
                .frequency_fine
                .replace_from_patch(value),
            Parameter::Operator(i, OperatorParameter::Feedback) => self.operators[i as usize]
                .feedback
                .replace_from_patch(value),
            Parameter::Operator(i, OperatorParameter::Panning) => {
                self.operators[i as usize].pan.replace_from_patch(value)
            }
            Parameter::Operator(i, OperatorParameter::ModOut) => {
                self.operators[i as usize].mod_out.replace_from_patch(value)
            }
            Parameter::Operator(i, OperatorParameter::ModTargets) => {
                match self.operators[i as usize].mod_targets.as_mut() {
                    None => return,
                    Some(OperatorModTargets::Two(v)) => v.replace_from_patch(value),
                    Some(OperatorModTargets::Three(v)) => v.replace_from_patch(value),
                    Some(OperatorModTargets::Four(v)) => v.replace_from_patch(value),
                }
            }
            _ => return,
        }

        self.canvas_cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }
}

// Drawing
impl WaveDisplay {
    fn draw_background(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        frame.fill(&self.canvas_bounds_path, style.background_color);
    }

    fn draw_border(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let stroke = Stroke::default().with_color(style.border_color_active);

        frame.stroke(&self.canvas_bounds_path, stroke);
    }

    fn draw_middle_line(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let path = Path::line(
            Point::new(0.5, HEIGHT_MIDDLE),
            Point::new(WIDTH as f32 - 0.5, HEIGHT_MIDDLE),
        );
        let stroke = Stroke::default().with_color(style.middle_line_color);

        frame.stroke(&path, stroke)
    }

    fn draw_wave_line(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let mut path = path::Builder::new();

        for i in 0..WIDTH - 1 {
            let phase = (i as f64) / (WIDTH - 1) as f64;
            let y = calculate_curve(self.operator_index, &self.operators, phase) as f32;

            let visual_y = HEIGHT_MIDDLE - y * SHAPE_HEIGHT_RANGE;
            let visual_x = 0.5 + i as f32;

            if i == 0 {
                path.move_to(Point::new(visual_x, visual_y))
            } else {
                path.line_to(Point::new(visual_x, visual_y))
            }
        }

        let path = path.build();

        let color = style.shape_line_color_active;

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&path, stroke)
    }
}

impl Program<Message> for WaveDisplay {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.canvas_cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, self.style.wave_display());
            self.draw_middle_line(frame, self.style.wave_display());
            self.draw_wave_line(frame, self.style.wave_display());
            self.draw_border(frame, self.style.wave_display());
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        _event: event::Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
    }
}

fn calculate_curve(operator_index: u8, operators: &[OperatorData; 4], phase: f64) -> f64 {
    0.0
}
