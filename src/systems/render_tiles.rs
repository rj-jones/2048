use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};

use crate::data::{board::Board, position::Position};

pub fn render_tiles(
    mut commands: Commands,
    // query accepts two type arguments
    // 1. A tuple of components that we want to query for (the data we get in our query)
    // 2. The set of filters to apply to the query
    mut tiles: Query<
        (Entity, &mut Transform, &Position),
        Changed<Position>, // gives a boolean values telling us whether pos changed or not
    >,
    query_board: Query<&Board>,
) {
    let board = query_board.single();
    for (entity, transform, pos) in tiles.iter_mut() {
        let x = board.cell_position_to_physical(pos.x);
        let y = board.cell_position_to_physical(pos.y);
        commands.entity(entity).insert(transform.ease_to(
            // final pos
            Transform::from_xyz(x, y, transform.translation.z),
            // easing fn (provides interpolation)
            EaseFunction::QuadraticInOut,
            // easing type
            EasingType::Once {
                duration: std::time::Duration::from_millis(100),
            },
        ));
    }
}
