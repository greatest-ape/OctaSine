use iced_audio::{knob::*, style::tick_marks};
use iced_baseview::Color;

use crate::gui::interface::style::Theme;

use super::colors::*;

const COLOR_TICK_MARKS_1: Color = GRAY_600;
const COLOR_TICK_MARKS_2: Color = GRAY_300;
const COLOR_EMPTY: Color = GRAY_600;
const COLOR_NOTCH: Color = TEXT_FG;

const NOTCH_STYLE: LineNotch = LineNotch {
    color: COLOR_NOTCH,
    width: StyleLength::Units(2.0),
    length: StyleLength::Units(6.0),
    cap: LineCap::Round,
    offset: StyleLength::Units(3.0),
};

const ARC_STYLE: ArcStyle = ArcStyle {
    width: StyleLength::Units(2.0),
    empty_color: COLOR_EMPTY,
    filled_color: BLUE,
    cap: LineCap::Square,
    notch: NotchShape::Line(NOTCH_STYLE),
};

const TICK_MARK_STYLE: tick_marks::Style = tick_marks::Style {
    tier_1: tick_marks::Shape::Circle {
        diameter: 3.0,
        color: COLOR_TICK_MARKS_1,
    },
    tier_2: tick_marks::Shape::Circle {
        diameter: 3.0,
        color: COLOR_TICK_MARKS_2,
    },
    tier_3: tick_marks::Shape::Line {
        length: 3.0,
        width: 2.0,
        color: COLOR_TICK_MARKS_2,
    },
};

pub struct KnobRegular;

impl iced_audio::knob::StyleSheet for KnobRegular {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        Appearance::Arc(ARC_STYLE)
    }
    fn hovered(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        self.active(style)
    }
    fn dragging(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        self.active(style)
    }
    fn tick_marks_style(&self, style: &Self::Style) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: TICK_MARK_STYLE,
            offset: 3.0,
        })
    }
}

const ARC_BIPOLAR_STYLE: ArcBipolarStyle = ArcBipolarStyle {
    width: StyleLength::Units(2.0),
    empty_color: COLOR_EMPTY,
    left_filled_color: BLUE,
    right_filled_color: BLUE,
    cap: LineCap::Square,
    notch_center: NotchShape::Line(NOTCH_STYLE),
    notch_left_right: None,
};

pub struct KnobBipolar;

impl iced_audio::knob::StyleSheet for KnobBipolar {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        Appearance::ArcBipolar(ARC_BIPOLAR_STYLE)
    }
    fn hovered(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        self.active(style)
    }
    fn dragging(&self, style: &Self::Style) -> iced_audio::knob::Appearance {
        self.active(style)
    }
    fn tick_marks_style(&self, style: &Self::Style) -> Option<TickMarksStyle> {
        Some(TickMarksStyle {
            style: TICK_MARK_STYLE,
            offset: 3.0,
        })
    }
}
