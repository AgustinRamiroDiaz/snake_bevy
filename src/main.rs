use std::collections::{HashSet, VecDeque};

use bevy::{prelude::*, window::WindowMode};
use rand::Rng;

mod coordinate;
use coordinate::Coordinate;

mod direction;
use direction::Direction;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(not(target_arch = "wasm32"))] // Borderless looks distorted when running in the web
                    mode: WindowMode::BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            SnakePlugin,
        ))
        .run();
}

struct SnakePlugin;

const SNAKE_TICK_SECONDS: f32 = 0.1;
const SIZE: f32 = 30.0;
const GAP: f32 = 4.0;
const HALF_LEN: i32 = 7;
const INMORTAL_TICKS: u8 = 10;
const CHUNKS_LOST_PER_HIT: u8 = 3;
const PADDING: f32 = 10.0;
const BOARD_VIEWPORT_IN_WORLD_UNITS: f32 =
    SIZE + 2.0 * (SIZE + GAP) * HALF_LEN as f32 + 2.0 * PADDING;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(
            SNAKE_TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tick,
                input_snake_direction,
                toroid_coordinates,
                eat_apple,
                collision,
            ),
        )
        .add_systems(
            PreUpdate,
            (
                update_local_coordinates_to_world_transforms,
                add_sprite_bundles,
            ),
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
                MyColor(Color::DARK_GRAY),
                Coordinate(Vec2::new(x as f32, y as f32)),
            ));
        }
    }
    commands.spawn_batch(grid);

    let snake_color_a = MyColor(Color::LIME_GREEN);

    let head_a = commands
        .spawn((snake_color_a, SnakeSegment, Coordinate::from((0.0, 0.0))))
        .id();

    let tail_a = commands
        .spawn((snake_color_a, SnakeSegment, Coordinate::from((1.0, 0.0))))
        .id();

    let mut segments = VecDeque::new();
    segments.push_front(head_a);
    segments.push_front(tail_a);

    commands.spawn((
        Snake {
            segments,
            direction: Direction::Left,
            player_number: Id::One,
            trail: Coordinate::from((0.0, 0.0)),
            input_blocked: false,
            inmortal_ticks: 0,
        },
        snake_color_a,
    ));

    let snake_color_b = MyColor(Color::PINK);
    let head_b = commands
        .spawn((snake_color_b, SnakeSegment, Coordinate::from((0.0, 1.0))))
        .id();

    let mut segments_b = VecDeque::new();
    segments_b.push_front(head_b);

    commands.spawn((
        Snake {
            segments: segments_b,
            direction: Direction::Right,
            player_number: Id::Two,
            trail: Coordinate::from((0.0, 1.0)),
            input_blocked: false,
            inmortal_ticks: 0,
        },
        snake_color_b,
    ));

    commands.spawn((MyColor(Color::RED), Apple, Coordinate::from((5.0, 5.0))));
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

#[derive(Component)]
struct Snake {
    segments: VecDeque<Entity>,
    direction: Direction,
    player_number: Id,
    trail: Coordinate,
    input_blocked: bool,
    inmortal_ticks: u8,
}

#[derive(Component)]
enum Id {
    One,
    Two,
}

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
    mut query: Query<(&Coordinate, &mut Transform), Or<(Changed<Coordinate>, Changed<Transform>)>>,
) {
    for (coordinate, mut transform) in query.iter_mut() {
        transform.translation = coordinate.0.extend(0.0) * (SIZE + GAP) // TODO: this logic is duplicated
    }
}

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
    query: Query<&Coordinate, With<SnakeSegment>>,
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
            for _ in 0..CHUNKS_LOST_PER_HIT {
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

// TODO: this could be more efficient by only checking the head. That would imply also changing the logic of where the apple spawns
// TODO: decouple this logic into smaller units
fn eat_apple(
    mut commands: Commands,
    snake_segments: Query<&Coordinate, With<SnakeSegment>>,
    mut snakes: Query<(&mut Snake, &MyColor)>,
    apples: Query<(Entity, &Coordinate), With<Apple>>,
) {
    for (mut snake, &color) in snakes.iter_mut() {
        let segments = snake.segments.iter().flat_map(|&e| snake_segments.get(e));

        let segments: HashSet<&Coordinate> = HashSet::from_iter(segments);

        for (apple, coord) in apples.iter() {
            if segments.contains(&coord) {
                commands.entity(apple).despawn();
                commands.spawn((
                    Apple,
                    MyColor(Color::RED),
                    Coordinate(Vec2::new(
                        rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
                        rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
                    )),
                ));

                let tail = commands
                    .spawn((color, SnakeSegment, snake.trail.clone()))
                    .id();

                snake.segments.push_back(tail);
            }
        }
    }
}

// Eventually we could use https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/multiplayer.rs for better input handling
// TODO: with this logic I can go back by pressing two keys at once
fn input_snake_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    for mut snake in query.iter_mut().filter(|snake| !snake.input_blocked) {
        let direction = match snake.player_number {
            Id::One => {
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
            Id::Two => {
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
        };

        // You cannot go back into yourself
        if let Some(direction) = direction {
            if snake.direction == !direction.clone() {
                continue;
            }
            if snake.direction == direction {
                continue;
            }
            snake.direction = direction;
            snake.input_blocked = true;
        }
    }
}

///////////
#[derive(Resource)]
struct SnakeTimer(Timer);
