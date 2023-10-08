use std::collections::VecDeque;

use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
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
const SIZE: f32 = 20.0;
const GAP: f32 = 4.0;
const HALF_LEN: i32 = 20;

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
                see_snake,
                update_local_coordinates_to_world_transforms,
                input_snake_direction,
                toroid_coordinates,
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut grid = vec![];

    for x in -HALF_LEN..HALF_LEN {
        for y in -HALF_LEN..HALF_LEN {
            grid.push((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Coordinate(Vec2::new(x as f32, y as f32)),
            ));
        }
    }
    commands.spawn_batch(grid);

    let new_black_tile = || SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
            color: Color::BLACK,
            ..Default::default()
        },
        ..Default::default()
    };

    let head = commands
        .spawn((
            new_black_tile(),
            SnakeSegment,
            Coordinate(Vec2::new(0.0, 0.0)),
        ))
        .id();

    let tail = commands
        .spawn((
            new_black_tile(),
            SnakeSegment,
            Coordinate(Vec2::new(1.0, 0.0)),
        ))
        .id();

    let mut segments = VecDeque::new();
    segments.push_front(head);
    segments.push_front(tail);

    commands.spawn((Snake {
        segments,
        direction: Direction::Left,
        player_number: Id::One,
    },));

    let head2 = commands
        .spawn((
            new_black_tile(),
            SnakeSegment,
            Coordinate(Vec2::new(0.0, 0.0)),
        ))
        .id();

    let mut segments2 = VecDeque::new();
    segments2.push_front(head2);

    commands.spawn((Snake {
        segments: segments2,
        direction: Direction::Right,
        player_number: Id::Two,
    },));
}

#[derive(Component, Debug)]
struct Snake {
    segments: VecDeque<Entity>,
    direction: Direction,
    player_number: Id,
}

#[derive(Component, Debug)]
enum Id {
    One,
    Two,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Coordinate(Vec2);

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Snake>,
    mut entity_query: Query<&mut Coordinate>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut snake in query.iter_mut() {
            println!("{:#?}", snake);

            // TODO: don't unwrap
            let &tail_entity = snake.segments.back().unwrap();
            let &head_entity = snake.segments.front().unwrap();

            let head = entity_query.get_mut(head_entity).unwrap();

            let head_translation = head.0;

            if let Ok(mut tail) = entity_query.get_mut(tail_entity) {
                println!("moving");
                tail.0 = head_translation + Into::<Vec2>::into(snake.direction.clone());
                snake.segments.rotate_right(1);
            }
        }
    }
}

fn update_local_coordinates_to_world_transforms(
    mut query: Query<(&Coordinate, &mut Transform), Changed<Coordinate>>,
) {
    for (coordinate, mut transform) in query.iter_mut() {
        transform.translation = coordinate.0.extend(0.0) * (SIZE + GAP)
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

// Eventually we could use https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/multiplayer.rs for better input handling
fn input_snake_direction(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    for mut snake in query.iter_mut() {
        let direction =
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                Some(Direction::Left)
            } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                Some(Direction::Right)
            } else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                Some(Direction::Up)
            } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                Some(Direction::Down)
            } else {
                None
            };

        if let Some(direction) = direction {
            if snake.direction == !direction.clone() {
                continue;
            }
            snake.direction = direction;
        }
    }
}

fn see_snake(query: Query<&Snake, Changed<Snake>>) {
    for snake in query.iter() {
        println!("{:#?}", snake);
    }
}

///////////
#[derive(Resource)]
struct SnakeTimer(Timer);

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Into<Vec2> for Direction {
    fn into(self) -> Vec2 {
        let x = match self {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
        };

        Vec2::new(x.0 as f32, x.1 as f32)
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}
