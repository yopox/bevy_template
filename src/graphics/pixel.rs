use bevy::prelude::{Component, Query, Transform, Vec3};

#[derive(Component)]
pub struct LogicPos(pub Vec3);

pub fn update_positions(
    mut entities: Query<(&mut Transform, &LogicPos)>,
) {
    for (mut pos, logic_pos) in entities.iter_mut() {
        pos.translation.x = logic_pos.0.x.round();
        pos.translation.y = logic_pos.0.y.round();
        pos.translation.z = logic_pos.0.z;
    }
}