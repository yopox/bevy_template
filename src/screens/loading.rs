use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;
use crate::music::{BGM, PlayBGMEvent};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, Textures>(GameState::Loading)
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Sounds>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), exit)
        ;
    }
}

fn exit(mut play_bgm: EventWriter<PlayBGMEvent>) { play_bgm.send(PlayBGMEvent(BGM::Title)); }

#[derive(AssetCollection, Resource)]
pub struct Textures {
    // #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 18, rows = 1, padding_x = 0., padding_y = 0.))]
    // #[asset(path = "atlas.png")]
    // pub atlas: Handle<TextureAtlas>,

    // #[asset(path = "image.png")]
    // pub image: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "fonts/Absolute 10 Basic.ttf")]
    pub absolute: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct Sounds {
    // #[asset(path = "audio.ogg")]
    // pub audio: Handle<AudioSource>,
}