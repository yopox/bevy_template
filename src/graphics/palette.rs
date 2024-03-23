use bevy::prelude::Color;
use bevy::utils::HashMap;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Palette {
    WHITE, BLACK,
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLORS[&self]
    }
}

lazy_static! {
    static ref COLORS: HashMap<Palette, Color> = HashMap::from([
        (Palette::WHITE, Color::hex("#B0AFA7").unwrap()),
        (Palette::BLACK, Color::hex("#313028").unwrap()),
    ]);
}