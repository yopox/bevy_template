use bevy::app::{App, Plugin};
use bevy::prelude::{Component, Query, ResMut, Resource, Without};
use bevy_text_mode::TextModeTextureAtlasSprite;

use crate::util;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationTimer(util::misc::ANIMATION_INTERVAL))
            .add_system(animate);
    }
}

fn flip(sprite: &TextModeTextureAtlasSprite) -> TextModeTextureAtlasSprite {
    let mut new_sprite = sprite.clone();
    new_sprite.flip_x = !new_sprite.flip_x;
    return new_sprite;
}

fn rotate(sprite: &TextModeTextureAtlasSprite, rotation: u8) -> TextModeTextureAtlasSprite {
    let mut new_sprite = sprite.clone();
    new_sprite.rotation = (new_sprite.rotation + rotation) % 4;
    return new_sprite;
}

fn animation(sprite: &TextModeTextureAtlasSprite) -> Option<TextModeTextureAtlasSprite> {
    match sprite.index {
        268 => Some(flip(sprite)),
        367 => Some(flip(sprite)),
        397 => Some(rotate(sprite, 1)),
        619 => Some(flip(&rotate(sprite, 2))),
        _ => None,
    }
}

#[derive(Resource)]
struct AnimationTimer(usize);

#[derive(Component)]
pub struct NoAnimation;

fn animate(
    mut timer: ResMut<AnimationTimer>,
    mut tiles: Query<&mut TextModeTextureAtlasSprite, Without<NoAnimation>>,
) {
    timer.0 -= 1;
    if timer.0 <= 0 {
        timer.0 = util::misc::ANIMATION_INTERVAL;

        for mut sprite in tiles.iter_mut() {
            let Some(new_sprite) = animation(&sprite) else { continue };
            sprite.flip_x = new_sprite.flip_x;
            sprite.rotation = new_sprite.rotation;
        }
    }
}