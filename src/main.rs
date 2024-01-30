use std::{cmp::Ordering, collections::HashMap, ops::Range};

use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use rand::prelude::*;

mod colors;
mod ui;

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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
enum RunState {
    #[default] // default attribute macro to specify default state
    Playing,
    GameOver,
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Default, Resource)]
struct Game {
    score: u32,
    best_score: u32,
}

struct NewTileEvent;

#[derive(Component, Debug, PartialEq)]
struct Points {
    value: u32,
}

#[derive(Component, Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText; // unit struct (no fields), used as a tag so we can find some entity later

// A font is required to display any text, so we define that here
#[derive(Resource)]
struct FontSpec {
    // could be called anything
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        FontSpec {
            family: asset_server.load("fonts/FiraCode-Bold.ttf"),
        }
    }
}

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        // the offset is the starting point for drawing a tile
        // we use the iterator values + the offset to draw each tile
        let offset = (-self.physical_size / 2.0) // move to the far left
            + (TILE_SIZE / 2.0); // move 1/2 of a tile to the right

        offset
        + (f32::from(pos) * TILE_SIZE) // add x/y coord offset
        + (f32::from(pos + 1) * TILE_SPACER) // add spacer offset
    }
    fn size(&self) -> Vec2 {
        Vec2::new(self.physical_size, self.physical_size)
    }
}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(board.size()),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: colors::TILE_PLACEHOLDER,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        })
        .insert(board);
}

fn spawn_tiles(
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

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>, // query for all text components labeled with TileText
    tiles: Query<(&Points, &Children)>, // query for entities with Points & Children (will give us all tiles)
) {
    for (points, children) in tiles.iter() {
        // the first child is going to be the text component
        if let Some(entity) = children.first() {
            // We can use the texts query like a database, passing in the id to get a mutable entity.
            let mut text = texts.get_mut(*entity).expect("expected Text to exist");
            let text_section = text.sections.first_mut().expect("expected first section");
            text_section.value = points.value.to_string();
        }
    }
}

enum BoardShift {
    Up,
    Down,
    Left,
    Right,
}

impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
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
    fn set_column_position(&self, board_size: u8, position: &mut Mut<Position>, index: u8) {
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
    fn get_row_position(&self, position: &Position) -> u8 {
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

fn board_shift(
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

fn render_tiles(
    mut commands: Commands,
    // query accepts two type arguments
    // 1. A tuple of components that we want to query for (the data we get in our query)
    // 2. The set of filters to apply to the query
    mut tiles: Query<
        (Entity, &mut Transform, &Position),
        Changed<Position>, // gives a boolean values telling us whether pos changed or not
    >,
    query_board: Query<&Board>,
) {
    let board = query_board.single();
    for (entity, transform, pos) in tiles.iter_mut() {
        let x = board.cell_position_to_physical(pos.x);
        let y = board.cell_position_to_physical(pos.y);
        commands.entity(entity).insert(transform.ease_to(
            // final pos
            Transform::from_xyz(x, y, transform.translation.z),
            // easing fn (provides interpolation)
            EaseFunction::QuadraticInOut,
            // easing type
            EasingType::Once {
                duration: std::time::Duration::from_millis(100),
            },
        ));
    }
}

fn new_tile_handler(
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

fn spawn_tile(commands: &mut Commands, board: &Board, font_spec: &Res<FontSpec>, pos: Position) {
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

fn end_game(
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

fn game_reset(
    mut commands: Commands,
    tiles: Query<Entity, With<Position>>,
    mut game: ResMut<Game>,
) {
    for entity in tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.score = 0;
}
