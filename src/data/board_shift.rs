use std::cmp::Ordering;

use bevy::prelude::*;

use super::position::Position;

pub enum BoardShift {
    Up,
    Down,
    Left,
    Right,
}

impl BoardShift {
    pub fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShift::Up => match Ord::cmp(&b.x, &a.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
            BoardShift::Down => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
            BoardShift::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            BoardShift::Right => match Ord::cmp(&b.y, &a.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
        }
    }
    pub fn set_column_position(&self, board_size: u8, position: &mut Mut<Position>, index: u8) {
        match self {
            BoardShift::Up => {
                position.y = board_size - 1 - index;
            }
            BoardShift::Down => {
                position.y = index;
            }
            BoardShift::Left => {
                position.x = index;
            }
            BoardShift::Right => {
                position.x = board_size - 1 - index;
            }
        }
    }
    pub fn get_row_position(&self, position: &Position) -> u8 {
        match self {
            BoardShift::Up | BoardShift::Down => position.x,
            BoardShift::Left | BoardShift::Right => position.y,
        }
    }
}

// impl TryFrom trait for shared reference to a key code
impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;
    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Up | KeyCode::W => Ok(BoardShift::Up),
            KeyCode::Down | KeyCode::S => Ok(BoardShift::Down),
            KeyCode::Left | KeyCode::A => Ok(BoardShift::Left),
            KeyCode::Right | KeyCode::D => Ok(BoardShift::Right),
            _ => Err("not a valid board_shift key"),
        }
    }
}
