use iced_baseview::widget::pick_list;
use iced_baseview::{Background, Color};

pub struct OctaSineStyle;

impl pick_list::StyleSheet for OctaSineStyle {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            selected_background: Background::from(Color::from_rgb(0.4, 0.4, 0.4)),
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        Default::default()
    }
    fn hovered(&self) -> pick_list::Style {
        Default::default()
    }
}
