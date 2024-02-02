use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Game {
    pub score: u32,
    pub best_score: u32,
}
