use bevy::ecs::system::EntityCommands;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Text, Text2dBundle};
use bevy::sprite::Anchor;
use bevy::text::TextStyle;

use crate::graphics::Palette;
use crate::graphics::pixel::LogicPos;
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
                color: Palette::BLACK.into(),
            },
        }
    }
}

pub fn text<'a, 'b, 'c>(commands: &'a mut Commands<'b, 'c>, fonts: &Fonts, text: &str, style: TextStyles, anchor: Anchor, pos: (f32, f32, f32)) -> EntityCommands<'a> {
    let mut e_c = commands
        .spawn(Text2dBundle {
            text: Text::from_section(text, style.style(fonts)),
            text_anchor: anchor,
            ..Default::default()
        });
    e_c.insert(LogicPos(Vec3::new(pos.0, pos.1, pos.2)));
    return e_c
}