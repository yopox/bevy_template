use bevy::prelude::Color;
use bevy::text::TextStyle;

use crate::screens::Fonts;

pub enum TextStyles {
    Basic,
}

impl TextStyles {
    pub fn style(&self, fonts: &Fonts) -> TextStyle {
        match self {
            TextStyles::Basic => TextStyle {
                font: fonts.absolute.clone(),
                font_size: 18.0,
                color: Color::WHITE,
            },
        }
    }
}