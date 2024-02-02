use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

use crate::data::{
    board::Board, font_spec::FontSpec, new_tile_event::NewTileEvent, position::Position,
};

use super::spawn_tiles::spawn_tile;

pub fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();

    // get all the events we haven't handled yet
    for _event in tile_reader.iter() {
        // insert new tile
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = (0..board.size)
            .cartesian_product(0..board.size)
            // filter out all tiles that already have a number
            .filter_map(|tile_pos| {
                let new_pos = Position {
                    x: tile_pos.0,
                    y: tile_pos.1,
                };
                match tiles.iter().find(|&&pos| pos == new_pos) {
                    Some(_) => None,
                    None => Some(new_pos),
                }
            })
            .choose(&mut rng); // pick a random tile from the filtered list

        if let Some(pos) = possible_position {
            spawn_tile(&mut commands, board, &font_spec, pos);
        }
    }
}
