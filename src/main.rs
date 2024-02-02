mod colors;
mod data;
mod systems;
mod ui;

use crate::data::board::Board;
use crate::data::font_spec::FontSpec;
use crate::data::game::Game;
use crate::data::new_tile_event::NewTileEvent;
use crate::data::run_state::RunState;
use crate::systems::board_shift::board_shift;
use crate::systems::end_game::end_game;
use crate::systems::game_reset::game_reset;
use crate::systems::new_tile_handler::new_tile_handler;
use crate::systems::render_tile_points::render_tile_points;
use crate::systems::render_tiles::render_tiles;
use crate::systems::setup::setup;
use crate::systems::spawn_board::spawn_board;
use crate::systems::spawn_tiles::spawn_tiles;
use bevy::prelude::*;
use bevy_easings::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#1f2638").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<RunState>()
        .add_plugin(ui::GameUiPlugin)
        .add_plugin(EasingsPlugin)
        // We use turbofish syntax because sometimes functions can operate on many different types.
        // It is important to init the font after the default plugins because the default plugins
        // initialize an asset server responsible for loading the font file.
        //
        // init_resource trys a couple of different approaches for instantiating the type we are
        // asking for. One of those is calling the FromWorld impl for that type which gives acess
        // to the world.
        .init_resource::<FontSpec>()
        .init_resource::<Game>()
        .add_event::<NewTileEvent>()
        // The apply_system_buffers system is used so that spawn_tiles system can query
        // for a board entity produced from the spawn_board system. Normally these all run in
        // parallel, which is what you typically want.
        .add_startup_systems((setup, spawn_board, apply_system_buffers).chain())
        .add_systems(
            (
                render_tile_points,
                board_shift,
                render_tiles,
                new_tile_handler,
                end_game,
            )
                // Use in_set to run systems when RunState::Playing, so when the state reaches RunState::GameOver,
                // it ends the game by stopping those systems.
                .in_set(OnUpdate(RunState::Playing)),
        )
        .add_systems((game_reset, spawn_tiles).in_schedule(OnEnter(RunState::Playing)))
        .run();
}
