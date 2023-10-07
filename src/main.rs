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
        .add_systems(Update, (tick,));
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

    let head = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                color: Color::BLACK,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        SnakeSegment,
    ));

    // let mut segments = VecDeque::new();
    // segments.push_front(head.id());

    // commands.spawn((Snake { segments },));

    //     {
    //     head: SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
    //             color: Color::BLACK,
    //             ..Default::default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //         ..Default::default()
    //     },
    //     body: Vec::new(),
    //     direction: Direction::Left,
    // },
}

// https://www.reddit.com/r/bevy/comments/yen4hg/best_practices_when_dealing_with_a_collection_of/
#[derive(Component)]
struct Snake {
    segments: VecDeque<Entity>,
}

#[derive(Component)]
struct SnakeSegment;

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Transform, With<SnakeSegment>>,
    mut commands: Commands,
) {
    debug!("tick");
    if timer.0.tick(time.delta()).just_finished() {
        for mut s in query.iter_mut() {
            debug!(s.translation.x);
            debug!(s.translation.y);
            let direction = Direction::Left;

            s.translation += Into::<Vec3>::into(direction) * (SIZE + GAP);
        }
    }
}

// fn update_chunk_transform(mut commands: Commands, mut query: Query<(&Snake, &mut Transform)>) {
//     for (snake, mut transform) in query.iter_mut() {
//         transform.translation = snake.position.extend(0.0) * (SIZE + GAP) // TODO: tidy this constant
//     }
// }

///////////
#[derive(Resource)]
struct SnakeTimer(Timer);

#[derive(Clone)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Into<Vec3> for Direction {
    fn into(self) -> Vec3 {
        let x = match self {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
        };

        Vec2::new(x.0 as f32, x.1 as f32).extend(0.0)
    }
}
