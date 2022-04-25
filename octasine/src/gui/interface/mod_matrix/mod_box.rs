use std::marker::PhantomData;

use iced_baseview::canvas::{event, Frame, Path, Stroke};
use iced_baseview::{mouse, Point, Rectangle, Size};

use crate::common::ModTarget;
use crate::parameters::values::ParameterValue;

use crate::gui::interface::{Message, SnapPoint};

use super::common::*;
use super::StyleSheet;

pub enum ModulationBoxChange {
    Update(Message),
    ClearCache(Option<Message>),
    None,
}

pub trait ModulationBoxUpdate {
    fn update(&mut self, bounds: Rectangle, event: event::Event) -> ModulationBoxChange;
}

pub struct ModulationBox<P, V> {
    path: Path,
    pub center: Point,
    rect: Rectangle,
    hover: bool,
    click_started: bool,
    parameter_index: usize,
    target_index: usize,
    pub v: V,
    _phantom_data: PhantomData<P>,
}

impl<P, V> ModulationBox<P, V>
where
    P: ParameterValue<Value = V>,
    V: ModTarget,
{
    pub fn new(
        bounds: Size,
        from: usize,
        to: usize,
        parameter_index: usize,
        target_index: usize,
        v: V,
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

        top_left.x += 1.0;

        top_left = top_left.snap();

        let rect = Rectangle::new(top_left, size);
        let center = rect.center();

        let path = Path::circle(center, size.width / 2.0);

        Self {
            path,
            center,
            rect,
            hover: false,
            click_started: false,
            parameter_index,
            target_index,
            v,
            _phantom_data: Default::default(),
        }
    }

    fn active(&self) -> bool {
        self.v.index_active(self.target_index)
    }

    pub fn draw(&self, frame: &mut Frame, style_sheet: Box<dyn StyleSheet>) {
        let style = style_sheet.active();

        let stroke = Stroke::default()
            .with_color(style.box_border_color)
            .with_width(1.0);

        if self.active() || self.hover {
            frame.fill(&self.path, style.modulation_box_color_active);
        } else {
            frame.fill(&self.path, style.modulation_box_color_inactive);
        }

        frame.stroke(&self.path, stroke);
    }
}

impl<P, V> ModulationBoxUpdate for ModulationBox<P, V>
where
    P: ParameterValue<Value = V>,
    V: ModTarget,
{
    fn update(&mut self, bounds: Rectangle, event: event::Event) -> ModulationBoxChange {
        match event {
            event::Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, y },
            }) => {
                let cursor = Point::new(x - bounds.x, y - bounds.y);

                match (self.hover, self.rect.contains(cursor)) {
                    (false, true) => {
                        self.hover = true;

                        return ModulationBoxChange::ClearCache(None);
                    }
                    (true, false) => {
                        self.hover = false;

                        return ModulationBoxChange::ClearCache(None);
                    }
                    _ => (),
                }
            }
            event::Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if self.hover {
                    self.click_started = true;
                }
            }
            event::Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                if self.hover && self.click_started {
                    self.click_started = false;

                    self.v.set_index(self.target_index, !self.active());
                    let sync_value = P::from_processing(self.v).to_sync();

                    return ModulationBoxChange::Update(Message::ChangeSingleParameterImmediate(
                        self.parameter_index,
                        sync_value,
                    ));
                }
            }
            _ => (),
        }

        ModulationBoxChange::None
    }
}
