use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

use crate::{
    colors,
    data::{
        board::Board, font_spec::FontSpec, points::Points, position::Position, tile_text::TileText,
    },
};

use super::spawn_board::TILE_SIZE;

pub fn spawn_tiles(
    mut commands: Commands,     // to spawn the tile sprites
    query_board: Query<&Board>, // query for the board component to get the board size
    font_spec: Res<FontSpec>,   // access to the FontSpec resource
) {
    let board = query_board.single(); // single will panic if != 1

    let mut rng = rand::thread_rng(); // rng to arbitrarily choose two locations in the grid
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size) // generate all possible grid locations
        .choose_multiple(&mut rng, 2); // pick two random tiles

    // destructure the x & y values from the tuples of the starting tiles
    // the iter() gives us references to the elements in a  Rust vec
    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        spawn_tile(&mut commands, board, &font_spec, pos);
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    board: &Board,
    font_spec: &Res<FontSpec>,
    pos: Position,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::TILE,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                board.cell_position_to_physical(pos.x),
                board.cell_position_to_physical(pos.y),
                2.0,
            ),
            ..default()
        })
        .with_children(|child_builder| {
            child_builder
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        // text uses sections to define updatable areas of content
                        "2", // defaault value for tiles
                        TextStyle {
                            font: font_spec.family.clone(), // family is a handle, so we clone it (only cloning the id)
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                })
                .insert(TileText); // insert component so we can find it later
        })
        .insert(Points { value: 2 })
        .insert(pos);
}
