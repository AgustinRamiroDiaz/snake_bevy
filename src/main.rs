use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, SnakePlugin)).run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Pet;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("peq".to_string())));
    commands.spawn((Person, Name("peq".to_string())));
    commands.spawn((Pet, Name("michi".to_string())));
    commands.spawn((Name("no tengo nombre".to_string()),));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(query: Query<&Name, With<Person>>, time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        query
            .iter()
            .map(|name| println!("hello {}", name.0))
            .for_each(drop);
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                color: Color::BLACK,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        Head,
    ));
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
struct Snake;
// {
//     head: SpriteBundle,
//     body: Vec<SpriteBundle>,
//     direction: Direction,
// }

#[derive(Component)]
struct Chunk(SpriteBundle, Direction);

#[derive(Component)]
struct Head;

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

#[derive(Resource)]
struct SnakeTimer(Timer);

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Transform, With<Head>>,
    mut commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut s in query.iter_mut() {
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
