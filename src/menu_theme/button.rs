use iced_graphics::{Color, Vector, Background};
use iced_native::widget::button;
use crate::color;

use super::Theme;


#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Button {
    #[default]
    Normal,
    Transparent,
    Close,
    OtherMenu,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Button) -> button::Appearance {
        let auto_fill = |background: Color, text: Color| button::Appearance {
            background: background.into(),
            text_color: text,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        match style {
            Button::Normal => auto_fill(self.light_blue, self.text),
            Button::Transparent => auto_fill(Color::TRANSPARENT, self.text),
            Button::Close => auto_fill(Color::TRANSPARENT, self.text),
            Button::OtherMenu => auto_fill(Color::TRANSPARENT, self.text),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        let difference = if &Button::Transparent == style {
            Vector::new(0.0, 0.0)
        } else {
            Vector::new(0.0, 1.0)
        };

        match style {
            Button::Close => button::Appearance {
                background: color!(163, 55, 55).into(),
                ..active
            },
            Button::OtherMenu => button::Appearance {
                background: color!(78, 77, 83).into(),
                ..active
            },
            _ => button::Appearance {
                shadow_offset: active.shadow_offset + difference,
                ..active
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            Button::Close => button::Appearance {
                background: color!(241, 111, 122).into(),
                ..active
            },
            Button::OtherMenu => button::Appearance {
                background: color!(101, 100, 105).into(),
                ..active
            },
            _ => button::Appearance {
                shadow_offset: Vector::default(),
                ..active
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) =>Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
