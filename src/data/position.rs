use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}
