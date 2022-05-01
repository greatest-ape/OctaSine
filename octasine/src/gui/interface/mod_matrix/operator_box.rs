use crate::gui::interface::{Message, SnapPoint, FONT_BOLD, FONT_SIZE};
use iced_baseview::canvas::{event, Frame, Path, Stroke, Text};
use iced_baseview::{mouse, Point, Rectangle, Size};

use super::common::*;
use super::{StyleSheet, OPERATOR_BOX_SCALE};

pub enum OperatorBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

pub enum BoxStatus {
    Normal,
    Hover,
    Dragging { from: Point, original_value: f64 },
}

impl BoxStatus {
    fn is_dragging(&self) -> bool {
        matches!(self, BoxStatus::Dragging { .. })
    }
}

pub struct OperatorBox {
    index: usize,
    text_position: Point,
    path: Path,
    pub center: Point,
    status: BoxStatus,
    last_cursor_position: Point,
    hitbox: Rectangle,
}

impl OperatorBox {
    pub fn new(bounds: Size, index: usize, style_sheet: Box<dyn StyleSheet>) -> Self {
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

        top_left.x += 1.0;
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

        text_position.x += 2.0;
        text_position.y -= 2.0;

        Self {
            index,
            text_position,
            path,
            center,
            status: BoxStatus::Normal,
            last_cursor_position: Point::new(-1.0, -1.0),
            hitbox: rect,
        }
    }

    fn get_parameter_index(&self) -> usize {
        match self.index {
            0 => 4,
            1 => 18,
            2 => 34,
            3 => 50,
            _ => unreachable!(),
        }
    }

    pub fn update(
        &mut self,
        bounds: Rectangle,
        event: event::Event,
        value: f64,
    ) -> OperatorBoxChange {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                self.last_cursor_position = cursor;

                let hit = self.hitbox.contains(cursor);

                match self.status {
                    BoxStatus::Normal if hit => {
                        self.status = BoxStatus::Hover;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Hover if !hit => {
                        self.status = BoxStatus::Normal;

                        return OperatorBoxChange::ClearCache(None);
                    }
                    BoxStatus::Dragging {
                        from,
                        original_value,
                    } => {
                        let change = -(cursor.y - from.y) as f64 / 100.0;

                        return OperatorBoxChange::Update(Message::ChangeSingleParameterSetValue(
                            self.get_parameter_index(),
                            (original_value + change).max(0.0).min(1.0),
                        ));
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if !self.status.is_dragging() && self.hitbox.contains(self.last_cursor_position) {
                    self.status = BoxStatus::Dragging {
                        from: self.last_cursor_position,
                        original_value: value,
                    };

                    return OperatorBoxChange::ClearCache(Some(
                        Message::ChangeSingleParameterBegin(self.get_parameter_index()),
                    ));
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if self.status.is_dragging() {
                    if self.hitbox.contains(self.last_cursor_position) {
                        self.status = BoxStatus::Hover;
                    } else {
                        self.status = BoxStatus::Normal;
                    }

                    return OperatorBoxChange::ClearCache(Some(Message::ChangeSingleParameterEnd(
                        self.get_parameter_index(),
                    )));
                }
            }
            _ => (),
        }

        OperatorBoxChange::None
    }

    pub fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let text = Text {
            content: format!("{}", self.index + 1),
            position: self.text_position,
            font: FONT_BOLD,
            size: FONT_SIZE as f32,
            color: style.text_color,
            ..Default::default()
        };

        let background_color = match self.status {
            BoxStatus::Normal => style.operator_box_color_active,
            BoxStatus::Hover => style.operator_box_color_hover,
            BoxStatus::Dragging { .. } => style.operator_box_color_dragging,
        };

        let stroke = Stroke::default()
            // .with_color(style.box_border_color)
            .with_color(background_color)
            .with_width(1.0);

        frame.fill(&self.path, background_color);
        frame.stroke(&self.path, stroke);
        frame.fill_text(text);
    }
}
