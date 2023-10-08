use std::collections::VecDeque;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, SnakePlugin)).run();
}

struct SnakePlugin;

const SNAKE_TICK_SECONDS: f32 = 0.3;
const SIZE: f32 = 20.0;
const GAP: f32 = 4.0;
const HALF_LEN: i32 = 10;

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
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // cells
    let mut cells_with_mm = vec![];

    for x in -HALF_LEN..HALF_LEN {
        for y in -HALF_LEN..HALF_LEN {
            let pos = Vec3::new(x as f32 * (SIZE + GAP), y as f32 * (SIZE + GAP), 0.0);
            // dbg!(&pos);
            cells_with_mm.push((SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            },));
        }
    }
    commands.spawn_batch(cells_with_mm);

    let sprite_bundle_at = |x: f32, y: f32| SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
            color: Color::BLACK,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
        ..Default::default()
    };

    let head = commands
        .spawn((
            sprite_bundle_at(0.0, 0.0),
            SnakeSegment,
            Coordinate(Vec2::new(0.0, 0.0)),
        ))
        .id();

    let tail = commands
        .spawn((
            sprite_bundle_at(SIZE + GAP, 0.0),
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
    },));

    let head2 = commands
        .spawn((
            sprite_bundle_at(0.0, 0.0),
            SnakeSegment,
            Coordinate(Vec2::new(0.0, 0.0)),
        ))
        .id();

    let mut segments2 = VecDeque::new();
    segments2.push_front(head2);

    commands.spawn((Snake {
        segments: segments2,
        direction: Direction::Right,
    },));
}

// https://www.reddit.com/r/bevy/comments/yen4hg/best_practices_when_dealing_with_a_collection_of/
#[derive(Component, Debug)]
struct Snake {
    segments: VecDeque<Entity>,
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Coordinate(Vec2);

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Snake>,
    mut entityQuery: Query<&mut Coordinate>,
    mut commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut snake in query.iter_mut() {
            println!("{:#?}", snake);

            // TODO: don't unwrap
            let &tail_entity = snake.segments.back().unwrap();
            let &head_entity = snake.segments.front().unwrap();

            let head = entityQuery.get_mut(head_entity).unwrap();

            let head_translation = head.0;

            if let Ok(mut tail) = entityQuery.get_mut(tail_entity) {
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

fn see_snake(query: Query<&Snake, Changed<Snake>>) {
    for snake in query.iter() {
        println!("{:#?}", snake);
    }
}

///////////
#[derive(Resource)]
struct SnakeTimer(Timer);

#[derive(Clone, Debug)]
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
