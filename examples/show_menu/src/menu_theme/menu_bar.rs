use iced::Color;
use iced_window::menu::{Appearance, StyleSheet};

use super::Theme;



/// The style of a menu bar and its menus
#[derive(Default)]
#[allow(missing_debug_implementations)]
pub enum MenuBarStyle {
    /// The default style.
    #[default]
    Default,
}

impl StyleSheet for Theme {
    type Style = MenuBarStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {

        match style {
            MenuBarStyle::Default => Appearance {
                background: Color::TRANSPARENT,
                border_width: 1.0,
                border_radius: [6.0; 4],
                border_color: Color::TRANSPARENT,
                background_expand: [6; 4],
                path: Color::TRANSPARENT,
            },
        }
    }
}