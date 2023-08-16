use iced_baseview::core::{mouse, Point, Rectangle, Size};
use iced_baseview::widget::canvas::{event, Frame, Path, Stroke};

use crate::gui::style::Theme;
use crate::parameters::operator_mod_target::ModTargetStorage;
use crate::parameters::{ParameterValue, WrappedParameter};

use crate::gui::{Message, SnapPoint};

use super::common::*;
use super::StyleSheet;

#[derive(Default)]
pub struct ModulationBoxCanvasState {
    hover: bool,
    click_started: bool,
}

pub enum ModulationBoxCanvasUpdateResult {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

pub trait ModulationBoxCanvasUpdate {
    fn update(
        &self,
        state: &mut ModulationBoxCanvasState,
        bounds: Rectangle,
        event: event::Event,
    ) -> ModulationBoxCanvasUpdateResult;
}

pub struct ModulationBox<P: ParameterValue> {
    path: Path,
    center: Point,
    rect: Rectangle,
    parameter: WrappedParameter,
    target_index: usize,
    pub v: P::Value,
}

impl<P> ModulationBox<P>
where
    P: ParameterValue<Value = ModTargetStorage>,
{
    pub fn new(
        bounds: Size,
        from: usize,
        to: usize,
        parameter: WrappedParameter,
        target_index: usize,
        v: ModTargetStorage,
    ) -> Self {
        let (x, y) = match (from, to) {
            (3, 2) => (2, 0),
            (3, 1) => (4, 0),
            (3, 0) => (6, 0),
            (2, 1) => (4, 2),
            (2, 0) => (6, 2),
            (1, 0) => (6, 4),
            _ => unreachable!(),
        };

        let (top_left, size) = get_box_base_point_and_size(bounds, x, y);

        let mut top_left = scale_point(bounds, top_left);
        let size = scale_size(size);

        top_left.x -= 1.0;

        top_left = top_left.snap();

        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let path = Path::circle(center, size.width / 2.0);

        Self {
            path,
            center,
            rect,
            parameter,
            target_index,
            v,
        }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    fn active(&self) -> bool {
        self.v.index_active(self.target_index)
    }

    pub fn draw(&self, state: &ModulationBoxCanvasState, frame: &mut Frame, theme: &Theme) {
        let apparence = theme.appearance();

        let stroke = Stroke::default()
            .with_color(apparence.box_border_color)
            .with_width(1.0);

        let fill_color = match (self.active(), state.hover) {
            (true, false) => apparence.modulation_box_color_active,
            (true, true) => apparence.modulation_box_color_hover,
            (false, false) => apparence.modulation_box_color_inactive,
            (false, true) => apparence.modulation_box_color_hover,
        };

        frame.fill(&self.path, fill_color);
        frame.stroke(&self.path, stroke);
    }
}

impl<P> ModulationBoxCanvasUpdate for ModulationBox<P>
where
    P: ParameterValue<Value = ModTargetStorage>,
{
    fn update(
        &self,
        state: &mut ModulationBoxCanvasState,
        bounds: Rectangle,
        event: event::Event,
    ) -> ModulationBoxCanvasUpdateResult {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                match (state.hover, self.rect.contains(cursor)) {
                    (false, true) => {
                        state.hover = true;

                        return ModulationBoxCanvasUpdateResult::ClearCache(None);
                    }
                    (true, false) => {
                        state.hover = false;

                        return ModulationBoxCanvasUpdateResult::ClearCache(None);
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if state.hover {
                    state.click_started = true;
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                if state.hover && state.click_started {
                    state.click_started = false;

                    let sync_value = {
                        let mut v = self.v;

                        v.set_index(self.target_index, !self.active());

                        P::new_from_audio(v).to_patch()
                    };

                    return ModulationBoxCanvasUpdateResult::Update(
                        Message::ChangeSingleParameterImmediate(self.parameter, sync_value),
                    );
                }
            }
            _ => (),
        }

        ModulationBoxCanvasUpdateResult::None
    }
}
