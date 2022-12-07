mod macros;

pub mod application;
pub mod boolean_button;
pub mod button;
pub mod checkbox;
pub mod colors;
pub mod container;
pub mod envelope;
pub mod knob;
pub mod menu;
pub mod mod_matrix;
pub mod pick_list;
pub mod radio;
pub mod scrollable;
pub mod text;
pub mod wave_display;
pub mod wave_picker;

use iced_baseview::{Color, Font};
use serde::{Deserialize, Serialize};

const OPEN_SANS_REGULAR: Font = Font::External {
    name: "Open Sans Regular",
    bytes: super::OPEN_SANS_BYTES_REGULAR,
};
const OPEN_SANS_SEMI_BOLD: Font = Font::External {
    name: "Open Sans Semi Bold",
    bytes: super::OPEN_SANS_BYTES_SEMI_BOLD,
};
const OPEN_SANS_BOLD: Font = Font::External {
    name: "Open Sans Bold",
    bytes: super::OPEN_SANS_BYTES_BOLD,
};
const OPEN_SANS_EXTRA_BOLD: Font = Font::External {
    name: "Open Sans Extra Bold",
    bytes: super::OPEN_SANS_BYTES_EXTRA_BOLD,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];

    pub fn background_color(&self) -> Color {
        match self {
            Theme::Dark => colors::dark::BACKGROUND,
            Theme::Light => Color::WHITE,
        }
    }

    pub fn font_regular(&self) -> Font {
        match self {
            Theme::Dark => OPEN_SANS_REGULAR,
            Theme::Light => OPEN_SANS_SEMI_BOLD,
        }
    }
    pub fn font_bold(&self) -> Font {
        match self {
            Theme::Dark => OPEN_SANS_SEMI_BOLD,
            Theme::Light => OPEN_SANS_BOLD,
        }
    }
    pub fn font_extra_bold(&self) -> Font {
        match self {
            Theme::Dark => OPEN_SANS_BOLD,
            Theme::Light => OPEN_SANS_EXTRA_BOLD,
        }
    }
    pub fn font_heading(&self) -> Font {
        match self {
            Theme::Dark => OPEN_SANS_BOLD,
            Theme::Light => OPEN_SANS_BOLD,
        }
    }

    pub fn tooltip_padding(&self) -> u16 {
        3
    }

    pub fn button_padding(&self) -> u16 {
        3
    }

    pub fn picklist_padding(&self) -> u16 {
        3
    }

    pub fn checkbox(&self) -> () {
        ()
    }
    pub fn radio(&self) -> () {
        ()
    }
    pub fn button(&self) -> button::ButtonStyle {
        button::ButtonStyle::Regular
    }
    pub fn value_button(&self) -> button::ButtonStyle {
        button::ButtonStyle::Value
    }
    pub fn pick_list(&self) -> () {
        ()
    }
    pub fn tooltip(&self) -> container::ContainerStyle {
        container::ContainerStyle::Tooltip
    }

    pub fn knob_regular(&self) -> knob::KnobStyle {
        knob::KnobStyle::Regular
    }
    pub fn knob_bipolar(&self) -> knob::KnobStyle {
        knob::KnobStyle::Bipolar
    }

    pub fn mute_button(&self) -> boolean_button::BooleanButtonStyle {
        boolean_button::BooleanButtonStyle::Mute
    }
    pub fn bpm_sync_button(&self) -> boolean_button::BooleanButtonStyle {
        boolean_button::BooleanButtonStyle::Regular
    }
    pub fn envelope_group_button(&self) -> boolean_button::BooleanButtonStyle {
        boolean_button::BooleanButtonStyle::Regular
    }
}
