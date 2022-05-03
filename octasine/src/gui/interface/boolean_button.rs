use iced_baseview::alignment::{Horizontal, Vertical};
use iced_baseview::canvas::{
    event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::parameter_values::lfo_mode::LfoMode;
use crate::parameter_values::{LfoBpmSyncValue, LfoModeValue, OperatorActiveValue, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::{style::Theme, Message, FONT_SIZE, LINE_HEIGHT};

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
}

pub trait StyleSheet {
    fn active(&self) -> Style;
    fn active_hover(&self) -> Style;
    fn inactive(&self) -> Style;
    fn inactive_hover(&self) -> Style;
}

pub fn operator_button<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
    operator_index: usize,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        parameter_index,
        style,
        LINE_HEIGHT * 3,
        LINE_HEIGHT * 3 / 2,
        |v| OperatorActiveValue::new_from_patch(v).get() == 1.0,
        |is_on| {
            if is_on {
                1.0
            } else {
                0.0
            }
        },
        |theme| theme.mute_button(),
        |theme, style, text| Text {
            content: text,
            color: style.text_color,
            size: f32::from(FONT_SIZE + FONT_SIZE / 2),
            font: theme.font_heading(),
            ..Default::default()
        },
        format!("OP {}", operator_index + 1),
    )
}

pub fn lfo_bpm_sync_button<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        parameter_index,
        style,
        LINE_HEIGHT * 2,
        LINE_HEIGHT * 3 / 2,
        |v| LfoBpmSyncValue::new_from_patch(v).get(),
        |on| LfoBpmSyncValue::new_from_audio(on).to_patch(),
        |theme| theme.bpm_sync_button(),
        |theme, style, text| Text {
            content: text,
            color: style.text_color,
            size: f32::from(FONT_SIZE),
            font: theme.font_regular(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            position: Point::new(0.0, 0.0),
            ..Default::default()
        },
        "BPM".to_string(),
    )
}

pub fn lfo_mode_button<H: GuiSyncHandle>(
    sync_handle: &H,
    parameter_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        parameter_index,
        style,
        LINE_HEIGHT * 2,
        LINE_HEIGHT * 3 / 2,
        |v| LfoModeValue::new_from_patch(v).get() == LfoMode::Once,
        |is_oneshot| {
            if is_oneshot {
                LfoModeValue::new_from_audio(LfoMode::Once).to_patch()
            } else {
                LfoModeValue::new_from_audio(LfoMode::Forever).to_patch()
            }
        },
        |theme| theme.bpm_sync_button(),
        |theme, style, text| Text {
            content: text,
            color: style.text_color,
            size: f32::from(FONT_SIZE),
            font: theme.font_regular(),
            ..Default::default()
        },
        "ONE".to_string(),
    )
}

pub struct BooleanButton {
    parameter_index: usize,
    on: bool,
    style: Theme,
    cache: Cache,
    bounds_path: Path,
    cursor_within_bounds: bool,
    click_started: bool,
    patch_value_to_is_on: fn(f64) -> bool,
    is_on_to_patch_value: fn(bool) -> f64,
    get_stylesheet: fn(Theme) -> Box<dyn StyleSheet>,
    make_text: fn(Theme, Style, String) -> Text,
    text_content: String,
    width: u16,
    height: u16,
}

impl BooleanButton {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter_index: usize,
        style: Theme,
        width: u16,
        height: u16,
        f: fn(f64) -> bool,
        g: fn(bool) -> f64,
        h: fn(Theme) -> Box<dyn StyleSheet>,
        make_text: fn(Theme, Style, String) -> Text,
        text_content: String,
    ) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((width - 1) as f32, (height - 1) as f32),
        );

        Self {
            parameter_index,
            on: f(sync_handle.get_parameter(parameter_index)),
            style,
            cache: Cache::new(),
            bounds_path,
            cursor_within_bounds: false,
            click_started: false,
            patch_value_to_is_on: f,
            is_on_to_patch_value: g,
            get_stylesheet: h,
            make_text,
            text_content,
            width,
            height,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.on = (self.patch_value_to_is_on)(value);

        self.cache.clear();
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.cache.clear();
    }

    pub fn view(&mut self) -> Element<Message> {
        let width = self.width;
        let height = self.height;

        Canvas::new(self)
            .width(Length::Units(width))
            .height(Length::Units(height))
            .into()
    }

    fn style(&self) -> Style {
        let stylesheet = (self.get_stylesheet)(self.style);

        match (self.on, self.cursor_within_bounds) {
            (true, false) => stylesheet.active(),
            (true, true) => stylesheet.active_hover(),
            (false, false) => stylesheet.inactive(),
            (false, true) => stylesheet.inactive_hover(),
        }
    }

    fn draw_background(&self, frame: &mut Frame) {
        frame.fill(&self.bounds_path, self.style().background_color);
    }

    fn draw_border(&self, frame: &mut Frame) {
        let stroke = Stroke::default().with_color(self.style().border_color);

        frame.stroke(&self.bounds_path, stroke);
    }

    fn draw_text(&self, frame: &mut Frame) {
        let text = (self.make_text)(self.style, self.style(), self.text_content.clone());

        let position = Point::new(f32::from(self.width) / 2.0, f32::from(self.height) / 2.0);

        let text = Text {
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            position,
            ..text
        };

        frame.fill_text(text);
    }
}

impl Program<Message> for BooleanButton {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(frame);
            self.draw_border(frame);
            self.draw_text(frame);
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
                        let patch_value = (self.is_on_to_patch_value)(!self.on);

                        Message::ChangeSingleParameterImmediate(self.parameter_index, patch_value)
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
