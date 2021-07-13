mod dark;
mod light;

use iced_baseview::{button, container, pick_list, radio, Color};
use serde::{Deserialize, Serialize};

use super::{divider, envelope, mod_matrix};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];

    pub fn background_color(&self) -> Color {
        match self {
            Theme::Dark => dark::BACKGROUND,
            Theme::Light => Color::WHITE,
        }
    }
    pub fn text_color(&self) -> Color {
        match self {
            Theme::Dark => dark::GRAY,
            Theme::Light => Color::BLACK,
        }
    }
    pub fn heading_color(&self) -> Color {
        match self {
            Theme::Dark => dark::LIGHT_GRAY,
            Theme::Light => Color::BLACK,
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn pick_list::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::PickList.into(),
            Theme::Dark => dark::PickList.into(),
        }
    }
}

impl From<Theme> for Box<dyn iced_audio::knob::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::knob::Knob.into(),
        }
    }
}

impl From<Theme> for Box<dyn envelope::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Box::new(light::Envelope) as Box<dyn envelope::StyleSheet>,
            Theme::Dark => Box::new(dark::Envelope) as Box<dyn envelope::StyleSheet>,
        }
    }
}

impl From<Theme> for Box<dyn mod_matrix::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Box::new(light::ModulationMatrix) as Box<dyn mod_matrix::StyleSheet>,
            Theme::Dark => Box::new(dark::ModulationMatrix) as Box<dyn mod_matrix::StyleSheet>,
        }
    }
}

impl From<Theme> for Box<dyn divider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Box::new(light::Divider) as Box<dyn divider::StyleSheet>,
            Theme::Dark => Box::new(dark::Divider) as Box<dyn divider::StyleSheet>,
        }
    }
}
