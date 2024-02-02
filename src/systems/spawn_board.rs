use crate::{colors, Board};
use bevy::prelude::*;
use itertools::Itertools;

pub const TILE_SIZE: f32 = 40.0;
pub const TILE_SPACER: f32 = 10.0;

pub fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(board.size()),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}
