mod common;
mod draw;
mod events;

use iced_baseview::widget::canvas::{event, Cache, Canvas, Cursor, Geometry, Program};
use iced_baseview::{widget::Container, Element, Length, Point, Rectangle, Size};

use crate::audio::voices::log10_table::Log10Table;
use crate::parameters::operator_envelope::{
    OperatorAttackDurationValue, OperatorDecayDurationValue, OperatorEnvelopeGroupValue,
    OperatorReleaseDurationValue, OperatorSustainVolumeValue,
};
use crate::parameters::{OperatorParameter, Parameter, ParameterValue};
use crate::sync::GuiSyncHandle;

use super::super::style::Theme;
use super::super::{Message, LINE_HEIGHT};

use common::*;
pub use common::{Appearance, StyleSheet};

#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeValues {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub viewport_factor: f32,
    pub x_offset: f32,
}

pub struct EnvelopeCanvas {
    log10table: Log10Table,
    cache: Cache,
    style: Theme,
    operator_index: u8,
    attack_duration: f32,
    decay_duration: f32,
    sustain_volume: f32,
    release_duration: f32,
    group: OperatorEnvelopeGroupValue,
    modified_by_automation: bool,
    size: Size,
    viewport_factor: f32,
    x_offset: f32,
    attack_stage_path: EnvelopeStagePath,
    decay_stage_path: EnvelopeStagePath,
    release_stage_path: EnvelopeStagePath,
    attack_dragger: EnvelopeDragger,
    decay_dragger: EnvelopeDragger,
    release_dragger: EnvelopeDragger,
}

impl EnvelopeCanvas {
    pub fn new<H: GuiSyncHandle>(sync_handle: &H, operator_index: usize, style: Theme) -> Self {
        let operator_index = operator_index as u8;

        let attack_duration =
            OperatorAttackDurationValue::new_from_patch(sync_handle.get_parameter(
                Parameter::Operator(operator_index, OperatorParameter::AttackDuration),
            ))
            .to_patch();
        let decay_duration = OperatorDecayDurationValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::DecayDuration),
        ))
        .to_patch();
        let release_duration =
            OperatorReleaseDurationValue::new_from_patch(sync_handle.get_parameter(
                Parameter::Operator(operator_index, OperatorParameter::ReleaseDuration),
            ))
            .to_patch();
        let sustain_volume = OperatorSustainVolumeValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::SustainVolume),
        ))
        .to_patch();
        let group = OperatorEnvelopeGroupValue::new_from_patch(sync_handle.get_parameter(
            Parameter::Operator(operator_index, OperatorParameter::EnvelopeLockGroup),
        ));

        let mut envelope = Self {
            log10table: Default::default(),
            cache: Cache::default(),
            style,
            operator_index,
            attack_duration,
            decay_duration,
            sustain_volume,
            release_duration,
            group,
            modified_by_automation: true,
            size: SIZE,
            viewport_factor: 1.0,
            x_offset: 0.0,
            attack_stage_path: Default::default(),
            decay_stage_path: Default::default(),
            release_stage_path: Default::default(),
            attack_dragger: Default::default(),
            decay_dragger: Default::default(),
            release_dragger: Default::default(),
        };

        let (viewport_factor, x_offset) = envelope.get_zoom_to_fit_data();

        envelope.set_viewport(viewport_factor, x_offset);

        envelope
    }

    pub fn view(&self) -> Element<Message, Theme> {
        Container::new(
            Canvas::new(self)
                .width(Length::Units(WIDTH))
                .height(Length::Units(HEIGHT)),
        )
        .height(Length::Units(LINE_HEIGHT * 6))
        .into()
    }
}

/// Public style / viewport / parameter value setters
impl EnvelopeCanvas {
    pub fn set_style(&mut self, style: Theme) {
        self.style = style;
        self.cache.clear();
    }

    pub fn set_viewport(&mut self, viewport_factor: f32, x_offset: f32) {
        self.viewport_factor = viewport_factor;
        self.x_offset = Self::process_x_offset(x_offset, viewport_factor);

        self.update_data();
    }

    pub fn set_attack_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorAttackDurationValue::new_from_patch(value).to_patch();

        if value != self.attack_duration {
            self.attack_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_decay_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorDecayDurationValue::new_from_patch(value).to_patch();

        if value != self.decay_duration {
            self.decay_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_sustain_volume(&mut self, value: f32, internal: bool) {
        let value = OperatorSustainVolumeValue::new_from_patch(value).to_patch();

        if value != self.sustain_volume {
            self.sustain_volume = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_release_duration(&mut self, value: f32, internal: bool) {
        let value = OperatorReleaseDurationValue::new_from_patch(value).to_patch();

        if value != self.release_duration {
            self.release_duration = value;
            self.modified_by_automation = !internal;

            self.update_data();
        }
    }

    pub fn set_group(&mut self, group: OperatorEnvelopeGroupValue, internal: bool) {
        if group != self.group {
            self.group = group;

            self.modified_by_automation = !internal;
        }
    }

    fn update_data(&mut self) {
        self.update_stage_paths();

        self.attack_dragger
            .set_center(self.attack_stage_path.end_point);
        self.decay_dragger
            .set_center(self.decay_stage_path.end_point);
        self.release_dragger
            .set_center(self.release_stage_path.end_point);

        self.cache.clear();
    }

    fn update_stage_paths(&mut self) {
        let total_duration = self.viewport_factor * TOTAL_DURATION;
        let x_offset = self.x_offset / self.viewport_factor;

        self.attack_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            0.0,
            0.0,
            self.attack_duration as f32,
            1.0,
        );

        self.decay_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration,
            1.0,
            self.decay_duration as f32,
            self.sustain_volume as f32,
        );

        self.release_stage_path = EnvelopeStagePath::new(
            &self.log10table,
            self.size,
            total_duration,
            x_offset,
            self.attack_duration + self.decay_duration,
            self.sustain_volume,
            self.release_duration as f32,
            0.0,
        );
    }

    fn process_x_offset(x_offset: f32, viewport_factor: f32) -> f32 {
        x_offset.min(0.0).max(-1.0 + viewport_factor)
    }
}

/// Public value getters
impl EnvelopeCanvas {
    pub fn get_envelope_values(&self) -> EnvelopeValues {
        EnvelopeValues {
            attack: self.attack_duration,
            decay: self.decay_duration,
            sustain: self.sustain_volume,
            release: self.release_duration,
            viewport_factor: self.viewport_factor,
            x_offset: self.x_offset,
        }
    }
    pub fn get_viewport_factor(&self) -> f32 {
        self.viewport_factor
    }
    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }
    pub fn get_modified_by_automation(&self) -> bool {
        self.modified_by_automation
    }
}

/// Viewport change helpers
impl EnvelopeCanvas {
    pub(super) fn get_zoom_in_data(&self) -> (f32, f32) {
        for factor in FIXED_VIEWPORT_FACTORS.iter().copied() {
            if factor < self.viewport_factor {
                let mut new_x_offset = self.x_offset;

                // Zoom towards center of viewport unless envelope is takes up
                // less than half of it (in which case, implicitly zoom towards
                // the left)
                if self.get_current_duration() / TOTAL_DURATION > self.viewport_factor * 0.5 {
                    new_x_offset -= (self.viewport_factor - factor) / 2.0;
                }

                let new_viewport_factor = factor;
                let new_x_offset = Self::process_x_offset(new_x_offset, new_viewport_factor);

                return (new_viewport_factor, new_x_offset);
            }
        }

        (self.viewport_factor, self.x_offset)
    }

    pub(super) fn get_zoom_out_data(&self) -> (f32, f32) {
        for factor in FIXED_VIEWPORT_FACTORS.iter().rev().copied() {
            if factor > self.viewport_factor {
                let new_x_offset = self.x_offset + (factor - self.viewport_factor) / 2.0;

                let new_viewport_factor = factor;
                let new_x_offset = Self::process_x_offset(new_x_offset, new_viewport_factor);

                return (new_viewport_factor, new_x_offset);
            }
        }

        (self.viewport_factor, self.x_offset)
    }

    pub(super) fn get_zoom_to_fit_data(&self) -> (f32, f32) {
        let duration_ratio = self.get_current_duration() / TOTAL_DURATION;

        let mut new_viewport_factor = 1.0;

        for factor in FIXED_VIEWPORT_FACTORS.iter().copied() {
            if duration_ratio > factor {
                break;
            }

            new_viewport_factor = factor;
        }

        let new_x_offset = Self::process_x_offset(0.0, new_viewport_factor);

        (new_viewport_factor, new_x_offset)
    }

    fn get_current_duration(&self) -> f32 {
        self.attack_duration + self.decay_duration + self.release_duration
    }
}

impl Program<Message, Theme> for EnvelopeCanvas {
    type State = EnvelopeCanvasState;

    fn draw(
        &self,
        state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(bounds.size(), |frame| {
            self.draw_time_markers(frame, self.style);
            self.draw_stage_paths(frame, self.style.envelope());

            self.attack_dragger
                .draw(frame, self.style.envelope(), &state.attack_dragger_status);
            self.decay_dragger
                .draw(frame, self.style.envelope(), &state.decay_dragger_status);
            self.release_dragger
                .draw(frame, self.style.envelope(), &state.release_dragger_status);

            self.draw_viewport_indicator(frame, self.style.envelope());
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
            event::Event::Mouse(iced_baseview::mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => self.handle_cursor_moved(state, bounds, x, y),
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonPressed(
                iced_baseview::mouse::Button::Left,
            )) => self.handle_button_pressed(state, bounds),
            event::Event::Mouse(iced_baseview::mouse::Event::ButtonReleased(
                iced_baseview::mouse::Button::Left,
            )) => self.handle_button_released(state),
            _ => (event::Status::Ignored, None),
        }
    }
}
