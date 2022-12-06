use iced_baseview::widget::text::{Appearance, StyleSheet};

use super::Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: Self::Style) -> Appearance {
        Appearance { color: None }
    }
	
}