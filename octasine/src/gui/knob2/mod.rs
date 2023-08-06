use std::f32::consts::PI;

use iced_baseview::{
    alignment::Horizontal,
    widget::{
        canvas::{path::Arc, Cache, Frame, Path, Program, Stroke},
        Canvas, Column, Container, Text,
    },
    Color, Element, Length, Point,
};

use super::{
    style::{knob2::KnobStyle, Theme},
    Message, LINE_HEIGHT,
};

const KNOB_SIZE: u16 = LINE_HEIGHT * 2;
const CANVAS_SIZE: u16 = KNOB_SIZE * 2;

const ARC_EXTRA_ANGLE: f32 = PI * 0.25;
const ARC_START_ANGLE: f32 = PI - ARC_EXTRA_ANGLE;
const ARC_END_ANGLE_ADDITION: f32 = PI + 2.0 * ARC_EXTRA_ANGLE;

fn arc_angle(value: f32) -> f32 {
    ARC_START_ANGLE + value * ARC_END_ANGLE_ADDITION
}

pub struct Knob {
    canvas: KnobCanvas,
}

impl Knob {
    pub fn new() -> Self {
        let center_x = KNOB_SIZE;
        let center_y = LINE_HEIGHT + KNOB_SIZE / 2;
        let center = Point::new((center_x).into(), (center_y).into());

        Self {
            canvas: KnobCanvas {
                cache: Cache::default(),
                center,
                radius: (KNOB_SIZE / 2) as f32,
                style: Default::default(),
            },
        }
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
                Container::new(self.canvas.view(theme))
                    .width(CANVAS_SIZE)
                    .height(CANVAS_SIZE),
            )
            .push(value)
            .into()
    }
}

struct KnobCanvas {
    cache: Cache,
    center: Point,
    radius: f32,
    style: KnobStyle,
}

impl KnobCanvas {
    fn view(&self, theme: &Theme) -> Element<Message, Theme> {
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

    fn draw_marker(&self, frame: &mut Frame, value: f32, color: Color) {
        let path = Path::new(|builder| {
            let angle = arc_angle(value);

            let bla = (KNOB_SIZE / 4) as f32;

            let x_addition = angle.cos() * (self.radius + bla);
            let y_addition = angle.sin() * (self.radius + bla);

            let mut point = self.center;

            point.x += x_addition;
            point.y += y_addition;

            builder.circle(point, 1.0)
        });

        let stroke = Stroke::default()
            .with_color(color)
            .with_width(2.0)
            .with_line_cap(iced_audio::knob::LineCap::Round);

        frame.stroke(&path, stroke);
    }
}

struct KnobCanvasState {
    value: f32,
}

impl Default for KnobCanvasState {
    fn default() -> Self {
        Self { value: 0.5 }
    }
}

impl Program<Message, Theme> for KnobCanvas {
    type State = KnobCanvasState;

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: iced_baseview::Rectangle,
        cursor: iced_baseview::widget::canvas::Cursor,
    ) -> Vec<iced_baseview::widget::canvas::Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            let appearance = StyleSheet::active(theme, self.style);

            self.draw_arc(frame, appearance.arc_empty, 1.0);
            self.draw_arc(frame, appearance.arc_filled, state.value);

            self.draw_notch(frame, appearance.notch, state.value);

            self.draw_marker(frame, 0.0, appearance.secondary_marker);
            self.draw_marker(frame, 1.0, appearance.secondary_marker);
        });

        vec![geometry]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced_baseview::Rectangle,
        _cursor: iced_baseview::widget::canvas::Cursor,
    ) -> iced_baseview::mouse::Interaction {
        iced_baseview::mouse::Interaction::Pointer
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced_baseview::widget::canvas::Event,
        _bounds: iced_baseview::Rectangle,
        _cursor: iced_baseview::widget::canvas::Cursor,
    ) -> (
        iced_baseview::widget::canvas::event::Status,
        Option<Message>,
    ) {
        (iced_baseview::widget::canvas::event::Status::Ignored, None)
    }
}

pub trait StyleSheet {
    type Style: Copy;

    fn active(&self, style: Self::Style) -> Appearance;
}

pub struct Appearance {
    pub arc_empty: Color,
    pub arc_filled: Color,
    pub notch: Color,
    pub primary_marker: Color,
    pub secondary_marker: Color,
}
