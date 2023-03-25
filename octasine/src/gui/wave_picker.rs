use iced_baseview::widget::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::{
    alignment::Horizontal, widget::Column, widget::Space, widget::Text, Alignment, Color, Element,
    Length, Point, Rectangle, Size,
};

use crate::common::{Phase, WaveformChoices};
use crate::parameters::{Parameter, ParameterValue, WrappedParameter};
use crate::sync::GuiSyncHandle;

use super::style::Theme;
use super::value_text::ValueText;
use super::{Message, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT * 2;
const HEIGHT: u16 = LINE_HEIGHT * 2;

const HEIGHT_MIDDLE: f32 = HEIGHT as f32 / 2.0 - 0.5;
const SHAPE_HEIGHT_RANGE: f32 = HEIGHT as f32 / 4.0;

#[derive(Debug, Clone)]
pub struct Appearance {
    pub background_color: Color,
    pub middle_line_color: Color,
    pub border_color_active: Color,
    pub border_color_hovered: Color,
    pub shape_line_color_active: Color,
    pub shape_line_color_hovered: Color,
}

pub trait StyleSheet {
    fn appearance(&self) -> Appearance;
}

pub struct WavePicker<P: ParameterValue> {
    title: String,
    shape: P::Value,
    canvas: WavePickerCanvas<P>,
    value_text: ValueText<P>,
}

impl<P> WavePicker<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: WaveformChoices,
{
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter: Parameter, title: &str) -> Self {
        let parameter = parameter.into();

        let value = P::new_from_patch(sync_handle.get_parameter(parameter));
        let shape = value.get();

        let canvas = WavePickerCanvas::new(parameter, shape);
        let value_text = ValueText::new(sync_handle, parameter);

        Self {
            title: title.into(),
            shape,
            canvas,
            value_text,
        }
    }

    pub fn theme_changed(&mut self) {
        self.canvas.theme_changed();
    }

    pub fn set_value(&mut self, value: f32) {
        let shape = P::new_from_patch(value).get();

        if self.shape != shape {
            self.shape = shape;

            self.canvas.set_value(value);
            self.value_text.set_value(value);
        }
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let title = Text::new(&self.title)
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_bold())
            .height(Length::Fixed(LINE_HEIGHT.into()));

        Column::new()
            .width(Length::Fixed(f32::from(LINE_HEIGHT * 4)))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
            .push(self.canvas.view())
            .push(Space::with_height(Length::Fixed(LINE_HEIGHT.into())))
            .push(self.value_text.view(theme))
            .into()
    }
}

#[derive(Default)]
struct CanvasState {
    cursor_within_bounds: bool,
    click_started: bool,
}

struct WavePickerCanvas<P: ParameterValue> {
    parameter: WrappedParameter,
    cache: Cache,
    bounds_path: Path,
    shape: P::Value,
}

impl<P> WavePickerCanvas<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: WaveformChoices,
{
    pub fn new(parameter: WrappedParameter, shape: P::Value) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );

        Self {
            parameter,
            cache: Cache::new(),
            bounds_path,
            shape,
        }
    }

    pub fn view(&self) -> Element<Message, Theme> {
        Canvas::new(self)
            .width(Length::Fixed(WIDTH.into()))
            .height(Length::Fixed(HEIGHT.into()))
            .into()
    }

    pub fn theme_changed(&mut self) {
        self.cache.clear();
    }

    pub fn set_value(&mut self, value: f32) {
        self.shape = P::new_from_patch(value).get();
        self.cache.clear();
    }

    fn draw_background(&self, frame: &mut Frame, theme: &Theme) {
        let apparence = theme.appearance();

        frame.fill(&self.bounds_path, apparence.background_color);
    }

    fn draw_border(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        let appearance = theme.appearance();

        let color = if state.cursor_within_bounds {
            appearance.border_color_hovered
        } else {
            appearance.border_color_active
        };

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&self.bounds_path, stroke);
    }

    fn draw_middle_line(&self, frame: &mut Frame, theme: &Theme) {
        let appearance = theme.appearance();

        let path = Path::line(
            Point::new(0.5, HEIGHT_MIDDLE),
            Point::new(WIDTH as f32 - 0.5, HEIGHT_MIDDLE),
        );
        let stroke = Stroke::default().with_color(appearance.middle_line_color);

        frame.stroke(&path, stroke)
    }

    fn draw_shape_line(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        let appearance = theme.appearance();

        let mut path = path::Builder::new();

        for i in 0..WIDTH - 1 {
            let phase = Phase((i as f64) / (WIDTH - 1) as f64);
            let y = WaveformChoices::calculate_for_current(self.shape, phase) as f32;

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
            appearance.shape_line_color_hovered
        } else {
            appearance.shape_line_color_active
        };

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&path, stroke)
    }
}

impl<P> Program<Message, Theme> for WavePickerCanvas<P>
where
    P: ParameterValue + Copy + 'static,
    P::Value: WaveformChoices,
{
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
            self.draw_middle_line(frame, theme);
            self.draw_shape_line(state, frame, theme);
            self.draw_border(state, frame, theme);
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
                    let shape_index = P::Value::choices()
                        .iter()
                        .position(|s| *s == self.shape)
                        .unwrap();

                    let new_shape_index = match button {
                        iced_baseview::mouse::Button::Left => {
                            (shape_index + 1) % P::Value::choices().len()
                        }
                        iced_baseview::mouse::Button::Right if shape_index == 0 => {
                            P::Value::choices().len() - 1
                        }
                        iced_baseview::mouse::Button::Right => shape_index - 1,
                        _ => unreachable!(),
                    };

                    let new_shape = P::Value::choices()[new_shape_index];
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
