use bevy::prelude::*;

use crate::data::{game::Game, position::Position};

pub fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.score = 0;
}
