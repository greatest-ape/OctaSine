mod dark;
mod light;
mod macros;

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
            Theme::Dark => dark::colors::BACKGROUND,
            Theme::Light => Color::WHITE,
        }
    }
    pub fn text_color(&self) -> Color {
        match self {
            Theme::Dark => dark::colors::TEXT_FG,
            Theme::Light => Color::BLACK,
        }
    }
    pub fn heading_color(&self) -> Color {
        match self {
            Theme::Dark => dark::colors::TEXT_FG,
            Theme::Light => Color::BLACK,
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
    pub fn font_heading(&self) -> Font {
        match self {
            Theme::Dark => OPEN_SANS_BOLD,
            Theme::Light => OPEN_SANS_BOLD,
        }
    }

    pub fn container_l0(&self) -> Box<dyn iced_style::container::StyleSheet> {
        match self {
            Self::Light => Box::new(light::ContainerL0),
            Self::Dark => Box::new(dark::ContainerL0),
        }
    }
    pub fn container_l1(&self) -> Box<dyn iced_style::container::StyleSheet> {
        match self {
            Self::Light => Box::new(light::ContainerL1),
            Self::Dark => Box::new(dark::ContainerL1),
        }
    }
    pub fn container_l2(&self) -> Box<dyn iced_style::container::StyleSheet> {
        match self {
            Self::Light => Box::new(light::ContainerL2),
            Self::Dark => Box::new(dark::ContainerL2),
        }
    }
    pub fn container_l3(&self) -> Box<dyn iced_style::container::StyleSheet> {
        match self {
            Self::Light => Box::new(light::ContainerL3),
            Self::Dark => Box::new(dark::ContainerL3),
        }
    }

    pub fn checkbox(&self) -> Box<dyn iced_baseview::checkbox::StyleSheet> {
        match self {
            Self::Light => Box::new(light::Checkbox),
            Self::Dark => Box::new(dark::Checkbox),
        }
    }
    pub fn radio(&self) -> Box<dyn iced_style::radio::StyleSheet> {
        match self {
            Self::Light => Box::new(light::Radio),
            Self::Dark => Box::new(dark::Radio),
        }
    }
    pub fn button(&self) -> Box<dyn iced_style::button::StyleSheet> {
        match self {
            Self::Light => Box::new(light::Button),
            Self::Dark => Box::new(dark::Button),
        }
    }
    pub fn pick_list(&self) -> Box<dyn iced_style::pick_list::StyleSheet> {
        match self {
            Self::Light => Box::new(light::PickList),
            Self::Dark => Box::new(dark::PickList),
        }
    }
    pub fn tooltip(&self) -> Box<dyn iced_baseview::container::StyleSheet> {
        match self {
            Self::Light => Box::new(light::Tooltip),
            Self::Dark => Box::new(dark::Tooltip),
        }
    }

    pub fn knob_regular(&self) -> Box<dyn iced_audio::knob::StyleSheet> {
        match self {
            Self::Light => Box::new(light::knob::KnobRegular),
            Self::Dark => Box::new(dark::knob::KnobRegular),
        }
    }
    pub fn knob_bipolar(&self) -> Box<dyn iced_audio::knob::StyleSheet> {
        match self {
            Self::Light => Box::new(light::knob::KnobBilpolar),
            Self::Dark => Box::new(dark::knob::KnobBilpolar),
        }
    }

    pub fn envelope(&self) -> Box<dyn super::envelope::StyleSheet> {
        match self {
            Self::Light => Box::new(light::Envelope),
            Self::Dark => Box::new(dark::Envelope),
        }
    }
    pub fn mod_matrix(&self) -> Box<dyn super::mod_matrix::StyleSheet> {
        match self {
            Self::Light => Box::new(light::ModulationMatrix),
            Self::Dark => Box::new(dark::ModulationMatrix),
        }
    }
    pub fn wave_picker(&self) -> Box<dyn super::wave_picker::StyleSheet> {
        match self {
            Self::Light => Box::new(light::LfoShapePicker),
            Self::Dark => Box::new(dark::LfoShapePicker),
        }
    }
    pub fn mute_button(&self) -> Box<dyn super::mute_button::StyleSheet> {
        match self {
            Self::Light => Box::new(light::MuteButton),
            Self::Dark => Box::new(dark::MuteButton),
        }
    }
}
