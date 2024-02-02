use bevy::prelude::*;
use itertools::Itertools;

use crate::data::{
    board::Board, board_shift::BoardShift, game::Game, new_tile_event::NewTileEvent,
    points::Points, position::Position,
};

pub fn board_shift(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>, // allows us to send events to the queue
    mut game: ResMut<Game>,
) {
    let board = query_board.single();

    // using just_pressed is useful because it only match once per press
    let shift_direction = input
        .get_just_pressed()
        .find_map(|key_code| BoardShift::try_from(key_code).ok());

    if let Some(board_shift) = shift_direction {
        // the tiles query is mutable so we use iter_mut()
        // sort by row, then by column (sorted_by is from the itertools crate)
        // Ordering is an Enum in the std crate
        // Ord is a trait from the std crate, requires the cmp (compare method), which compares to values
        // of the given type. u8 already has an Ord impl
        let mut it = tiles
            .iter_mut()
            // determine if any two y values in a tiles positions component are equal (same row)
            // if they are not equal, pass the return value back to sorted_by because it will be an
            // ordering variant.
            .sorted_by(|a, b| board_shift.sort(&a.1, &b.1))
            .peekable(); // allows us to look at the next value in the iterator

        // use the underscore to tell the compiler to figure out the type
        // dbg!(it.collect::<Vec<_>>());

        let mut column: u8 = 0;
        while let Some(mut tile) = it.next() {
            board_shift.set_column_position(board.size, &mut tile.1, column);
            if let Some(tile_next) = it.peek() {
                if board_shift.get_row_position(&tile.1)
                    != board_shift.get_row_position(&tile_next.1)
                {
                    // different rows, don't merge
                    column = 0;
                } else if tile.2.value != tile_next.2.value {
                    // different values, don't merge
                    column = column + 1;
                } else {
                    // merge
                    let real_next_tile = it
                        .next()
                        .expect("A peeked tile should always exist when we ...");
                    tile.2.value = tile.2.value + real_next_tile.2.value;

                    game.score += tile.2.value;

                    commands.entity(real_next_tile.0).despawn_recursive();

                    if let Some(future) = it.peek() {
                        if board_shift.get_row_position(&tile.1)
                            != board_shift.get_row_position(&future.1)
                        {
                            column = 0;
                        } else {
                            column = column + 1;
                        }
                    }
                }
            }
        }
        tile_writer.send(NewTileEvent);
        if game.best_score < game.score {
            game.best_score = game.score;
        }
    }
}
