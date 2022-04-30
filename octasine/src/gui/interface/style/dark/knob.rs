
use crate::hex_gray;

use super::*;
use iced_audio::{knob::*, style::tick_marks};

const NOTCH_STYLE: LineNotch = LineNotch {
    color: hex_gray!(0xB0),
    width: StyleLength::Units(2.0),
    length: StyleLength::Units(6.0),
    cap: LineCap::Round,
    offset: StyleLength::Units(3.0),
};

const ARC_STYLE: ArcStyle = ArcStyle {
    width: StyleLength::Units(2.0),
    empty_color: hex_gray!(0x70),
    filled_color: BLUE, // hex_gray!(0xB0),
    cap: LineCap::Square,
    notch: NotchShape::Line(NOTCH_STYLE),
};

const TICK_MARK_STYLE: tick_marks::Style = tick_marks::Style {
    tier_1: tick_marks::Shape::Line {
        length: 3.0,
        width: 2.0,
        color: hex_gray!(0x70),
    },
    tier_2: tick_marks::Shape::Line {
        length: 3.0,
        width: 2.0,
        color: hex_gray!(0x70),
    },
    tier_3: tick_marks::Shape::Line {
        length: 3.0,
        width: 2.0,
        color: hex_gray!(0x70),
    },
};

pub struct KnobRegular;

impl iced_audio::knob::StyleSheet for KnobRegular {
    fn active(&self) -> iced_audio::knob::Style {
        Style::Arc(ARC_STYLE)
    }
    fn hovered(&self) -> iced_audio::knob::Style {
        self.active()
    }
    fn dragging(&self) -> iced_audio::knob::Style {
        self.active()
    }
    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: TICK_MARK_STYLE,
            offset: 3.0,
        })
    }
}

const ARC_BIPOLAR_STYLE: ArcBipolarStyle = ArcBipolarStyle {
    width: StyleLength::Units(2.0),
    empty_color: hex_gray!(0x70),
    left_filled_color: BLUE,  // hex_gray!(0xB0),
    right_filled_color: BLUE, // hex_gray!(0xB0),
    cap: LineCap::Square,
    notch_center: NotchShape::Line(NOTCH_STYLE),
    notch_left_right: None,
};

pub struct KnobBilpolar;

impl iced_audio::knob::StyleSheet for KnobBilpolar {
    fn active(&self) -> iced_audio::knob::Style {
        Style::ArcBipolar(ARC_BIPOLAR_STYLE)
    }
    fn hovered(&self) -> iced_audio::knob::Style {
        self.active()
    }
    fn dragging(&self) -> iced_audio::knob::Style {
        self.active()
    }
    fn tick_marks_style(&self) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: TICK_MARK_STYLE,
            offset: 3.0,
        })
    }
}
