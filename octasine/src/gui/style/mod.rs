mod macros;

pub mod application;
pub mod boolean_button;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod colors;
pub mod container;
pub mod envelope;
pub mod knob2;
pub mod menu;
pub mod mod_matrix;
pub mod modal;
pub mod pick_list;
pub mod radio;
pub mod scrollable;
pub mod text;
pub mod text_input;
pub mod wave_display;
pub mod wave_picker;

use iced_baseview::Font;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
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
}
