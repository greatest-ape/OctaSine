use iced_baseview::canvas::event;
use iced_baseview::{Point, Rectangle};

use crate::gui::interface::Message;
use crate::parameters::operator_envelope::{ENVELOPE_MAX_DURATION, ENVELOPE_MIN_DURATION};
use crate::parameters::{OperatorParameter, Parameter};

use super::common::*;
use super::EnvelopeCanvas;

/// Canvas event handlers
impl EnvelopeCanvas {
    pub fn handle_button_pressed(
        &self,
        state: &mut EnvelopeCanvasState,
        bounds: Rectangle,
    ) -> (event::Status, Option<Message>) {
        if bounds.contains(state.last_cursor_position) {
            let relative_position = Point::new(
                state.last_cursor_position.x - bounds.x,
                state.last_cursor_position.y - bounds.y,
            );

            if self.release_dragger.hitbox.contains(relative_position)
                && !state.release_dragger_status.is_dragging()
            {
                state.release_dragger_status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.release_duration,
                    original_end_value: 0.0,
                };
            } else if self.decay_dragger.hitbox.contains(relative_position)
                && !state.decay_dragger_status.is_dragging()
            {
                state.decay_dragger_status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.decay_duration,
                    original_end_value: self.sustain_volume,
                };
            } else if self.attack_dragger.hitbox.contains(relative_position)
                && !state.attack_dragger_status.is_dragging()
            {
                state.attack_dragger_status = EnvelopeDraggerStatus::Dragging {
                    from: state.last_cursor_position,
                    original_duration: self.attack_duration,
                    original_end_value: 1.0,
                };
            } else {
                let pos_in_bounds = state.last_cursor_position.x - bounds.x;
                let pos_in_viewport =
                    (pos_in_bounds - (WIDTH as f32 * (1.0 - ENVELOPE_PATH_SCALE_X)) / 2.0).max(0.0);
                let pos_in_viewport =
                    (pos_in_viewport / (WIDTH as f32 * ENVELOPE_PATH_SCALE_X)).min(1.0);

                state.dragging_background_from = Some(DraggingBackground {
                    from_point: state.last_cursor_position,
                    original_visible_position: pos_in_viewport,
                    original_x_offset: self.x_offset,
                    viewport_factor: self.viewport_factor,
                });

                if state.double_click_data.is_none() {
                    state.double_click_data = Some(DoubleClickData {
                        point: state.last_cursor_position,
                        releases: 0,
                    });
                }
            }

            self.cache.clear();

            (event::Status::Captured, None)
        } else {
            (event::Status::Ignored, None)
        }
    }

    pub fn handle_cursor_moved(
        &self,
        state: &mut EnvelopeCanvasState,
        bounds: Rectangle,
        x: f32,
        y: f32,
    ) -> (event::Status, Option<Message>) {
        state.last_cursor_position = Point::new(x, y);

        if let Some(data) = state.double_click_data {
            if data.point != state.last_cursor_position {
                state.double_click_data = None;
            }
        }

        let relative_position = Point::new(x - bounds.x, y - bounds.y);

        let attack_hitbox_hit = self.attack_dragger.hitbox.contains(relative_position);

        match state.attack_dragger_status {
            EnvelopeDraggerStatus::Normal => {
                if attack_hitbox_hit {
                    state.attack_dragger_status = EnvelopeDraggerStatus::Hover;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !attack_hitbox_hit {
                    state.attack_dragger_status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                ..
            } => {
                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(self.operator_index, OperatorParameter::AttackDuration),
                        dragging_to_duration(self.viewport_factor, x, from, original_duration)
                            as f32,
                    ),
                    parameter_2: None,
                };

                return (event::Status::Captured, Some(message));
            }
        }

        let decay_hitbox_hit = self.decay_dragger.hitbox.contains(relative_position);

        if decay_hitbox_hit {
            state
                .attack_dragger_status
                .set_to_normal_if_in_hover_state();
        }

        match state.decay_dragger_status {
            EnvelopeDraggerStatus::Normal => {
                if decay_hitbox_hit {
                    state.decay_dragger_status = EnvelopeDraggerStatus::Hover;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !decay_hitbox_hit {
                    state.decay_dragger_status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                original_end_value,
            } => {
                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(self.operator_index, OperatorParameter::DecayDuration),
                        dragging_to_duration(self.viewport_factor, x, from, original_duration)
                            as f32,
                    ),
                    parameter_2: Some((
                        Parameter::Operator(self.operator_index, OperatorParameter::SustainVolume),
                        dragging_to_end_value(y, from, original_end_value) as f32,
                    )),
                };

                return (event::Status::Captured, Some(message));
            }
        }

        let release_hitbox_hit = self.release_dragger.hitbox.contains(relative_position);

        if release_hitbox_hit {
            state
                .attack_dragger_status
                .set_to_normal_if_in_hover_state();
            state.decay_dragger_status.set_to_normal_if_in_hover_state();
        }

        match state.release_dragger_status {
            EnvelopeDraggerStatus::Normal => {
                if release_hitbox_hit {
                    state.release_dragger_status = EnvelopeDraggerStatus::Hover;

                    state
                        .attack_dragger_status
                        .set_to_normal_if_in_hover_state();
                    state.decay_dragger_status.set_to_normal_if_in_hover_state();

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Hover => {
                if !release_hitbox_hit {
                    state.release_dragger_status = EnvelopeDraggerStatus::Normal;

                    self.cache.clear();
                }
            }
            EnvelopeDraggerStatus::Dragging {
                from,
                original_duration,
                ..
            } => {
                let message = Message::ChangeEnvelopeParametersSetValue {
                    operator_index: self.operator_index,
                    parameter_1: (
                        Parameter::Operator(
                            self.operator_index,
                            OperatorParameter::ReleaseDuration,
                        ),
                        dragging_to_duration(self.viewport_factor, x, from, original_duration)
                            as f32,
                    ),
                    parameter_2: None,
                };

                return (event::Status::Captured, Some(message));
            }
        }

        if let Some(dragging_from) = state.dragging_background_from {
            let zoom_factor = (dragging_from.from_point.y - y) / 50.0;

            let new_viewport_factor = (dragging_from.viewport_factor * zoom_factor.exp2())
                .min(1.0)
                .max(MIN_VIEWPORT_FACTOR);

            let x_offset_change_zoom = -dragging_from.original_visible_position
                * (dragging_from.viewport_factor - new_viewport_factor);

            let x_offset_change_drag =
                (x - dragging_from.from_point.x) / WIDTH as f32 * new_viewport_factor;

            let new_x_offset = Self::process_x_offset(
                dragging_from.original_x_offset + x_offset_change_zoom + x_offset_change_drag,
                new_viewport_factor,
            );

            let message = Message::EnvelopeChangeViewport {
                operator_index: self.operator_index,
                viewport_factor: new_viewport_factor,
                x_offset: new_x_offset,
            };

            return (event::Status::Captured, Some(message));
        }

        if bounds.contains(Point::new(x, y)) {
            (event::Status::Captured, None)
        } else {
            (event::Status::Ignored, None)
        }
    }

    pub fn handle_button_released(
        &self,
        state: &mut EnvelopeCanvasState,
    ) -> (event::Status, Option<Message>) {
        if state.release_dragger_status.is_dragging() {
            state.release_dragger_status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::ReleaseDuration),
                    self.release_duration,
                ),
                parameter_2: None,
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else if state.decay_dragger_status.is_dragging() {
            state.decay_dragger_status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::DecayDuration),
                    self.decay_duration,
                ),
                parameter_2: Some((
                    Parameter::Operator(self.operator_index, OperatorParameter::SustainVolume),
                    self.sustain_volume,
                )),
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else if state.attack_dragger_status.is_dragging() {
            state.attack_dragger_status = EnvelopeDraggerStatus::Normal;

            let message = Message::ChangeEnvelopeParametersEnd {
                operator_index: self.operator_index as u8,
                parameter_1: (
                    Parameter::Operator(self.operator_index, OperatorParameter::AttackDuration),
                    self.attack_duration,
                ),
                parameter_2: None,
            };

            self.cache.clear();

            (event::Status::Captured, Some(message))
        } else {
            let mut event_status = event::Status::Ignored;
            let mut opt_message = None;

            if state.dragging_background_from.is_some() {
                state.dragging_background_from = None;

                event_status = event::Status::Captured;
            }

            // Increment double click data release count if set
            if let Some(data) = state.double_click_data.as_mut() {
                data.releases += 1;

                event_status = event::Status::Captured;
            }

            // If this is second release without mouse movement in between,
            // send zoom to fit message
            if let Some(DoubleClickData { releases: 2, .. }) = state.double_click_data {
                state.double_click_data = None;

                self.cache.clear();

                event_status = event::Status::Captured;

                let (viewport_factor, x_offset) = self.get_zoom_to_fit_data();

                opt_message = Some(Message::EnvelopeChangeViewport {
                    operator_index: self.operator_index,
                    viewport_factor,
                    x_offset,
                });
            }

            (event_status, opt_message)
        }
    }
}

// Almost-correct reverse transformation for envelope dragger to duration
fn dragging_to_duration(
    viewport_factor: f32,
    cursor_x: f32,
    from: Point,
    original_value: f32,
) -> f32 {
    let change = (cursor_x - from.x) / WIDTH as f32;
    let change = change / ENVELOPE_PATH_SCALE_X;
    let change = change * viewport_factor * TOTAL_DURATION;

    (original_value + change)
        .min(1.0)
        .max(ENVELOPE_MIN_DURATION as f32 / ENVELOPE_MAX_DURATION as f32)
}

fn dragging_to_end_value(cursor_y: f32, from: Point, original_value: f32) -> f32 {
    let change = -(cursor_y - from.y) / HEIGHT as f32;
    let change = change / ENVELOPE_PATH_SCALE_Y;

    (original_value + change).min(1.0).max(0.0)
}
