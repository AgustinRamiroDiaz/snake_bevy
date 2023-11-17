use core::panic;
use std::collections::VecDeque;
use std::iter;

use bevy::{prelude::*, window::WindowMode};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;

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
        MainMenu {
            max_number_of_players: MAX_NUMBER_OF_PLAYERS,
        },
        GameStatePlugin,
        EguiPlugin,
    ));

    if env::var("AI").unwrap_or("false".to_string()) == "true" {
        app.add_plugins(AIPlugin {
            player_numbers: vec![Id(1), Id(2), Id(3), Id(4)],
        });
    }

    app.run();
}

fn how_to_play(mut contexts: EguiContexts) {
    egui::Window::new("How to play").show(contexts.ctx_mut(), |ui| {
        ui.label("`Esc` escape key to open the menu");
        ui.label("`Esc` escape key to get back into the game");
        ui.label("`Arrow keys` to move player 1");
        ui.label("`WASD` to move player 2");
        ui.label("`IJKL` to move player 3");
        ui.label("`Numpad 8456` to move player 4");
    });
}

struct SnakePlugin;

const SNAKE_TICK_SECONDS: f32 = 0.1;
const SIZE: f32 = 30.0;
const GAP: f32 = 4.0;
const HALF_LEN: i32 = 7;
const INMORTAL_TICKS: u8 = 10;
const PROPORTION_LOST_PER_HIT: f32 = 0.3;
const PADDING: f32 = 10.0;
const BOARD_VIEWPORT_IN_WORLD_UNITS: f32 =
    SIZE + 2.0 * (SIZE + GAP) * HALF_LEN as f32 + 2.0 * PADDING;

const MAX_NUMBER_OF_PLAYERS: usize = 4;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(
            SNAKE_TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_event::<ProposeDirection>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tick,
                input_snake_direction,
                toroid_coordinates,
                eat_apple,
                collision,
                update_score,
                handle_snake_direction,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            PreUpdate,
            (
                update_local_coordinates_to_world_transforms,
                add_sprite_bundles,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), despawn_snakes)
        .add_systems(OnEnter(AppState::InGame), spawn_snakes)
        .add_systems(Update, how_to_play);
    }
}

fn setup(
    mut commands: Commands,
    number_of_players: Res<NumberOfPlayersSelected>,
    asset_server: Res<AssetServer>,
) {
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
                MyColor(Color::DARK_GRAY),
                Coordinate(Vec2::new(x as f32, y as f32)),
                Depth(-1.0),
            ));
        }
    }
    commands.spawn_batch(grid);

    commands.spawn((
        TextBundle::from_sections((0..number_of_players.0).map(|_| TextSection::default()))
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        Score,
    ));
    spawn_apple(&mut commands, &asset_server);
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
    let mut spawn_snake = |id, spawn_coord: Coordinate, direction: Direction, color: MyColor| {
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
        ),
        (
            Id(2),
            Coordinate::from((3.0, 3.0)),
            Direction::Left,
            MyColor(Color::PINK),
        ),
        (
            Id(3),
            Coordinate::from((-3.0, 3.0)),
            Direction::Down,
            MyColor(Color::SALMON),
        ),
        (
            Id(4),
            Coordinate::from((3.0, -3.0)),
            Direction::Up,
            MyColor(Color::TURQUOISE),
        ),
    ];

    snakes
        .into_iter()
        .take(number_of_players.0)
        .map(|(id, coord, direction, color)| spawn_snake(id, coord, direction, color))
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
struct Score;

#[derive(Component)]
struct Snake {
    segments: VecDeque<Entity>,
    direction: Direction,
    player_number: Id,
    trail: Coordinate,
    input_blocked: bool,
    inmortal_ticks: u8,
}

#[derive(Component, Debug, PartialEq, Clone)]
struct Id(u8);

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Apple;

#[derive(Component, Clone, Copy)]
struct MyColor(Color);

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Snake>,
    mut entity_query: Query<&mut Coordinate>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut snake in query.iter_mut() {
            snake.input_blocked = false;
            snake.inmortal_ticks = snake.inmortal_ticks.saturating_sub(1);
            // TODO: don't unwrap
            let &tail_entity = snake.segments.back().unwrap();
            let &head_entity = snake.segments.front().unwrap();

            let head = entity_query.get_mut(head_entity).unwrap();

            let head_translation = head.0;

            if let Ok(mut tail) = entity_query.get_mut(tail_entity) {
                snake.trail = Coordinate(tail.0); // TODO: remove double conversion
                tail.0 = head_translation + Into::<Vec2>::into(snake.direction.clone());
                snake.segments.rotate_right(1);
            }
        }
    }
}

fn update_local_coordinates_to_world_transforms(
    mut query: Query<
        (&Coordinate, &mut Transform, Option<&Depth>),
        Or<(Changed<Coordinate>, Changed<Transform>)>,
    >,
) {
    for (coordinate, mut transform, depth) in query.iter_mut() {
        transform.translation = coordinate.0.extend(depth.map_or(0.0, |x| x.0)) * (SIZE + GAP)
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

            // TODO: find another way of hiding this
            // The problem is that the SpriteBundle is added with a Transform, and just in the next frame it's updated to its corresponding Coordinate. So we've got a frame where the sprite is in the wrong place (middle of the screen)
            // This is a quick patch to spawn the sprite in a place where it's not visible
            transform: Transform::from_translation(
                Vec3::ONE * BOARD_VIEWPORT_IN_WORLD_UNITS * 100.0,
            ),
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

fn collision(
    mut commands: Commands,
    mut snake_query: Query<(Entity, &mut Snake)>,
    query: Query<&Coordinate>,
    changed_coordinates: Query<Entity, Changed<Coordinate>>,
) {
    if changed_coordinates.iter().count() == 0 {
        // TODO: find a better way to do this
        return;
    }

    let mut bodies_coordinates = std::collections::HashSet::new();

    for (_, snake) in snake_query.iter() {
        snake
            .segments
            .iter()
            .skip(1)
            .flat_map(|&e| query.get(e))
            .for_each(|e| {
                bodies_coordinates.insert(e);
            });
    }

    let get_head = |snake: &Snake| {
        let &head = snake.segments.front()?;
        query.get(head).ok()
    };

    let snake_heads_coordinates = snake_query
        .iter()
        .flat_map(|(entity, snake)| Some((entity, get_head(snake)?)))
        .collect::<Vec<_>>();

    for (entity, mut snake) in snake_query
        .iter_mut()
        .filter(|(_, snake)| snake.inmortal_ticks == 0)
    {
        let head_coordinate = get_head(&snake).unwrap();

        if bodies_coordinates.contains(head_coordinate)
            || snake_heads_coordinates
                .iter()
                .filter(|(e, _)| *e != entity)
                .any(|(_, c)| *c == head_coordinate)
        {
            for _ in 0..(snake.segments.len() as f32 * PROPORTION_LOST_PER_HIT).ceil() as i8 {
                if snake.segments.len() == 1 {
                    break;
                }

                commands
                    .entity(snake.segments.pop_back().unwrap())
                    .despawn_recursive();
            }
            snake.inmortal_ticks = INMORTAL_TICKS;
        }
    }
}

// TODO: decouple this logic into smaller units
// TODO: decouple spawning from eating
// TODO: sometime the apple spawns inside a snake and it's not visible until the snake moves
fn eat_apple(
    mut commands: Commands,
    mut snakes: Query<(&mut Snake, &MyColor)>,
    coordinates: Query<&Coordinate>,
    apples: Query<(Entity, &Coordinate), With<Apple>>,
    asset_server: Res<AssetServer>,
) {
    let get_head = |snake: &Snake| {
        let &head = snake.segments.front()?;
        coordinates.get(head).ok()
    };

    for (mut snake, &color) in snakes.iter_mut() {
        for (apple, coord) in apples.iter() {
            if coord == get_head(&snake).unwrap() {
                commands.entity(apple).despawn();
                spawn_apple(&mut commands, &asset_server);

                let tail = commands
                    .spawn((color, SnakeSegment, snake.trail.clone()))
                    .id();

                snake.segments.push_back(tail);
                return;
            }
        }
    }
}

fn spawn_apple(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Apple,
        Depth(1.0),
        Coordinate(Vec2::new(
            rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
            rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
        )),
        SpriteBundle {
            texture: asset_server.load("pumpkin.png"),
            transform: Transform::from_translation(Vec3::ONE * 1000.0), // This is done in order to not show the apple until the next frame. TODO: find a more "elegant" way of doing this
            ..default()
        },
    ));
}

// Eventually we could use https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/multiplayer.rs for better input handling
fn input_snake_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Snake>,
    mut propose_direction: EventWriter<ProposeDirection>,
) {
    for snake in query.iter_mut().filter(|snake| !snake.input_blocked) {
        let direction = match snake.player_number.0 {
            1 => {
                if keyboard_input.pressed(KeyCode::Left) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::Right) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::Up) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::Down) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            2 => {
                if keyboard_input.pressed(KeyCode::A) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::D) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::W) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::S) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            3 => {
                if keyboard_input.pressed(KeyCode::J) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::L) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::I) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::K) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            4 => {
                if keyboard_input.pressed(KeyCode::Numpad4) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::Numpad6) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::Numpad8) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::Numpad5) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            other => panic!("Invalid player number {}, only 1-4 supported", other),
        };

        if let Some(direction) = direction {
            propose_direction.send(ProposeDirection {
                id: snake.player_number.clone(),
                direction,
            });
        }
    }
}

// TODO: can we remove the `clone`s?
fn handle_snake_direction(
    mut snakes: Query<&mut Snake>,
    mut proposed_direction: EventReader<ProposeDirection>,
) {
    for direction in proposed_direction.read() {
        for mut snake in snakes
            .iter_mut()
            .filter(|snake| snake.player_number == direction.id)
            .filter(|snake| !snake.input_blocked)
        {
            if snake.direction == !direction.direction.clone() {
                return;
            }
            if snake.direction == direction.direction.clone() {
                return;
            }
            snake.direction = direction.direction.clone();
            snake.input_blocked = true;
        }
    }
}

fn update_score(snakes: Query<(&Snake, &MyColor)>, mut text: Query<&mut Text, With<Score>>) {
    let mut snakes = Vec::from_iter(snakes.iter());
    snakes.sort_by_key(|(snake, _)| -1 * snake.segments.len() as i8);

    text.single_mut()
        .sections
        .resize(snakes.len(), TextSection::default());

    for (text_section, (snake, color)) in
        iter::zip(text.single_mut().sections.iter_mut(), snakes.iter())
    {
        text_section.value = format!(
            "Player {:?}: {}\n",
            snake.player_number.0,
            snake.segments.len()
        );
        text_section.style = TextStyle {
            font_size: 60.0,
            color: color.0,
            ..default()
        }
    }
}

/// This event proposes a direction for the snake
/// Then its up to the handler to decide if that direction is valid
#[derive(Event)]
struct ProposeDirection {
    id: Id,
    direction: Direction,
}

///////////

#[derive(Resource)]
struct SnakeTimer(Timer);
