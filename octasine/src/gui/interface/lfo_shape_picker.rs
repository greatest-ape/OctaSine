use iced_baseview::canvas::{
    event, path, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke,
};
use iced_baseview::{
    alignment::Horizontal, Alignment, Color, Column, Element, Length, Point, Rectangle, Size,
    Space, Text,
};

use crate::common::{LfoShape, Phase};
use crate::constants::LFO_SHAPE_STEPS;
use crate::parameters::values::{LfoShapeValue, ParameterValue};
use crate::voices::lfos::VoiceLfo;
use crate::GuiSyncHandle;

use super::style::Theme;
use super::{Message, FONT_BOLD, LINE_HEIGHT};

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

pub struct LfoShapePicker {
    parameter_index: usize,
    cache: Cache,
    bounds_path: Path,
    cursor_within_bounds: bool,
    click_started: bool,
    style: Theme,
    shape: LfoShape,
    value_text: String,
}

impl LfoShapePicker {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter_index: usize, style: Theme) -> Self {
        let value = LfoShapeValue::from_sync(sync_handle.get_parameter(parameter_index));
        let shape = value.get();
        let value_text = value.format();
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );

        Self {
            parameter_index,
            cache: Cache::new(),
            bounds_path,
            cursor_within_bounds: false,
            click_started: false,
            style,
            shape,
            value_text,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("SHAPE")
            .horizontal_alignment(Horizontal::Center)
            .font(FONT_BOLD);

        let value = Text::new(self.value_text.clone()).horizontal_alignment(Horizontal::Center);

        let canvas = Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT));

        Column::new()
            .width(Length::Units(LINE_HEIGHT * 4))
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(canvas)
            .push(Space::with_height(Length::Units(LINE_HEIGHT)))
            .push(value)
            .into()
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    pub fn set_value(&mut self, value: f64) {
        let value = LfoShapeValue::from_sync(value);
        let shape = value.get();

        if self.shape != shape {
            self.shape = shape;
            self.value_text = value.format();

            self.cache.clear();
        }
    }

    fn draw_background(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        frame.fill(&self.bounds_path, style.background_color);
    }

    fn draw_border(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let color = if self.cursor_within_bounds {
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

    fn draw_shape_line(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let mut path = path::Builder::new();

        for i in 0..WIDTH - 1 {
            let phase = Phase((i as f64) / (WIDTH - 1) as f64);
            let y = VoiceLfo::calculate_curve(self.shape, phase) as f32;

            let visual_y = HEIGHT_MIDDLE - y * SHAPE_HEIGHT_RANGE;
            let visual_x = 0.5 + i as f32;

            if i == 0 {
                path.move_to(Point::new(visual_x, visual_y))
            } else {
                path.line_to(Point::new(visual_x, visual_y))
            }
        }

        let path = path.build();

        let color = if self.cursor_within_bounds {
            style.shape_line_color_hovered
        } else {
            style.shape_line_color_active
        };

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&path, stroke)
    }
}

impl Program<Message> for LfoShapePicker {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, self.style.into());
            self.draw_middle_line(frame, self.style.into());
            self.draw_shape_line(frame, self.style.into());
            self.draw_border(frame, self.style.into());
        });

        vec![geometry]
    }

    fn update(
        &mut self,
        event: event::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved { position }) => {
                let cursor_within_bounds = bounds.contains(position);

                if self.cursor_within_bounds != cursor_within_bounds {
                    self.cursor_within_bounds = cursor_within_bounds;

                    self.cache.clear();
                }

                (event::Status::Ignored, None)
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(
                iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right,
            )) if self.cursor_within_bounds => {
                self.click_started = true;

                (event::Status::Captured, None)
            }
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(
                button @ (iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right),
            )) if self.click_started => {
                if self.cursor_within_bounds {
                    let shape_index = LFO_SHAPE_STEPS
                        .iter()
                        .position(|s| *s == self.shape)
                        .unwrap();

                    let new_shape_index = match button {
                        iced_baseview::mouse::Button::Left => {
                            (shape_index + 1) % LFO_SHAPE_STEPS.len()
                        }
                        iced_baseview::mouse::Button::Right if shape_index == 0 => {
                            LFO_SHAPE_STEPS.len() - 1
                        }
                        iced_baseview::mouse::Button::Right => shape_index - 1,
                        _ => unreachable!(),
                    };

                    let new_shape = LFO_SHAPE_STEPS[new_shape_index];
                    let new_value = LfoShapeValue::from_processing(new_shape).to_sync();

                    self.set_value(new_value);
                    self.click_started = false;

                    (
                        event::Status::Captured,
                        Some(Message::ChangeSingleParameterImmediate(
                            self.parameter_index,
                            new_value,
                        )),
                    )
                } else {
                    self.click_started = false;

                    (event::Status::Ignored, None)
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
