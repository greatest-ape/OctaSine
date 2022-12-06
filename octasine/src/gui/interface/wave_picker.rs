use iced_baseview::widget::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::{
    alignment::Horizontal, Alignment, Color, widget::Column, Element, Length, Point, Rectangle, Size,
    widget::Space, widget::Text,
};

use crate::common::{CalculateCurve, Phase};
use crate::parameters::{Parameter, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::value_text::ValueText;
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

pub struct WavePicker<P: ParameterValue> {
    title: String,
    style: Theme,
    shape: P::Value,
    canvas: WavePickerCanvas<P>,
    value_text: ValueText<P>,
}

impl<P> WavePicker<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: CalculateCurve,
{
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        style: Theme,
        title: &str,
    ) -> Self {
        let value = P::new_from_patch(sync_handle.get_parameter(parameter));
        let shape = value.get();

        let canvas = WavePickerCanvas::new(parameter, shape, style);
        let value_text = ValueText::new(sync_handle, style, parameter);

        Self {
            title: title.into(),
            style,
            shape,
            canvas,
            value_text,
        }
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.canvas.set_style(style);
        self.value_text.style = style;
    }

    pub fn set_value(&mut self, value: f32) {
        let shape = P::new_from_patch(value).get();

        if self.shape != shape {
            self.shape = shape;

            self.canvas.set_value(value);
            self.value_text.set_value(value);
        }
    }

    pub fn view(&self) -> Element<Message, Theme> {
        let title = Text::new(&self.title)
            .horizontal_alignment(Horizontal::Center)
            .font(self.style.font_bold())
            .height(Length::Units(LINE_HEIGHT));

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(self.canvas.view())
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(self.value_text.view())
            .into()
    }
}

#[derive(Default)]
struct CanvasState {
    cursor_within_bounds: bool,
    click_started: bool,
}

struct WavePickerCanvas<P: ParameterValue> {
    parameter: Parameter,
    cache: Cache,
    bounds_path: Path,
    shape: P::Value,
    style: Theme,
}

impl<P> WavePickerCanvas<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: CalculateCurve,
{
    pub fn new(parameter: Parameter, shape: P::Value, style: Theme) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );

        Self {
            parameter,
            cache: Cache::new(),
            bounds_path,
            style,
            shape,
        }
    }

    pub fn view(&self) -> Element<Message, Theme> {
        Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT))
            .into()
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    pub fn set_value(&mut self, value: f32) {
        self.shape = P::new_from_patch(value).get();
        self.cache.clear();
    }

    fn draw_background(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        frame.fill(&self.bounds_path, style.background_color);
    }

    fn draw_border(
        &self,
        state: &CanvasState,
        frame: &mut Frame,
        style_sheet: Box<dyn StyleSheet>,
    ) {
        let style = style_sheet.active();

        let color = if state.cursor_within_bounds {
            style.border_color_hovered
        } else {
            style.border_color_active
        };

        let stroke = Stroke::default().with_color(color);

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

    fn draw_shape_line(
        &self,
        state: &CanvasState,
        frame: &mut Frame,
        style_sheet: Box<dyn StyleSheet>,
    ) {
        let style = style_sheet.active();

        let mut path = path::Builder::new();

        for i in 0..WIDTH - 1 {
            let phase = Phase((i as f64) / (WIDTH - 1) as f64);
            let y = CalculateCurve::calculate(self.shape, phase) as f32;

            let visual_y = HEIGHT_MIDDLE - y * SHAPE_HEIGHT_RANGE;
            let visual_x = 0.5 + i as f32;

            if i == 0 {
                path.move_to(Point::new(visual_x, visual_y))
            } else {
                path.line_to(Point::new(visual_x, visual_y))
            }
        }

        let path = path.build();

        let color = if state.cursor_within_bounds {
            style.shape_line_color_hovered
        } else {
            style.shape_line_color_active
        };

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&path, stroke)
    }
}

impl<P> Program<Message, Theme> for WavePickerCanvas<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: CalculateCurve,
{
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, self.style.wave_picker());
            self.draw_middle_line(frame, self.style.wave_picker());
            self.draw_shape_line(state, frame, self.style.wave_picker());
            self.draw_border(state, frame, self.style.wave_picker());
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
        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved { position }) => {
                let cursor_within_bounds = bounds.contains(position);

                if state.cursor_within_bounds != cursor_within_bounds {
                    state.cursor_within_bounds = cursor_within_bounds;

                    self.cache.clear();
                }

                (event::Status::Ignored, None)
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(
                iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right,
            )) if state.cursor_within_bounds => {
                state.click_started = true;

                (event::Status::Captured, None)
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(
                button @ (iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right),
            )) if state.click_started => {
                if state.cursor_within_bounds {
                    let shape_index = P::Value::steps()
                        .iter()
                        .position(|s| *s == self.shape)
                        .unwrap();

                    let new_shape_index = match button {
                        iced_baseview::mouse::Button::Left => {
                            (shape_index + 1) % P::Value::steps().len()
                        }
                        iced_baseview::mouse::Button::Right if shape_index == 0 => {
                            P::Value::steps().len() - 1
                        }
                        iced_baseview::mouse::Button::Right => shape_index - 1,
                        _ => unreachable!(),
                    };

                    let new_shape = P::Value::steps()[new_shape_index];
                    let new_value = P::new_from_audio(new_shape).to_patch();

                    state.click_started = false;

                    (
                        event::Status::Captured,
                        Some(Message::ChangeSingleParameterImmediate(
                            self.parameter,
                            new_value,
                        )),
                    )
                } else {
                    state.click_started = false;

                    (event::Status::Ignored, None)
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
