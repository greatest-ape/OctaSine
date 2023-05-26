use iced_baseview::alignment::{Horizontal, Vertical};
use iced_baseview::widget::canvas::{
    event, Cache, Canvas, Cursor, Frame, Geometry, Path, Program, Stroke, Text,
};
use iced_baseview::{Color, Element, Length, Point, Rectangle, Size};

use crate::parameters::lfo_key_sync::LfoKeySyncValue;
use crate::parameters::lfo_mode::LfoMode;
use crate::parameters::operator_envelope::OperatorEnvelopeGroupValue;
use crate::parameters::{
    LfoActiveValue, LfoBpmSyncValue, LfoModeValue, LfoParameter, OperatorActiveValue,
    OperatorParameter, Parameter, ParameterValue, WrappedParameter,
};
use crate::sync::GuiSyncHandle;

use super::style::boolean_button::BooleanButtonStyle;
use super::SnapPoint;
use super::{style::Theme, Message, FONT_SIZE, LINE_HEIGHT};

#[derive(Debug, Clone)]
pub struct Appearance {
    pub background_color: Color,
    pub border_color: Color,
    pub text_color: Color,
}

pub trait StyleSheet {
    type Style: Default + Copy;

    fn active(&self, style: &Self::Style, hover: bool) -> Appearance;
    fn inactive(&self, style: &Self::Style, hover: bool) -> Appearance;
}

pub fn operator_mute_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::Active),
        "M",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| OperatorActiveValue::new_from_patch(v).get() == 0.0,
        |is_muted| {
            if is_muted {
                0.0
            } else {
                1.0
            }
        },
        BooleanButtonStyle::Mute,
    )
}

pub fn lfo_bpm_sync_button<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::BpmSync),
        "B",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| LfoBpmSyncValue::new_from_patch(v).get(),
        |on| LfoBpmSyncValue::new_from_audio(on).to_patch(),
        BooleanButtonStyle::Regular,
    )
}

pub fn lfo_key_sync_button<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::KeySync),
        "K",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| LfoKeySyncValue::new_from_patch(v).get(),
        |on| LfoKeySyncValue::new_from_audio(on).to_patch(),
        BooleanButtonStyle::Regular,
    )
}

pub fn lfo_mode_button<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Mode),
        "1",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| LfoModeValue::new_from_patch(v).get() == LfoMode::Once,
        |is_oneshot| {
            if is_oneshot {
                LfoModeValue::new_from_audio(LfoMode::Once).to_patch()
            } else {
                LfoModeValue::new_from_audio(LfoMode::Forever).to_patch()
            }
        },
        BooleanButtonStyle::Regular,
    )
}

pub fn lfo_active_button<H: GuiSyncHandle>(sync_handle: &H, lfo_index: usize) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Lfo(lfo_index as u8, LfoParameter::Active),
        "M",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| LfoActiveValue::new_from_patch(v).get() == 0.0,
        |is_muted| {
            if is_muted {
                0.0
            } else {
                1.0
            }
        },
        BooleanButtonStyle::Mute,
    )
}

pub fn envelope_group_a_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup),
        "A",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| OperatorEnvelopeGroupValue::new_from_patch(v).get() == OperatorEnvelopeGroupValue::A,
        |is_active| {
            if is_active {
                OperatorEnvelopeGroupValue::A.to_patch()
            } else {
                OperatorEnvelopeGroupValue::Off.to_patch()
            }
        },
        BooleanButtonStyle::Regular,
    )
}

pub fn envelope_group_b_button<H: GuiSyncHandle>(
    sync_handle: &H,
    operator_index: usize,
) -> BooleanButton {
    BooleanButton::new(
        sync_handle,
        Parameter::Operator(operator_index as u8, OperatorParameter::EnvelopeLockGroup),
        "B",
        LINE_HEIGHT,
        LINE_HEIGHT,
        Default::default(),
        |v| OperatorEnvelopeGroupValue::new_from_patch(v).get() == OperatorEnvelopeGroupValue::B,
        |is_active| {
            if is_active {
                OperatorEnvelopeGroupValue::B.to_patch()
            } else {
                OperatorEnvelopeGroupValue::Off.to_patch()
            }
        },
        BooleanButtonStyle::Regular,
    )
}

#[derive(Debug, Clone, Copy, Default)]
pub enum BooleanButtonTextAlignment {
    #[default]
    Center,
    Offsets { x: f32, y: f32 }
}

pub struct BooleanButton {
    parameter: WrappedParameter,
    on: bool,
    cache: Cache,
    bounds_path: Path,
    patch_value_to_is_on: fn(f32) -> bool,
    is_on_to_patch_value: fn(bool) -> f32,
    button_style: BooleanButtonStyle,
    text: &'static str,
    width: u16,
    height: u16,
    text_alignment: BooleanButtonTextAlignment,
}

impl BooleanButton {
    pub fn new<H: GuiSyncHandle>(
        sync_handle: &H,
        parameter: Parameter,
        text: &'static str,
        width: u16,
        height: u16,
        text_alignment: BooleanButtonTextAlignment,
        f: fn(f32) -> bool,
        g: fn(bool) -> f32,
        button_style: BooleanButtonStyle,
    ) -> Self {
        let parameter = parameter.into();

        let bounds_path = Path::rectangle(
            Point::new(0.5, 0.5),
            Size::new((width - 1) as f32, (height - 1) as f32),
        );

        Self {
            parameter,
            on: f(sync_handle.get_parameter(parameter)),
            cache: Cache::new(),
            bounds_path,
            patch_value_to_is_on: f,
            is_on_to_patch_value: g,
            button_style,
            text,
            width,
            height,
            text_alignment,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.on = (self.patch_value_to_is_on)(value);

        self.cache.clear();
    }

    pub fn theme_changed(&mut self) {
        self.cache.clear();
    }

    pub fn view(&self) -> Element<Message, Theme> {
        let width = self.width;
        let height = self.height;

        Canvas::new(self)
            .width(Length::Fixed(width.into()))
            .height(Length::Fixed(height.into()))
            .into()
    }

    fn appearance(&self, state: &CanvasState, theme: &Theme) -> Appearance {
        let hover = state.cursor_within_bounds;

        if self.on {
            theme.active(&self.button_style, hover)
        } else {
            theme.inactive(&self.button_style, hover)
        }
    }

    fn draw_background(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        frame.fill(
            &self.bounds_path,
            self.appearance(state, theme).background_color,
        );
    }

    fn draw_border(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        let stroke = Stroke::default().with_color(self.appearance(state, theme).border_color);

        frame.stroke(&self.bounds_path, stroke);
    }

    fn draw_text(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {
        let mut text = Text {
            content: self.text.to_string(),
            color: self.appearance(state, theme).text_color,
            size: f32::from(FONT_SIZE),
            font: theme.font_regular(),
            ..Default::default()
        };

        match self.text_alignment {
            BooleanButtonTextAlignment::Center => {
                text.position = Point::new(f32::from(self.width) / 2.0, f32::from(self.height) / 2.0);
                text.horizontal_alignment = Horizontal::Center;
                text.vertical_alignment = Vertical::Center;
            }
            BooleanButtonTextAlignment::Offsets { x, y } => {
                text.position = Point::new(x, y);
            }
        }

        ::log::info!("{:?}: position: {:?}", self.parameter.parameter().name(), text.position);

        frame.fill_text(text);
    }
}

#[derive(Default)]
pub struct CanvasState {
    cursor_within_bounds: bool,
    click_started: bool,
}

impl Program<Message, Theme> for BooleanButton {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            ::log::info!("{:?}: bounds: {:?}", self.parameter.parameter().name(), bounds);
            self.draw_background(state, frame, theme);
            self.draw_border(state, frame, theme);
            self.draw_text(state, frame, theme);
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
