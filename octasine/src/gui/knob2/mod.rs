use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use iced_baseview::{
    alignment::Horizontal,
    event::Status,
    keyboard,
    mouse::{self, Button},
    widget::{
        canvas::{self, path::Arc, Cache, Frame, Path, Program, Stroke},
        Canvas, Column, Container, Text,
    },
    Color, Element, Length, Point, Rectangle,
};

use crate::parameters::{list::Parameter, WrappedParameter};

use super::{style::Theme, Message, LINE_HEIGHT};

const KNOB_SIZE: u16 = LINE_HEIGHT * 2;
const CANVAS_SIZE: u16 = KNOB_SIZE * 2;

const ARC_EXTRA_ANGLE: f32 = PI * 0.5 / 3.0 * 2.0;
const ARC_START_ANGLE: f32 = PI - ARC_EXTRA_ANGLE;
const ARC_END_ANGLE_ADDITION: f32 = PI + 2.0 * ARC_EXTRA_ANGLE;

const MARKER_DOT_DISTANCE: u16 = LINE_HEIGHT / 2;

fn arc_angle(value: f32) -> f32 {
    ARC_START_ANGLE + value * ARC_END_ANGLE_ADDITION
}

pub struct KnobWithText {
    canvas: Knob,
}

impl KnobWithText {
    pub fn new() -> Self {
        let p = Parameter::Master(crate::parameters::list::MasterParameter::Volume).into();

        Self {
            canvas: Knob::new(p, KnobVariant::Regular, Some(0.5), 0.5, 0.5),
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.canvas.set_value(value);
    }

    pub fn theme_changed(&mut self) {
        self.canvas.theme_changed();
    }

    pub fn view(&self, theme: &Theme) -> Element<Message, Theme> {
        let title = Text::new("VOL")
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_bold())
            .height(Length::Fixed(LINE_HEIGHT.into()))
            .width(Length::from(CANVAS_SIZE));
        let value = Text::new("0.0")
            .horizontal_alignment(Horizontal::Center)
            .font(theme.font_regular())
            .height(Length::Fixed(LINE_HEIGHT.into()))
            .width(Length::from(CANVAS_SIZE));

        Column::new()
            .width(Length::Fixed(f32::from(CANVAS_SIZE)))
            .push(title)
            .push(
                Container::new(self.canvas.view())
                    .width(CANVAS_SIZE)
                    .height(CANVAS_SIZE),
            )
            .push(value)
            .into()
    }
}

pub struct Knob {
    parameter: WrappedParameter,
    variant: KnobVariant,
    /// Where to place anchor dot
    anchor_dot_value: Option<f32>,
    /// Value to reset to when double-clicking
    reset_value: f32,
    value: f32,

    cache: Cache,
    center: Point,
    radius: f32,
}

impl Knob {
    pub fn new(
        parameter: WrappedParameter,
        variant: KnobVariant,
        anchor_dot_value: Option<f32>,
        reset_value: f32,
        value: f32,
    ) -> Self {
        let center_x = KNOB_SIZE;
        let center_y = LINE_HEIGHT + KNOB_SIZE / 2;
        let center = Point::new((center_x).into(), (center_y).into());

        Self {
            parameter,
            variant,
            anchor_dot_value,
            reset_value,
            value,

            cache: Cache::default(),
            center,
            radius: (KNOB_SIZE / 2) as f32 - 1.0,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;

        self.cache.clear();
    }

    pub fn theme_changed(&mut self) {
        self.cache.clear();
    }

    pub fn view(&self) -> Element<Message, Theme> {
        Canvas::new(self)
            .width(Length::from(KNOB_SIZE * 2))
            .height(Length::from(KNOB_SIZE * 2))
            .into()
    }

    fn draw_arc(&self, frame: &mut Frame, color: Color, value: f32) {
        let arc = Arc {
            center: self.center,
            radius: self.radius,
            start_angle: ARC_START_ANGLE,
            end_angle: arc_angle(value),
        };

        let path = Path::new(|builder| {
            builder.arc(arc);
        });

        let stroke = Stroke::default()
            .with_color(color)
            .with_width(2.0)
            .with_line_cap(iced_audio::knob::LineCap::Square);

        frame.stroke(&path, stroke);
    }

    fn draw_notch(&self, frame: &mut Frame, color: Color, value: f32) {
        let path = Path::new(|builder| {
            let angle = arc_angle(value);

            let x_addition = angle.cos() * (self.radius - 2.0);
            let y_addition = angle.sin() * (self.radius - 2.0);

            let mut point = self.center;

            point.x += x_addition / 3.0;
            point.y += y_addition / 3.0;

            builder.move_to(point);

            point.x += x_addition / 3.0 * 2.0;
            point.y += y_addition / 3.0 * 2.0;

            builder.line_to(point)
        });

        let stroke = Stroke::default()
            .with_color(color)
            .with_width(2.0)
            .with_line_cap(iced_audio::knob::LineCap::Round);

        frame.stroke(&path, stroke);
    }

    fn draw_marker_dot(&self, frame: &mut Frame, value: f32, color: Color) {
        let path = Path::new(|builder| {
            let angle = arc_angle(value);
            let distance = self.radius + MARKER_DOT_DISTANCE as f32;

            let mut point = self.center;

            point.x += angle.cos() * distance;
            point.y += angle.sin() * distance;

            builder.circle(point, 1.0)
        });

        let stroke = Stroke::default()
            .with_color(color)
            .with_width(2.0)
            .with_line_cap(iced_audio::knob::LineCap::Round);

        frame.stroke(&path, stroke);
    }

    fn is_cursor_over_knob(&self, bounds: Rectangle, cursor: Point) -> bool {
        if bounds.contains(cursor) {
            let relative_cursor_position = Point {
                x: cursor.x - bounds.x,
                y: cursor.y - bounds.y,
            };

            relative_cursor_position.distance(self.center) <= self.radius
        } else {
            false
        }
    }
}

pub struct KnobState {
    last_cursor_position: Point,
    modifier_key_pressed: bool,
    previous_click: Option<(Point, Instant)>,
    is_dragging: bool,
}

impl Default for KnobState {
    fn default() -> Self {
        Self {
            last_cursor_position: Point::default(),
            modifier_key_pressed: false,
            previous_click: None,
            is_dragging: false,
        }
    }
}

impl Program<Message, Theme> for Knob {
    type State = KnobState;

    fn draw(
        &self,
        _state: &Self::State,
        theme: &Theme,
        bounds: iced_baseview::Rectangle,
        _cursor: iced_baseview::widget::canvas::Cursor,
    ) -> Vec<iced_baseview::widget::canvas::Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let appearance = StyleSheet::active(theme, ());

            self.draw_arc(frame, appearance.arc_empty_color, 1.0);
            self.draw_arc(frame, appearance.arc_filled_color, self.value);

            self.draw_notch(frame, appearance.notch_color, self.value);

            self.draw_marker_dot(frame, 0.0, appearance.end_dot_color);
            self.draw_marker_dot(frame, 1.0, appearance.end_dot_color);

            if let Some(anchor_dot_value) = self.anchor_dot_value {
                self.draw_marker_dot(frame, anchor_dot_value, appearance.anchor_dot_color);
            }
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: iced_baseview::widget::canvas::Event,
        bounds: iced_baseview::Rectangle,
        _cursor: iced_baseview::widget::canvas::Cursor,
    ) -> (
        iced_baseview::widget::canvas::event::Status,
        Option<Message>,
    ) {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if state.is_dragging {
                    let speed = if state.modifier_key_pressed {
                        0.0005
                    } else {
                        0.005
                    };

                    let diff = state.last_cursor_position.y - position.y;
                    let new_value = (self.value + diff * speed).clamp(0.0, 1.0);

                    let message = Message::ChangeSingleParameterSetValue(self.parameter, new_value);

                    state.last_cursor_position = position;

                    return (Status::Captured, Some(message));
                } else {
                    state.last_cursor_position = position;
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonPressed(Button::Left)) => {
                if !state.is_dragging {
                    if let Some((point, time)) = state.previous_click {
                        if point.distance(state.last_cursor_position) < 5.0
                            && time.elapsed() < Duration::from_millis(300)
                        {
                            state.is_dragging = false;
                            state.previous_click = None;

                            let message = Message::ChangeSingleParameterImmediate(
                                self.parameter,
                                self.reset_value,
                            );

                            return (Status::Captured, Some(message));
                        }
                    }

                    if self.is_cursor_over_knob(bounds, state.last_cursor_position) {
                        state.is_dragging = true;
                        state.previous_click = Some((state.last_cursor_position, Instant::now()));

                        let message = Message::ChangeSingleParameterBegin(self.parameter);

                        return (Status::Captured, Some(message));
                    }
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(Button::Left)) => {
                if state.is_dragging {
                    state.is_dragging = false;

                    let message = Message::ChangeSingleParameterEnd(self.parameter);

                    return (Status::Captured, Some(message));
                }
            }
            canvas::Event::Keyboard(
                keyboard::Event::ModifiersChanged(modifiers)
                | keyboard::Event::KeyPressed { modifiers, .. }
                | keyboard::Event::KeyReleased { modifiers, .. },
            ) => {
                state.modifier_key_pressed = modifiers.shift();
            }
            _ => (),
        }

        (iced_baseview::widget::canvas::event::Status::Ignored, None)
    }
}

pub trait StyleSheet {
    type Style: Copy;

    fn active(&self, style: Self::Style) -> Appearance;
}

pub struct Appearance {
    pub arc_empty_color: Color,
    pub arc_filled_color: Color,
    pub notch_color: Color,
    pub anchor_dot_color: Color,
    pub end_dot_color: Color,
}

pub enum KnobVariant {
    Regular,
    Bipolar { center: f32 },
}
