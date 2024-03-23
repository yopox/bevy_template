use bevy::app::App;
use bevy::prelude::*;

pub use palette::Palette;
pub use pixel::LogicPos;
pub use text::text;
pub use text::TextStyles;
pub use transition::ScreenTransition;

mod palette;
mod text;
mod transition;
mod pixel;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Palette::WHITE.into()))
            .insert_resource(ScreenTransition::default())
            .add_systems(Update, (transition::update, pixel::update_positions))
        ;
    }
}