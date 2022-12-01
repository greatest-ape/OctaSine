use iced_baseview::alignment::{Horizontal, Vertical};
use iced_baseview::canvas::{
    event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::parameters::lfo_mode::LfoMode;
use crate::parameters::operator_envelope::OperatorEnvelopeGroupValue;
use crate::parameters::{
    LfoActiveValue, LfoBpmSyncValue, LfoModeValue, LfoParameter, OperatorActiveValue,
    OperatorParameter, Parameter, ParameterValue,
};
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

pub fn operator_mute_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Active),
        style,
        "M",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| OperatorActiveValue::new_from_patch(v).get() == 0.0,
        |is_muted| {
            if is_muted {
                0.0
            } else {
                1.0
            }
        },
        |theme| theme.mute_button(),
    )
}

pub fn lfo_bpm_sync_button<H: GuiSyncHandle>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::BpmSync),
        style,
        "B",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| LfoBpmSyncValue::new_from_patch(v).get(),
        |on| LfoBpmSyncValue::new_from_audio(on).to_patch(),
        |theme| theme.bpm_sync_button(),
    )
}

pub fn lfo_mode_button<H: GuiSyncHandle>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Mode),
        style,
        "1",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| LfoModeValue::new_from_patch(v).get() == LfoMode::Once,
        |is_oneshot| {
            if is_oneshot {
                LfoModeValue::new_from_audio(LfoMode::Once).to_patch()
            } else {
                LfoModeValue::new_from_audio(LfoMode::Forever).to_patch()
            }
        },
        |theme| theme.bpm_sync_button(),
    )
}

pub fn lfo_active_button<H: GuiSyncHandle>(
    sync_handle: &H,
    lfo_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Active),
        style,
        "M",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| LfoActiveValue::new_from_patch(v).get() == 0.0,
        |is_muted| {
            if is_muted {
                0.0
            } else {
                1.0
            }
        },
        |theme| theme.mute_button(),
    )
}

pub fn envelope_group_a_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup),
        style,
        "A",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| OperatorEnvelopeGroupValue::new_from_patch(v).get() == OperatorEnvelopeGroupValue::A,
        |is_active| {
            if is_active {
                OperatorEnvelopeGroupValue::A.to_patch()
            } else {
                OperatorEnvelopeGroupValue::Off.to_patch()
            }
        },
        |theme| theme.envelope_group_button(),
    )
}

pub fn envelope_group_b_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
    style: Theme,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup),
        style,
        "B",
        LINE_HEIGHT,
        LINE_HEIGHT,
        |v| OperatorEnvelopeGroupValue::new_from_patch(v).get() == OperatorEnvelopeGroupValue::B,
        |is_active| {
            if is_active {
                OperatorEnvelopeGroupValue::B.to_patch()
            } else {
                OperatorEnvelopeGroupValue::Off.to_patch()
            }
        },
        |theme| theme.envelope_group_button(),
    )
}

pub struct BooleanButton {
    parameter: Parameter,
    on: bool,
    style: Theme,
    cache: Cache,
    bounds_path: Path,
    patch_value_to_is_on: fn(f32) -> bool,
    is_on_to_patch_value: fn(bool) -> f32,
    get_stylesheet: fn(Theme) -> Box<dyn StyleSheet>,
    text: &'static str,
    width: u16,
    height: u16,
}

impl BooleanButton {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        style: Theme,
        text: &'static str,
        width: u16,
        height: u16,
        f: fn(f32) -> bool,
        g: fn(bool) -> f32,
        h: fn(Theme) -> Box<dyn StyleSheet>,
    ) -> Self {
        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((width - 1) as f32, (height - 1) as f32),
        );

        Self {
            parameter,
            on: f(sync_handle.get_parameter(parameter)),
            style,
            cache: Cache::new(),
            bounds_path,
            patch_value_to_is_on: f,
            is_on_to_patch_value: g,
            get_stylesheet: h,
            text: text.into(),
            width,
            height,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.on = (self.patch_value_to_is_on)(value);

        self.cache.clear();
    }

    pub fn set_style(&mut self, style: Theme) {
        self.style = style;

        self.cache.clear();
    }

    pub fn view(&self) -> Element<Message> {
        let width = self.width;
        let height = self.height;

        Canvas::new(self)
            .width(Length::Units(width))
            .height(Length::Units(height))
            .into()
    }

    fn style(&self, state: &CanvasState) -> Style {
        let stylesheet = (self.get_stylesheet)(self.style);

        match (self.on, state.cursor_within_bounds) {
            (true, false) => stylesheet.active(),
            (true, true) => stylesheet.active_hover(),
            (false, false) => stylesheet.inactive(),
            (false, true) => stylesheet.inactive_hover(),
        }
    }

    fn draw_background(&self, state: &CanvasState, frame: &mut Frame) {
        frame.fill(&self.bounds_path, self.style(state).background_color);
    }

    fn draw_border(&self, state: &CanvasState, frame: &mut Frame) {
        let stroke = Stroke::default().with_color(self.style(state).border_color);

        frame.stroke(&self.bounds_path, stroke);
    }

    fn draw_text(&self, state: &CanvasState, frame: &mut Frame) {
        let position = Point::new(f32::from(self.width) / 2.0, f32::from(self.height) / 2.0);

        let text = Text {
            content: self.text.to_string(),
            color: self.style(state).text_color,
            size: f32::from(FONT_SIZE),
            font: self.style.font_regular(),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            position,
            ..Default::default()
        };

        frame.fill_text(text);
    }
}

#[derive(Default)]
pub struct CanvasState {
    cursor_within_bounds: bool,
    click_started: bool,
}

impl Program<Message> for BooleanButton {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        _theme: &iced_baseview::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_background(state, frame);
            self.draw_border(state, frame);
            self.draw_text(state, frame);
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
                iced_baseview::mouse::Button::Left | iced_baseview::mouse::Button::Right,
            )) if state.click_started => {
                if state.cursor_within_bounds {
                    let message = {
                        let patch_value = (self.is_on_to_patch_value)(!self.on);

                        Message::ChangeSingleParameterImmediate(self.parameter, patch_value)
                    };

                    (event::Status::Captured, Some(message))
                } else {
                    state.click_started = false;

                    (event::Status::Ignored, None)
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
