mod dark;
mod light;
mod macros;

use std::rc::Rc;

use iced_baseview::{theme, Color, Font};
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

    pub fn container_l0(&self) -> theme::Container {
        match self {
            Self::Light => theme::Container::Custom(Box::new(light::ContainerL0)),
            Self::Dark => theme::Container::Custom(Box::new(dark::ContainerL0)),
        }
    }
    pub fn container_l1(&self) -> theme::Container {
        match self {
            Self::Light => theme::Container::Custom(Box::new(light::ContainerL1)),
            Self::Dark => theme::Container::Custom(Box::new(dark::ContainerL1)),
        }
    }
    pub fn container_l2(&self) -> theme::Container {
        match self {
            Self::Light => theme::Container::Custom(Box::new(light::ContainerL2)),
            Self::Dark => theme::Container::Custom(Box::new(dark::ContainerL2)),
        }
    }
    pub fn container_l3(&self) -> theme::Container {
        match self {
            Self::Light => theme::Container::Custom(Box::new(light::ContainerL3)),
            Self::Dark => theme::Container::Custom(Box::new(dark::ContainerL3)),
        }
    }

    pub fn checkbox(&self) -> theme::Checkbox {
        match self {
            Self::Light => theme::Checkbox::Custom(Box::new(light::Checkbox)),
            Self::Dark => theme::Checkbox::Custom(Box::new(dark::Checkbox)),
        }
    }
    pub fn radio(&self) -> theme::Radio {
        match self {
            Self::Light => theme::Radio::Custom(Box::new(light::Radio)),
            Self::Dark => theme::Radio::Custom(Box::new(dark::Radio)),
        }
    }
    pub fn button(&self) -> theme::Button {
        match self {
            Self::Light => theme::Button::Custom(Box::new(light::Button)),
            Self::Dark => theme::Button::Custom(Box::new(dark::Button)),
        }
    }
    pub fn value_button(&self) -> theme::Button {
        match self {
            Self::Light => theme::Button::Custom(Box::new(light::ValueButton)),
            Self::Dark => theme::Button::Custom(Box::new(dark::ValueButton)),
        }
    }
    pub fn pick_list(&self) -> theme::PickList {
        match self {
            Self::Light => theme::PickList::Custom(Rc::new(light::PickList), Rc::new(light::Menu)),
            Self::Dark => theme::PickList::Custom(Rc::new(dark::PickList), Rc::new(dark::Menu)),
        }
    }
    pub fn tooltip(&self) -> theme::Container {
        match self {
            Self::Light => theme::Container::Custom(Box::new(light::Tooltip)),
            Self::Dark => theme::Container::Custom(Box::new(dark::Tooltip)),
        }
    }

    pub fn knob_regular(&self) -> iced_audio::style::theme::Knob {
        match self {
            Self::Light => {
                iced_audio::style::theme::Knob::Custom(Box::new(light::knob::KnobRegular))
            }
            Self::Dark => iced_audio::style::theme::Knob::Custom(Box::new(dark::knob::KnobRegular)),
        }
    }
    pub fn knob_bipolar(&self) -> iced_audio::style::theme::Knob {
        match self {
            Self::Light => {
                iced_audio::style::theme::Knob::Custom(Box::new(light::knob::KnobBipolar))
            }
            Self::Dark => iced_audio::style::theme::Knob::Custom(Box::new(dark::knob::KnobBipolar)),
        }
    }

    pub fn envelope(&self) -> Box<dyn super::envelope::widget::StyleSheet> {
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
            Self::Light => Box::new(light::WavePicker),
            Self::Dark => Box::new(dark::WavePicker),
        }
    }
    pub fn wave_display(&self) -> Box<dyn super::wave_display::StyleSheet> {
        match self {
            Self::Light => Box::new(light::WaveDisplay),
            Self::Dark => Box::new(dark::WaveDisplay),
        }
    }
    pub fn mute_button(&self) -> Box<dyn super::boolean_button::StyleSheet> {
        match self {
            Self::Light => Box::new(light::MuteButton),
            Self::Dark => Box::new(dark::MuteButton),
        }
    }
    pub fn bpm_sync_button(&self) -> Box<dyn super::boolean_button::StyleSheet> {
        match self {
            Self::Light => Box::new(light::BooleanButton),
            Self::Dark => Box::new(dark::BooleanButton),
        }
    }
    pub fn envelope_group_button(&self) -> Box<dyn super::boolean_button::StyleSheet> {
        match self {
            Self::Light => Box::new(light::BooleanButton),
            Self::Dark => Box::new(dark::BooleanButton),
        }
    }
}
