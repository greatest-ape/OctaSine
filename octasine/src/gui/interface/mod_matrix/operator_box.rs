use crate::gui::interface::style::Theme;
use crate::gui::interface::{Message, SnapPoint, FONT_SIZE};
use crate::parameters::{OperatorParameter, Parameter};
use iced_baseview::widget::canvas::{event, Frame, Path, Stroke, Text};
use iced_baseview::{mouse, Point, Rectangle, Size};

use super::common::*;
use super::OPERATOR_BOX_SCALE;

#[derive(Default)]
pub struct OperatorBoxCanvasState {
    status: BoxStatus,
    last_cursor_position: Point,
}

pub enum OperatorBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

pub struct OperatorBox {
    index: usize,
    text_position: Point,
    path: Path,
    center: Point,
    hitbox: Rectangle,
}

impl OperatorBox {
    pub fn new(bounds: Size, index: usize) -> Self {
        let (x, y) = match index {
            3 => (0, 0),
            2 => (2, 2),
            1 => (4, 4),
            0 => (6, 6),
            _ => unreachable!(),
        };

        let (base_top_left, base_size) = get_box_base_point_and_size(bounds, x, y);

        let size = Size {
            width: base_size.width * OPERATOR_BOX_SCALE,
            height: base_size.height * OPERATOR_BOX_SCALE,
        };
        let top_left = Point {
            x: base_top_left.x - (OPERATOR_BOX_SCALE - 1.0) * base_size.width / 2.0,
            y: base_top_left.y - (OPERATOR_BOX_SCALE - 1.0) * base_size.height / 2.0,
        };

        let mut top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        top_left.x -= 1.0;
        top_left = top_left.snap();

        let path = Path::rectangle(top_left, size);
        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let text_position = Point {
            x: base_top_left.x,
            y: base_top_left.y,
        };

        let mut text_position = scale_point(bounds, text_position);

        text_position = text_position.snap();

        text_position.x += 0.0;
        text_position.y -= 2.0;

        Self {
            index,
            text_position,
            path,
            center,
            hitbox: rect,
        }
    }

    fn get_parameter(&self) -> Parameter {
        Parameter::Operator(self.index as u8, OperatorParameter::MixOut)
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn update(
        &self,
        state: &mut OperatorBoxCanvasState,
        bounds: Rectangle,
        event: event::Event,
        value: f32,
    ) -> OperatorBoxChange {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                state.last_cursor_position = cursor;

                let hit = self.hitbox.contains(cursor);

                match state.status {
                    BoxStatus::Normal if hit => {
                        state.status = BoxStatus::Hover;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Hover if !hit => {
                        state.status = BoxStatus::Normal;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Dragging {
                        from,
                        original_value,
                    } => {
                        let change = -(cursor.y - from.y) / 100.0;

                        return OperatorBoxChange::Update(Message::ChangeSingleParameterSetValue(
                            self.get_parameter(),
                            (original_value + change).max(0.0).min(1.0),
                        ));
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if !state.status.is_dragging() && self.hitbox.contains(state.last_cursor_position) {
                    state.status = BoxStatus::Dragging {
                        from: state.last_cursor_position,
                        original_value: value,
                    };

                    return OperatorBoxChange::ClearCache(Some(
                        Message::ChangeSingleParameterBegin(self.get_parameter()),
                    ));
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.status.is_dragging() {
                    if self.hitbox.contains(state.last_cursor_position) {
                        state.status = BoxStatus::Hover;
                    } else {
                        state.status = BoxStatus::Normal;
                    }

                    return OperatorBoxChange::ClearCache(Some(Message::ChangeSingleParameterEnd(
                        self.get_parameter(),
                    )));
                }
            }
            _ => (),
        }

        OperatorBoxChange::None
    }

    pub fn draw(&self, state: &OperatorBoxCanvasState, frame: &mut Frame, style: Theme) {
        let font_bold = style.font_bold();
        let style = style.mod_matrix().appearance();

        let text = Text {
            content: format!("{}", self.index + 1),
            position: self.text_position,
            font: font_bold,
            size: FONT_SIZE as f32,
            color: style.text_color,
            ..Default::default()
        };

        let background_color = match state.status {
            BoxStatus::Normal => style.operator_box_color_active,
            BoxStatus::Hover => style.operator_box_color_hover,
            BoxStatus::Dragging { .. } => style.operator_box_color_dragging,
        };

        let border_color = if let Some(color) = style.operator_box_border_color {
            color
        } else {
            background_color
        };

        let stroke = Stroke::default().with_color(border_color).with_width(1.0);

        frame.fill(&self.path, background_color);
        frame.stroke(&self.path, stroke);
        frame.fill_text(text);
    }
}
