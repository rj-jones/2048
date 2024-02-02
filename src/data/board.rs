use bevy::prelude::*;

use crate::systems::spawn_board::{TILE_SIZE, TILE_SPACER};

#[derive(Component)]
pub struct Board {
    pub size: u8,
    pub physical_size: f32,
}

impl Board {
    pub fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    pub fn cell_position_to_physical(&self, pos: u8) -> f32 {
        // the offset is the starting point for drawing a tile
        // we use the iterator values + the offset to draw each tile
        let offset = (-self.physical_size / 2.0) // move to the far left
            + (TILE_SIZE / 2.0); // move 1/2 of a tile to the right

        offset
        + (f32::from(pos) * TILE_SIZE) // add x/y coord offset
        + (f32::from(pos + 1) * TILE_SPACER) // add spacer offset
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.physical_size, self.physical_size)
    }
}
