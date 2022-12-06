mod light {
    use super::super::light::colors::*;

    use iced_audio::{knob::*, style::tick_marks};
    use iced_baseview::Color;

    const COLOR_TICK_MARKS_1: Color = GRAY_600;
    const COLOR_TICK_MARKS_2: Color = GRAY_300;
    const COLOR_EMPTY: Color = GRAY_600;
    const COLOR_NOTCH: Color = TEXT;

    pub const NOTCH_STYLE: LineNotch = LineNotch {
        color: COLOR_NOTCH,
        width: StyleLength::Units(2.0),
        length: StyleLength::Units(6.0),
        cap: LineCap::Round,
        offset: StyleLength::Units(3.0),
    };

    pub const ARC_STYLE: ArcStyle = ArcStyle {
        width: StyleLength::Units(2.0),
        empty_color: COLOR_EMPTY,
        filled_color: BLUE,
        cap: LineCap::Square,
        notch: NotchShape::Line(NOTCH_STYLE),
    };

    pub const TICK_MARK_STYLE: tick_marks::Style = tick_marks::Style {
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

    pub const ARC_BIPOLAR_STYLE: ArcBipolarStyle = ArcBipolarStyle {
        width: StyleLength::Units(2.0),
        empty_color: COLOR_EMPTY,
        left_filled_color: BLUE,
        right_filled_color: BLUE,
        cap: LineCap::Square,
        notch_center: NotchShape::Line(NOTCH_STYLE),
        notch_left_right: None,
    };
}

mod dark {

    use iced_audio::{knob::*, style::tick_marks};
    use iced_baseview::Color;

    use super::super::dark::colors::*;

    const COLOR_TICK_MARKS_1: Color = GRAY_600;
    const COLOR_TICK_MARKS_2: Color = GRAY_800;
    const COLOR_EMPTY: Color = GRAY_500;
    const COLOR_NOTCH: Color = GRAY_900;

    pub const NOTCH_STYLE: LineNotch = LineNotch {
        color: COLOR_NOTCH,
        width: StyleLength::Units(2.0),
        length: StyleLength::Units(6.0),
        cap: LineCap::Round,
        offset: StyleLength::Units(3.0),
    };

    pub const ARC_STYLE: ArcStyle = ArcStyle {
        width: StyleLength::Units(2.0),
        empty_color: COLOR_EMPTY,
        filled_color: BLUE,
        cap: LineCap::Square,
        notch: NotchShape::Line(NOTCH_STYLE),
    };

    pub const TICK_MARK_STYLE: tick_marks::Style = tick_marks::Style {
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

    pub const ARC_BIPOLAR_STYLE: ArcBipolarStyle = ArcBipolarStyle {
        width: StyleLength::Units(2.0),
        empty_color: COLOR_EMPTY,
        left_filled_color: BLUE,
        right_filled_color: BLUE,
        cap: LineCap::Square,
        notch_center: NotchShape::Line(NOTCH_STYLE),
        notch_left_right: None,
    };
}

use iced_audio::style::knob::{Appearance, StyleSheet, TickMarksStyle};

use super::Theme;

#[derive(Default)]
pub enum KnobStyle {
    #[default]
    Regular,
    Bipolar,
}

impl StyleSheet for Theme {
    type Style = KnobStyle;

    fn active(&self, style: &Self::Style) -> Appearance {
        match (self, style) {
            (Self::Dark, Self::Style::Regular) => Appearance::Arc(dark::ARC_STYLE),
            (Self::Dark, Self::Style::Bipolar) => Appearance::ArcBipolar(dark::ARC_BIPOLAR_STYLE),
            (Self::Light, Self::Style::Regular) => Appearance::Arc(light::ARC_STYLE),
            (Self::Light, Self::Style::Bipolar) => Appearance::ArcBipolar(light::ARC_BIPOLAR_STYLE),
        }
    }
    fn hovered(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
    fn dragging(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
    fn tick_marks_style(&self, style: &Self::Style) -> Option<TickMarksStyle> {
        let style = match self {
            Self::Dark => dark::TICK_MARK_STYLE,
            Self::Light => light::TICK_MARK_STYLE,
        };

        Some(TickMarksStyle { style, offset: 3.0 })
    }
}
