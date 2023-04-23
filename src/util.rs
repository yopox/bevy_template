use bevy::prelude::Color;
use lazy_static::lazy_static;

pub mod size {
    /// Tile size from the tileset
    const TILE_SIZE: usize = 8;

    /// Screen size in tiles
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 18;

    /// Camera scale
    pub const SCALE: f32 = 5.;

    /// Returns world coordinates for a tile, for instance `2` -> `(2 * TILE_SIZE) as f32 `.
    pub const fn tile_to_f32(tile: usize) -> f32 { (tile * TILE_SIZE) as f32 }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Palette {
    Black = 0,
    White,
    Transparent,
}

lazy_static! {
    static ref COLOR_OF_PALETTE: [Color; 3] = [
        Color::hex("#000000").unwrap(),
        Color::hex("#FFFFFF").unwrap(),
        Color::hex("#00000000").unwrap(),
    ];
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLOR_OF_PALETTE[self as usize]
    }
}