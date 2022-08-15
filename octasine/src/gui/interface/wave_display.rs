use duplicate::duplicate_item;

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
use crate::simd::*;
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
            frequency_free: Default::default(),
            frequency_ratio: Default::default(),
            frequency_fine: Default::default(),
            feedback: Default::default(),
            pan: Default::default(),
            mod_out: (operator_index > 0).then_some(Default::default()),
            mod_targets,
        }
    }
}

pub struct WaveDisplay {
    operator_index: usize,
    style: Theme,
    canvas_cache: Cache,
    canvas_bounds_path: Path,
    operators: [OperatorData; 4],
}

impl WaveDisplay {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
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
            // Any changes pertaining to lower-index operators can be ignored
            Parameter::Operator(i, _) if (i as usize) < self.operator_index => return,
            // Any changes to frequencies, panning, mod out or mod targets of
            // this or lower-index operators can be ignored
            Parameter::Operator(
                i,
                OperatorParameter::FrequencyRatio
                | OperatorParameter::FrequencyFree
                | OperatorParameter::FrequencyFine
                | OperatorParameter::Panning
                | OperatorParameter::ModOut
                | OperatorParameter::ModTargets,
            ) if (i as usize) <= self.operator_index => return,
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

        let mut y_values = [0.0; 4];
        let mut offset = 0;

        loop {
            let num_remaining_samples = (WIDTH as u64 - offset) as u64;

            unsafe {
                let samples_processed = match num_remaining_samples {
                    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                    (4..) if is_x86_feature_detected!("avx") => {
                        Avx::gen_segment(
                            &mut y_values[..4],
                            self.operator_index,
                            &self.operators,
                            offset as usize,
                        );

                        4
                    }
                    2.. => {
                        cfg_if::cfg_if!(
                            if #[cfg(feature = "simd")] {
                                cfg_if::cfg_if!(
                                    if #[cfg(target_arch = "x86_64")] {
                                        // SSE2 is always supported on x86_64
                                        Sse2::gen_segment(
                                            &mut y_values[..2],
                                            self.operator_index,
                                            &self.operators,
                                            offset as usize,
                                        );
                                    } else {
                                        FallbackSleef::gen_segment(
                                            &mut y_values[..2],
                                            self.operator_index,
                                            &self.operators,
                                            offset as usize,
                                        );
                                    }
                                )
                            } else {
                                FallbackStd::gen_segment(
                                    &mut y_values[..2],
                                    self.operator_index,
                                    &self.operators,
                                    offset as usize,
                                );
                            }
                        );

                        2
                    }
                    1 | 0 => {
                        break;
                    }
                };

                for (i, y) in y_values.iter().copied().take(samples_processed).enumerate() {
                    let visual_y = HEIGHT_MIDDLE - y as f32 * SHAPE_HEIGHT_RANGE;
                    let visual_x = 0.5 + (offset + i as u64) as f32;

                    if offset == 0 {
                        path.move_to(Point::new(visual_x, visual_y))
                    } else {
                        path.line_to(Point::new(visual_x, visual_y))
                    }
                }

                offset += samples_processed as u64;
            }
        }

        frame.stroke(
            &path.build(),
            Stroke::default().with_color(style.shape_line_color_active),
        )
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

trait PathGen {
    unsafe fn gen_segment(
        y_values: &mut [f64],
        operator_index: usize,
        operator_data: &[OperatorData; 4],
        offset: usize,
    );
}

#[duplicate_item(
    [
        S [ FallbackStd ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(not(feature = "fake-feature")) ]
    ]
    [
        S [ FallbackSleef ]
        target_feature_enable [ cfg(not(feature = "fake-feature")) ]
        feature_gate [ cfg(all(feature = "simd")) ]
    ]
    [
        S [ Sse2 ]
        target_feature_enable [ target_feature(enable = "sse2") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
    [
        S [ Avx ]
        target_feature_enable [ target_feature(enable = "avx") ]
        feature_gate [ cfg(all(feature = "simd", target_arch = "x86_64")) ]
    ]
)]
mod gen {
    #[feature_gate]
    use std::f64::consts::TAU;

    #[feature_gate]
    use crate::simd::Simd;

    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl PathGen for S {
        #[target_feature_enable]
        unsafe fn gen_segment(
            y_values: &mut [f64],
            operator_index: usize,
            operator_data: &[OperatorData; 4],
            offset: usize,
        ) {
            assert_eq!(y_values.len(), S::PD_WIDTH);

            let mut phases = [0.0; S::PD_WIDTH];

            for (i, phase) in phases.iter_mut().enumerate() {
                *phase = (offset as f64 + i as f64) / (WIDTH - 1) as f64;
            }

            let phases = S::pd_mul(S::pd_loadu(phases.as_ptr()), S::pd_set1(TAU));

            let mut mod_inputs = [
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
            ];

            for i in (operator_index..4).rev() {
                let feedback = S::pd_mul(
                    S::pd_fast_sin(phases),
                    S::pd_set1(operator_data[operator_index].feedback.get() as f64),
                );

                let samples = S::pd_fast_sin(S::pd_add(S::pd_add(feedback, mod_inputs[i]), phases));

                // Store modulation outputs
                match (
                    operator_data[i].mod_out.as_ref(),
                    operator_data[i].mod_targets.as_ref(),
                ) {
                    (Some(mod_out), Some(mod_targets)) => {
                        let mod_out = S::pd_mul(S::pd_set1(mod_out.get() as f64), samples);

                        let mod_targets = match mod_targets {
                            OperatorModTargets::Two(v) => v.get(),
                            OperatorModTargets::Three(v) => v.get(),
                            OperatorModTargets::Four(v) => v.get(),
                        };

                        for target_index in mod_targets.active_indices() {
                            mod_inputs[target_index] = S::pd_add(mod_inputs[target_index], mod_out);
                        }
                    }
                    _ => (),
                }

                if i == operator_index {
                    S::pd_storeu(y_values.as_mut_ptr(), samples);
                }
            }
        }
    }
}
