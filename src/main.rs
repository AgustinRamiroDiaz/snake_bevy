use std::collections::VecDeque;

use bevy::{prelude::*, window::WindowMode};
use movement::ProposeDirection;

mod coordinate;
use coordinate::Coordinate;

mod direction;
use direction::Direction;

mod main_menu;
use main_menu::MainMenu;
use main_menu::NumberOfPlayersSelected;

mod game_state;
use game_state::{AppState, GameStatePlugin};

mod ai;
use ai::AIPlugin;

mod asset_loader;
use asset_loader::AssetLoaderPlugin;

mod movement;
use movement::SnakeMovementPlugin;

mod score;
use score::ScorePlugin;

mod win;
use win::WinPlugin;

mod apple;
use apple::ApplePlugin;

mod collision;
use collision::CollisionPlugin;

mod blink;

mod schedule;
use schedule::InGameSet;
use schedule::SchedulePlugin;

use std::env;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // Borderless looks distorted when running in the web
                    #[cfg(not(target_arch = "wasm32"))]
                    mode: WindowMode::BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
        SnakePlugin,
        SnakeMovementPlugin,
        MainMenu {
            max_number_of_players: MAX_NUMBER_OF_PLAYERS,
        },
        GameStatePlugin,
        AssetLoaderPlugin,
        ScorePlugin,
        WinPlugin,
        ApplePlugin,
        CollisionPlugin,
        SchedulePlugin,
    ));

    if env::var("AI").unwrap_or("false".to_string()) == "true" {
        app.add_plugins(AIPlugin {
            player_numbers: vec![Id(1), Id(2), Id(3), Id(4)],
        });
    }

    app.run();
}

struct SnakePlugin;

const SIZE: f32 = 0.8;
const HALF_LEN: i32 = 7;
const BOARD_LEN: i32 = 2 * HALF_LEN;
const PADDING: f32 = 1.0;
const BOARD_VIEWPORT_IN_WORLD_UNITS: f32 = BOARD_LEN as f32 + 2.0 * PADDING;

const MAX_NUMBER_OF_PLAYERS: usize = 4;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    toroid_coordinates,
                    add_sprite_bundles,
                    // This is needed in order to render the sprites correctly, we need to flush the sprites into the world and then update their transforms
                    apply_deferred,
                    update_local_coordinates_to_world_transforms,
                )
                    .chain()
                    .in_set(InGameSet::Last)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(AppState::InGame),
                (despawn_snakes, spawn_snakes).chain(),
            );
    }
}

fn setup(mut commands: Commands) {
    let mut grid = vec![];

    for x in -HALF_LEN..=HALF_LEN {
        for y in -HALF_LEN..=HALF_LEN {
            grid.push((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Coordinate(Vec2::new(x as f32, y as f32)),
                Depth(-1.0),
            ));
        }
    }
    commands.spawn_batch(grid);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: BOARD_VIEWPORT_IN_WORLD_UNITS,
                min_height: BOARD_VIEWPORT_IN_WORLD_UNITS,
            },
            ..Default::default()
        },
        ..default()
    });
}

fn spawn_snakes(mut commands: Commands, number_of_players: Res<NumberOfPlayersSelected>) {
    let mut spawn_snake =
        |id, spawn_coord: Coordinate, direction: Direction, color: MyColor, name: String| {
            let head_a = commands
                .spawn((color, SnakeSegment, spawn_coord.clone()))
                .id();

            commands.spawn((
                Snake {
                    segments: VecDeque::from([head_a]),
                    player_number: id,
                    direction: direction.clone(),
                    trail: Coordinate(
                        spawn_coord.0 - <direction::Direction as Into<Vec2>>::into(direction),
                    ),
                    input_blocked: false,
                    inmortal_ticks: 0,
                    name,
                },
                color,
            ));
        };

    let snakes = [
        (
            Id(1),
            Coordinate::from((-3.0, -3.0)),
            Direction::Right,
            MyColor(Color::LIME_GREEN),
            "Ninja".to_string(),
        ),
        (
            Id(2),
            Coordinate::from((3.0, 3.0)),
            Direction::Left,
            MyColor(Color::PINK),
            "Panther".to_string(),
        ),
        (
            Id(3),
            Coordinate::from((-3.0, 3.0)),
            Direction::Down,
            MyColor(Color::SALMON),
            "Sushi".to_string(),
        ),
        (
            Id(4),
            Coordinate::from((3.0, -3.0)),
            Direction::Up,
            MyColor(Color::TURQUOISE),
            "Sonic".to_string(),
        ),
    ];

    snakes
        .into_iter()
        .take(number_of_players.0)
        .map(|(id, coord, direction, color, name)| spawn_snake(id, coord, direction, color, name))
        .count();
}

fn despawn_snakes(mut commands: Commands, snakes: Query<(Entity, &Snake)>) {
    snakes.iter().for_each(|(entity, snake)| {
        snake
            .segments
            .iter()
            .for_each(|&entity| commands.entity(entity).despawn_recursive());
        commands.entity(entity).despawn_recursive();
    });
}

#[derive(Component)]
pub(crate) struct Snake {
    name: String,
    segments: VecDeque<Entity>,
    direction: Direction,
    player_number: Id, // TODO: move into its own component
    trail: Coordinate,
    input_blocked: bool,
    inmortal_ticks: u8,
}

#[derive(Component, Debug, PartialEq, Clone)]
struct Id(u8);

#[derive(Component)]
struct SnakeSegment;

#[derive(Component, Clone, Copy)]
struct MyColor(Color);

fn update_local_coordinates_to_world_transforms(
    mut query: Query<
        (&Coordinate, &mut Transform, Option<&Depth>),
        Or<(Changed<Coordinate>, Changed<Transform>)>,
    >,
) {
    for (coordinate, mut transform, depth) in query.iter_mut() {
        transform.translation = coordinate.0.extend(depth.map_or(0.0, |x| x.0))
    }
}

#[derive(Component)]
struct Depth(f32);

// TODO: we assume that Transform == SpriteBundle
fn add_sprite_bundles(
    mut query: Query<(Entity, &MyColor), (Changed<Coordinate>, Without<Transform>)>,
    mut commands: Commands,
) {
    for (entity, color) in query.iter_mut() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                color: color.0,
                ..Default::default()
            },

            ..Default::default()
        });
    }
}

fn toroid_coordinates(
    mut query: Query<&mut Coordinate, (With<SnakeSegment>, Changed<Coordinate>)>,
) {
    for mut coordinate in query.iter_mut() {
        if coordinate.0.x.abs() > HALF_LEN as f32 {
            coordinate.0.x = -coordinate.0.x.signum() * HALF_LEN as f32;
        }
        if coordinate.0.y.abs() > HALF_LEN as f32 {
            coordinate.0.y = -coordinate.0.y.signum() * HALF_LEN as f32;
        }
    }
}
