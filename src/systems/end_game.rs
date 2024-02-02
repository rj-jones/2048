use std::{collections::HashMap, ops::Range};

use bevy::prelude::*;

use crate::data::{board::Board, points::Points, position::Position, run_state::RunState};

pub fn end_game(
    tiles: Query<(&Position, &Points)>,
    query_board: Query<&Board>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    let board = query_board.single();

    if tiles.iter().len() == 16 {
        let map: HashMap<&Position, &Points> = tiles.iter().collect();

        let neighbor_points = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let board_range: Range<i8> = 0..(board.size as i8);

        let has_move = tiles.iter().any(|(Position { x, y }, value)| {
            neighbor_points
                .iter()
                .filter_map(|(x2, y2)| {
                    let new_x = *x as i8 - x2;
                    let new_y = *y as i8 - y2;

                    if !board_range.contains(&new_x) || !board_range.contains(&new_y) {
                        return None;
                    }

                    map.get(&Position {
                        x: new_x.try_into().unwrap(),
                        y: new_y.try_into().unwrap(),
                    })
                })
                .any(|&v| v == value)
        });

        if !has_move {
            dbg!("game over!");
            run_state.set(RunState::GameOver);
        }
    }
}
