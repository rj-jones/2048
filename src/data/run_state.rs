use bevy::prelude::*;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum RunState {
    #[default] // default attribute macro to specify default state
    Playing,
    GameOver,
}
