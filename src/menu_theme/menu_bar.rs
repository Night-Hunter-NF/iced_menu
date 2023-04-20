use iced_graphics::Color;

use crate::menu::{Appearance, StyleSheet};

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
                background: Color::BLACK,
                border_width: 1.0,
                border_radius: [0.0; 4],
                border_color: Color::TRANSPARENT,
                background_expand: [6; 4],
                path: Color::TRANSPARENT,
            },
        }
    }
}
