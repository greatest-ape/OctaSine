use iced_baseview::canvas::{event, Frame, Path, Stroke};
use iced_baseview::{mouse, Point, Rectangle, Size};

use crate::parameters::operator_mod_target::ModTargetStorage;
use crate::parameters::{Parameter, ParameterValue};

use crate::gui::interface::{Message, SnapPoint};

use super::common::*;
use super::StyleSheet;

pub struct ModulationBoxCanvasState {
    hover: bool,
    click_started: bool,
}

impl Default for ModulationBoxCanvasState {
    fn default() -> Self {
        Self {
            hover: false,
            click_started: false,
        }
    }
}

pub enum ModulationBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

pub trait ModulationBoxUpdate {
    fn update(
        &self,
        state: &mut ModulationBoxCanvasState,
        bounds: Rectangle,
        event: event::Event,
    ) -> ModulationBoxChange;
}

pub struct ModulationBox<P: ParameterValue> {
    path: Path,
    pub center: Point,
    rect: Rectangle,
    parameter: Parameter,
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
        parameter: Parameter,
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

    fn active(&self) -> bool {
        self.v.index_active(self.target_index)
    }

    pub fn draw(
        &self,
        state: &ModulationBoxCanvasState,
        frame: &mut Frame,
        style_sheet: Box<dyn StyleSheet>,
    ) {
        let style = style_sheet.active();

        let stroke = Stroke::default()
            .with_color(style.box_border_color)
            .with_width(1.0);

        let fill_color = match (self.active(), state.hover) {
            (true, false) => style.modulation_box_color_active,
            (true, true) => style.modulation_box_color_hover,
            (false, false) => style.modulation_box_color_inactive,
            (false, true) => style.modulation_box_color_hover,
        };

        frame.fill(&self.path, fill_color);
        frame.stroke(&self.path, stroke);
    }
}

impl<P> ModulationBoxUpdate for ModulationBox<P>
where
    P: ParameterValue<Value = ModTargetStorage>,
{
    fn update(
        &self,
        state: &mut ModulationBoxCanvasState,
        bounds: Rectangle,
        event: event::Event,
    ) -> ModulationBoxChange {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                match (state.hover, self.rect.contains(cursor)) {
                    (false, true) => {
                        state.hover = true;

                        return ModulationBoxChange::ClearCache(None);
                    }
                    (true, false) => {
                        state.hover = false;

                        return ModulationBoxChange::ClearCache(None);
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

                    return ModulationBoxChange::Update(Message::ChangeSingleParameterImmediate(
                        self.parameter,
                        sync_value,
                    ));
                }
            }
            _ => (),
        }

        ModulationBoxChange::None
    }
}
