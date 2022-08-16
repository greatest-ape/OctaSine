use duplicate::duplicate_item;

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
    Operator2ModulationTargetValue, Operator3ModulationTargetValue, Operator4ModulationTargetValue,
};
use crate::parameters::operator_panning::OperatorPanningValue;
use crate::parameters::operator_volume::OperatorVolumeValue;
use crate::parameters::operator_wave_type::OperatorWaveTypeValue;
use crate::parameters::{Parameter, ParameterValue};
use crate::simd::*;
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::{Message, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT * 2;
const HEIGHT: u16 = LINE_HEIGHT * 2;

const HEIGHT_MIDDLE: f32 = HEIGHT as f32 / 2.0 - 0.5;
const WAVE_HEIGHT_RANGE: f32 = HEIGHT as f32 / 4.0;

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

    fn create_all_four<H: GuiSyncHandle>(sync_handle: &H) -> [Self; 4] {
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

        operators
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
        let operators = OperatorData::create_all_four(sync_handle);

        let values = ::std::array::from_fn(|i| Point::new(0.5 + i as f32, 0.0));

        let mut display = Self {
            operator_index,
            style,
            canvas_left: WaveDisplayCanvas::new(style, values),
            canvas_right: WaveDisplayCanvas::new(style, values),
            operators,
        };

        display.recalculate_values();

        display
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.canvas_left.set_style(style);
        self.canvas_right.set_style(style);
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

        self.recalculate_values();
    }

    fn recalculate_values(&mut self) {
        let mut offset = 0;

        loop {
            let num_remaining_samples = self.canvas_left.values.len() as u64 - offset as u64;

            unsafe {
                match num_remaining_samples {
                    #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                    (2..) if is_x86_feature_detected!("avx") => {
                        let end_offset = offset + 2;

                        Avx::gen_segment(
                            &mut self.canvas_left.values[offset..end_offset],
                            &mut self.canvas_right.values[offset..end_offset],
                            self.operator_index,
                            &self.operators,
                            offset as usize,
                        );

                        offset = end_offset;
                    }
                    1.. => {
                        let end_offset = offset + 1;

                        cfg_if::cfg_if!(
                            if #[cfg(feature = "simd")] {
                                cfg_if::cfg_if!(
                                    if #[cfg(target_arch = "x86_64")] {
                                        // SSE2 is always supported on x86_64
                                        Sse2::gen_segment(
                                            &mut self.canvas_left.values[offset..end_offset],
                                            &mut self.canvas_right.values[offset..end_offset],
                                            self.operator_index,
                                            &self.operators,
                                            offset as usize,
                                        );
                                    } else {
                                        FallbackSleef::gen_segment(
                                            &mut self.canvas_left.values[offset..end_offset],
                                            &mut self.canvas_right.values[offset..end_offset],
                                            self.operator_index,
                                            &self.operators,
                                            offset as usize,
                                        );
                                    }
                                )
                            } else {
                                FallbackStd::gen_segment(
                                    &mut self.canvas_left.values[offset..end_offset],
                                    &mut self.canvas_right.values[offset..end_offset],
                                    self.operator_index,
                                    &self.operators,
                                    offset as usize,
                                );
                            }
                        );

                        offset = end_offset;
                    }
                    0 => {
                        break;
                    }
                };
            }
        }

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
    values: [Point; WIDTH as usize],
}

impl WaveDisplayCanvas {
    fn new(style: Theme, values: [Point; WIDTH as usize]) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );
        let cache = Cache::new();

        Self {
            bounds_path,
            cache,
            style,
            values,
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

        path.move_to(self.values[0]);

        for point in self.values[1..].iter().copied() {
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

trait PathGen {
    unsafe fn gen_segment(
        lefts: &mut [Point],
        rights: &mut [Point],
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
    use crate::parameters::operator_wave_type::WaveType;

    #[feature_gate]
    use crate::simd::Simd;

    #[feature_gate]
    use super::*;

    #[feature_gate]
    impl PathGen for S {
        #[target_feature_enable]
        unsafe fn gen_segment(
            lefts: &mut [Point],
            rights: &mut [Point],
            operator_index: usize,
            operator_data: &[OperatorData; 4],
            offset: usize,
        ) {
            assert_eq!(lefts.len(), S::SAMPLES);
            assert_eq!(rights.len(), S::SAMPLES);

            let mut phases_arr = [0.0; S::PD_WIDTH];

            for sample_index in 0..S::SAMPLES {
                let phase = ((offset + sample_index) as f64) / (WIDTH - 1) as f64;

                let sample_index_offset = sample_index * 2;

                phases_arr[sample_index_offset] = phase;
                phases_arr[sample_index_offset + 1] = phase;
            }

            let phases = S::pd_mul(S::pd_loadu(phases_arr.as_ptr()), S::pd_set1(TAU));

            let mut mod_inputs = [
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
                S::pd_setzero(),
            ];

            let operator_frequency = operator_data[operator_index].frequency();

            for i in (operator_index..4).rev() {
                let samples = match operator_data[i].wave_type.get() {
                    WaveType::Sine => {
                        let phases = {
                            let relative_frequency =
                                S::pd_set1(operator_data[i].frequency() / operator_frequency);

                            S::pd_mul(phases, relative_frequency)
                        };

                        let feedback = S::pd_mul(
                            S::pd_fast_sin(phases),
                            S::pd_set1(operator_data[i].feedback.get() as f64),
                        );

                        // Modulation input panning. See audio gen code for more info
                        let modulation_in = {
                            let pan = S::pd_set1(operator_data[i].pan.get() as f64);

                            // Get panning as value between -1 and 1
                            let pan = S::pd_mul(S::pd_set1(2.0), S::pd_sub(pan, S::pd_set1(0.5)));

                            let pan_tendency = S::pd_max(
                                S::pd_mul(pan, S::pd_distribute_left_right(-1.0, 1.0)),
                                S::pd_setzero(),
                            );
                            let one_minus_pan_tendency = S::pd_sub(S::pd_set1(1.0), pan_tendency);

                            let modulation_in_channel_sum =
                                S::pd_pairwise_horizontal_sum(mod_inputs[i]);

                            S::pd_add(
                                S::pd_mul(pan_tendency, modulation_in_channel_sum),
                                S::pd_mul(one_minus_pan_tendency, mod_inputs[i]),
                            )
                        };

                        S::pd_fast_sin(S::pd_add(S::pd_add(feedback, modulation_in), phases))
                    }
                    WaveType::WhiteNoise => {
                        let mut samples = [0.0f64; S::PD_WIDTH];

                        for sample_index in 0..S::SAMPLES {
                            let sample_index_offset = sample_index * 2;

                            // Generate random numbers like this to get same
                            // output as in WavePicker
                            let seed = phases_arr[sample_index_offset].to_bits() + 2;
                            let random_value = fastrand::Rng::with_seed(seed).f64();

                            samples[sample_index_offset] = random_value;
                            samples[sample_index_offset + 1] = random_value;
                        }

                        S::pd_mul(
                            S::pd_sub(S::pd_loadu(samples.as_ptr()), S::pd_set1(0.5)),
                            S::pd_set1(2.0),
                        )
                    }
                };

                let constant_power_panning = {
                    let [l, r] = operator_data[i].constant_power_panning;

                    S::pd_distribute_left_right(l as f64, r as f64)
                };

                let samples = S::pd_mul(samples, S::pd_set1(operator_data[i].active.get() as f64));
                let samples = S::pd_mul(samples, S::pd_set1(operator_data[i].volume.get() as f64));
                let samples = S::pd_mul(samples, constant_power_panning);

                // Store modulation outputs
                match (
                    operator_data[i].mod_out.map(|v| v.get() as f64),
                    operator_data[i].mod_targets.as_ref(),
                ) {
                    (Some(mod_out), Some(mod_targets)) if mod_out > 0.0 => {
                        let mod_targets = match mod_targets {
                            OperatorModTargets::Two(v) => v.get(),
                            OperatorModTargets::Three(v) => v.get(),
                            OperatorModTargets::Four(v) => v.get(),
                        };

                        let mod_out = S::pd_mul(S::pd_set1(mod_out), samples);

                        for target_index in mod_targets.active_indices() {
                            mod_inputs[target_index] = S::pd_add(mod_inputs[target_index], mod_out);
                        }
                    }
                    _ => (),
                }

                // If this is current operator, set output point y values
                if i == operator_index {
                    let out = S::pd_sub(
                        S::pd_set1(HEIGHT_MIDDLE as f64),
                        S::pd_mul(samples, S::pd_set1(WAVE_HEIGHT_RANGE as f64)),
                    );

                    let mut out_arr = [0.0f64; S::PD_WIDTH];

                    S::pd_storeu(out_arr.as_mut_ptr(), out);

                    for sample_index in 0..S::SAMPLES {
                        let sample_index_offset = sample_index * 2;

                        lefts[sample_index].y = out_arr[sample_index_offset] as f32;
                        rights[sample_index].y = out_arr[sample_index_offset + 1] as f32;
                    }
                }
            }
        }
    }
}
