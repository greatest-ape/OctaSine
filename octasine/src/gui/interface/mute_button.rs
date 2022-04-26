use iced_baseview::alignment::{Horizontal, Vertical};
use iced_baseview::canvas::{
    event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::{
    parameters::values::{OperatorActiveValue, ParameterValue},
    GuiSyncHandle,
};

use super::FONT_BOLD;
use super::{style::Theme, Message, FONT_SIZE, LINE_HEIGHT};

const WIDTH: u16 = LINE_HEIGHT;
const HEIGHT: u16 = LINE_HEIGHT;

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
}

pub trait StyleSheet {
    fn volume_on(&self) -> Style;
    fn volume_off(&self) -> Style;
    fn hovered(&self) -> Style;
}

pub struct OperatorMuteButton {
    parameter_index: usize,
    volume_on: bool,
    style: Theme,
    cache: Cache,
    bounds_path: Path,
    cursor_within_bounds: bool,
    click_started: bool,
}

impl OperatorMuteButton {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, parameter_index: usize, style: Theme) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((WIDTH - 1) as f32, (HEIGHT - 1) as f32),
        );

        Self {
            parameter_index,
            volume_on: Self::volume_on(sync_handle.get_parameter(parameter_index)),
            style,
            cache: Cache::new(),
            bounds_path,
            cursor_within_bounds: false,
            click_started: false,
        }
    }

    fn volume_on(sync_value: f64) -> bool {
        OperatorActiveValue::from_sync(sync_value).get() > 0.5
    }

    pub fn set_value(&mut self, value: f64) {
        self.volume_on = Self::volume_on(value);

        self.cache.clear();
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
        frame.fill(&self.bounds_path, style_sheet.volume_on().background_color);
    }

    fn draw_border(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let color = if self.volume_on {
            if self.cursor_within_bounds {
                style_sheet.hovered().border_color
            } else {
                style_sheet.volume_on().border_color
            }
        } else {
            style_sheet.volume_off().border_color
        };

        let stroke = Stroke::default().with_color(color);

        frame.stroke(&self.bounds_path, stroke);
    }

    fn draw_text(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let color = if self.volume_on {
            if self.cursor_within_bounds {
                style_sheet.hovered().text_color
            } else {
                style_sheet.volume_on().text_color
            }
        } else {
            style_sheet.volume_off().text_color
        };

        let position = Point::new(f32::from(WIDTH) / 2.0, f32::from(HEIGHT) / 2.0);

        let text = Text {
            content: "M".into(),
            color,
            size: f32::from(FONT_SIZE),
            font: FONT_BOLD,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            position,
            ..Default::default()
        };

        frame.fill_text(text);
    }
}

impl Program<Message> for OperatorMuteButton {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame, self.style.into());
            self.draw_border(frame, self.style.into());
            self.draw_text(frame, self.style.into());
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
                iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right,
            )) if self.click_started => {
                if self.cursor_within_bounds {
                    let message = {
                        let sync_value = if self.volume_on { 0.0 } else { 1.0 };

                        Message::ChangeSingleParameterImmediate(self.parameter_index, sync_value)
                    };

                    (event::Status::Captured, Some(message))
                } else {
                    self.click_started = false;

                    (event::Status::Ignored, None)
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
