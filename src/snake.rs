use std::collections::VecDeque;

use bevy::{color::palettes::css, prelude::*};

use crate::{
    apple::AppleEaten, coordinate::Coordinate, direction::Direction, game_state::AppState,
    main_menu::NumberOfPlayersSelected, schedule::InGameSet, BOARD_VIEWPORT_IN_WORLD_UNITS,
    HALF_LEN, SIZE,
};

pub(crate) struct SnakePlugin;

const TILE_SIZE: f32 = 1.0;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_and_camera)
            .add_systems(
                Update,
                (
                    grow_snake,
                    ApplyDeferred,
                    toroid_coordinates,
                    add_sprite_bundles,
                    ApplyDeferred, // This is needed in order to render the sprites correctly, we need to flush the sprites into the world and then update their transforms
                    set_sprite_size,
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

fn setup_grid_and_camera(mut commands: Commands) {
    let mut grid = vec![];

    for x in -HALF_LEN..=HALF_LEN {
        for y in -HALF_LEN..=HALF_LEN {
            grid.push((
                Sprite {
                    custom_size: Some(Vec2 { x: SIZE, y: SIZE }),
                    color: Color::Srgba(css::DARK_SLATE_GRAY),
                    ..Default::default()
                },
                Coordinate(Vec2::new(x as f32, y as f32)),
                Depth(-1.0),
            ));
        }
    }
    commands.spawn_batch(grid);

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: BOARD_VIEWPORT_IN_WORLD_UNITS,
                min_height: BOARD_VIEWPORT_IN_WORLD_UNITS,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn spawn_snakes(mut commands: Commands, number_of_players: Res<NumberOfPlayersSelected>) {
    let mut spawn_snake =
        |id, spawn_coord: Coordinate, direction: Direction, color: MyColor, name: String| {
            let head_a = commands
                .spawn((color, SnakeSegment, spawn_coord.clone(), Tile))
                .id();

            commands.spawn((
                Snake {
                    segments: VecDeque::from([head_a]),
                    player_number: id,
                    direction: direction.clone(),
                    trail: Coordinate(spawn_coord.0 - <Direction as Into<Vec2>>::into(direction)),
                    input_blocked: false,
                    inmortal_ticks: 0,
                    name,
                },
                color,
                Tile,
            ));
        };

    let snakes = [
        (
            Id(1),
            Coordinate::from((-3.0, -3.0)),
            Direction::Right,
            MyColor(Color::Srgba(css::LIMEGREEN)),
            "Ninja".to_string(),
        ),
        (
            Id(2),
            Coordinate::from((3.0, 3.0)),
            Direction::Left,
            MyColor(Color::Srgba(css::PINK)),
            "Panther".to_string(),
        ),
        (
            Id(3),
            Coordinate::from((-3.0, 3.0)),
            Direction::Down,
            MyColor(Color::Srgba(css::SALMON)),
            "Sushi".to_string(),
        ),
        (
            Id(4),
            Coordinate::from((3.0, -3.0)),
            Direction::Up,
            MyColor(Color::Srgba(css::TURQUOISE)),
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
            .for_each(|&entity| commands.entity(entity).despawn());
        commands.entity(entity).despawn();
    });
}

#[derive(Component)]
pub(crate) struct Snake {
    pub(crate) name: String,
    pub(crate) segments: VecDeque<Entity>,
    pub(crate) direction: Direction,
    pub(crate) player_number: Id,
    pub(crate) trail: Coordinate,
    pub(crate) input_blocked: bool, // TODO: is it reasonable to move this into its own component given that we filter by it?
    pub(crate) inmortal_ticks: u8,
}

#[derive(Component, Debug, PartialEq, Clone)]
pub(crate) struct Id(pub(crate) u8);

#[derive(Component)]
pub(crate) struct SnakeSegment;

fn grow_snake(
    mut commands: Commands,
    mut query: Query<(&mut Snake, &MyColor)>,
    mut apple_eaten: EventReader<AppleEaten>,
) {
    for AppleEaten(entity) in apple_eaten.read() {
        if let Ok((mut snake, &color)) = query.get_mut(*entity) {
            let tail = commands
                .spawn((color, SnakeSegment, snake.trail.clone(), Tile))
                .id();

            snake.segments.push_back(tail);
        }
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

// UI sprite handling

#[derive(Component, Clone, Copy)]
pub(crate) struct MyColor(pub(crate) Color);

#[derive(Component)]
pub(crate) struct Depth(pub(crate) f32);

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

// TODO: we assume that Transform == Sprite
fn add_sprite_bundles(
    query: Query<(Entity, &MyColor), (Changed<Coordinate>, Without<Transform>)>,
    mut commands: Commands,
) {
    for (entity, color) in query.iter() {
        commands.entity(entity).insert(Sprite {
            color: color.0,
            ..Default::default()
        });
    }
}

#[derive(Component)]
pub(crate) struct Tile;

fn set_sprite_size(mut query: Query<&mut Sprite, Added<Tile>>) {
    for mut sprite in query.iter_mut() {
        sprite.custom_size = Some(Vec2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
        });
    }
}
