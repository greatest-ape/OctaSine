mod dark;
mod light;

use iced_baseview::{
    button, container, pick_list, radio,
    rule::{self, FillMode},
    Color,
};
use serde::{Deserialize, Serialize};

use super::{envelope, mod_matrix};

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
            Theme::Dark => dark::TEXT_BG,
            Theme::Light => Color::BLACK,
        }
    }
    pub fn heading_color(&self) -> Color {
        match self {
            Theme::Dark => dark::TEXT_FG,
            Theme::Light => Color::BLACK,
        }
    }
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

pub struct Rule;

impl rule::StyleSheet for Rule {
    fn style(&self) -> rule::Style {
        let default: Box<dyn rule::StyleSheet> = Default::default();
        let color = default.style().color;

        rule::Style {
            color,
            width: 1,
            radius: 0.0,
            fill_mode: FillMode::Full,
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Container.into(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(_theme: Theme) -> Self {
        Rule.into()
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Radio.into(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Button.into(),
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
            Theme::Light => light::knob::Knob.into(),
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
