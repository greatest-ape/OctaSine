mod gen;

use std::borrow::Borrow;

use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::tooltip::Position;
use iced_baseview::{Color, Element, Length, Point, Rectangle, Row, Size, Space, Tooltip};

use crate::parameters::list::OperatorParameter;
use crate::parameters::operator_active::OperatorActiveValue;
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
use crate::parameters::operator_volume::OperatorVolumeValue;
use crate::parameters::operator_wave_type::OperatorWaveTypeValue;
use crate::parameters::{Parameter, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::{Message, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT * 2;
const HEIGHT: u16 = LINE_HEIGHT * 2;

const HEIGHT_MIDDLE: f32 = HEIGHT as f32 / 2.0 - 0.5;
const WAVE_HEIGHT_RANGE: f32 = HEIGHT as f32 / 4.0;

const NUM_POINTS: usize = WIDTH as usize;

type PointArray = [Point; NUM_POINTS];

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub middle_line_color: Color,
    pub border_color: Color,
    pub wave_line_color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
}

enum OperatorModTargets {
    Two(Operator2ModulationTargetValue),
    Three(Operator3ModulationTargetValue),
    Four(Operator4ModulationTargetValue),
}

impl OperatorModTargets {
    fn active_indices<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        ModTargetStorage::active_indices(match self {
            Self::Two(v) => v.borrow(),
            Self::Three(v) => v.borrow(),
            Self::Four(v) => v.borrow(),
        })
    }
}

struct OperatorData {
    wave_type: OperatorWaveTypeValue,
    active: OperatorActiveValue,
    volume: OperatorVolumeValue,
    frequency_ratio: OperatorFrequencyRatioValue,
    frequency_free: OperatorFrequencyFreeValue,
    frequency_fine: OperatorFrequencyFineValue,
    feedback: OperatorFeedbackValue,
    pan: OperatorPanningValue,
    constant_power_panning: [f32; 2],
    mod_out: Option<OperatorModOutValue>,
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
            wave_type: Default::default(),
            active: Default::default(),
            volume: Default::default(),
            frequency_free: Default::default(),
            frequency_ratio: Default::default(),
            frequency_fine: Default::default(),
            feedback: Default::default(),
            pan: Default::default(),
            constant_power_panning: OperatorPanningValue::default().calculate_left_and_right(),
            mod_out: (operator_index > 0).then_some(Default::default()),
            mod_targets,
        }
    }

    fn frequency(&self) -> f64 {
        self.frequency_ratio.get().value * self.frequency_free.get() * self.frequency_fine.get()
    }
}

pub struct WaveDisplay {
    operator_index: usize,
    style: Theme,
    canvas_left: WaveDisplayCanvas,
    canvas_right: WaveDisplayCanvas,
    operators: [OperatorData; 4],
}

impl WaveDisplay {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let mut operators = ::std::array::from_fn(OperatorData::new);

        for (i, operator) in operators.iter_mut().enumerate() {
            let i = i as u8;

            operator.wave_type.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::WaveType)),
            );
            operator.active.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::Active)),
            );
            operator.volume.replace_from_patch(
                sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::Volume)),
            );
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
            operator.constant_power_panning = operator.pan.calculate_left_and_right();
            if let Some(v) = operator.mod_out.as_mut() {
                v.replace_from_patch(
                    sync_handle.get_parameter(Parameter::Operator(i, OperatorParameter::ModOut)),
                )
            }

            match operator.mod_targets.as_mut() {
                None => (),
                Some(OperatorModTargets::Two(v)) => v.replace_from_patch(
                    sync_handle
                        .get_parameter(Parameter::Operator(i, OperatorParameter::ModTargets)),
                ),
                Some(OperatorModTargets::Three(v)) => v.replace_from_patch(
                    sync_handle
                        .get_parameter(Parameter::Operator(i, OperatorParameter::ModTargets)),
                ),
                Some(OperatorModTargets::Four(v)) => v.replace_from_patch(
                    sync_handle
                        .get_parameter(Parameter::Operator(i, OperatorParameter::ModTargets)),
                ),
            }
        }

        let canvas_points = ::std::array::from_fn(|i| Point::new(0.5 + i as f32, 0.0));

        let mut display = Self {
            operator_index,
            style,
            canvas_left: WaveDisplayCanvas::new(style, canvas_points),
            canvas_right: WaveDisplayCanvas::new(style, canvas_points),
            operators,
        };

        display.recalculate_canvas_points();

        display
    }

    pub fn set_value(&mut self, parameter: Parameter, value: f32) {
        match parameter {
            // Any changes pertaining to lower-index operators can be ignored
            Parameter::Operator(i, _) if (i as usize) < self.operator_index => return,
            // Any changes to frequencies, mod out or mod targets of this or
            // lower-index operators can be ignored
            Parameter::Operator(
                i,
                OperatorParameter::FrequencyRatio
                | OperatorParameter::FrequencyFree
                | OperatorParameter::FrequencyFine
                | OperatorParameter::ModOut
                | OperatorParameter::ModTargets,
            ) if (i as usize) <= self.operator_index => return,
            Parameter::Operator(i, OperatorParameter::WaveType) => self.operators[i as usize]
                .wave_type
                .replace_from_patch(value),
            Parameter::Operator(i, OperatorParameter::Active) => {
                self.operators[i as usize].active.replace_from_patch(value)
            }
            Parameter::Operator(i, OperatorParameter::Volume) => {
                self.operators[i as usize].volume.replace_from_patch(value)
            }
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
                self.operators[i as usize].pan.replace_from_patch(value);
                self.operators[i as usize].constant_power_panning =
                    self.operators[i as usize].pan.calculate_left_and_right();
            }
            Parameter::Operator(i, OperatorParameter::ModOut) => {
                if let Some(v) = self.operators[i as usize].mod_out.as_mut() {
                    v.replace_from_patch(value)
                }
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

        self.recalculate_canvas_points();
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.canvas_left.set_style(style);
        self.canvas_right.set_style(style);
    }

    fn recalculate_canvas_points(&mut self) {
        gen::recalculate_canvas_points(
            &mut self.canvas_left.points,
            &mut self.canvas_right.points,
            self.operator_index,
            &self.operators,
        );

        self.canvas_left.cache.clear();
        self.canvas_right.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        let canvas_left = Tooltip::new(self.canvas_left.view(), "Left channel", Position::Bottom)
            .style(self.style.tooltip())
            .font(self.style.font_regular())
            .padding(self.style.tooltip_padding());

        let canvas_right =
            Tooltip::new(self.canvas_right.view(), "Right channel", Position::Bottom)
                .style(self.style.tooltip())
                .font(self.style.font_regular())
                .padding(self.style.tooltip_padding());

        Row::new()
            .push(canvas_left)
            .push(Space::with_width(Length::Units(4)))
            .push(canvas_right)
            .into()
    }
}

struct WaveDisplayCanvas {
    bounds_path: Path,
    cache: Cache,
    style: Theme,
    points: PointArray,
}

impl WaveDisplayCanvas {
    fn new(style: Theme, points: PointArray) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );
        let cache = Cache::new();

        Self {
            bounds_path,
            cache,
            style,
            points,
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    fn draw_background(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        frame.fill(&self.bounds_path, style.background_color);
    }

    fn draw_border(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let stroke = Stroke::default().with_color(style.border_color);

        frame.stroke(&self.bounds_path, stroke);
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

        path.move_to(self.points[0]);

        for point in self.points[1..].iter().copied() {
            path.line_to(point);
        }

        frame.stroke(
            &path.build(),
            Stroke::default().with_color(style.wave_line_color),
        )
    }
}

impl Program<Message> for WaveDisplayCanvas {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
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
