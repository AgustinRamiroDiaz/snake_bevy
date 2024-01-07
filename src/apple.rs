use bevy::prelude::*;
use rand::Rng;

use crate::snake::Tile;

use super::{
    asset_loader::SceneAssets,
    coordinate::Coordinate,
    game_state::AppState,
    schedule::InGameSet,
    snake::{Depth, MyColor, Snake, SnakeSegment},
    HALF_LEN,
};

pub(crate) struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            eat_apple
                .in_set(InGameSet::SpawnDespawnEntities)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup(mut commands: Commands, assets: Res<SceneAssets>) {
    spawn_apple(&mut commands, &assets);
}

fn spawn_apple(commands: &mut Commands, assets: &Res<SceneAssets>) {
    commands.spawn((
        Apple,
        Depth(1.0),
        Coordinate(Vec2::new(
            rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
            rand::thread_rng().gen_range(-HALF_LEN..HALF_LEN) as f32,
        )),
        SpriteBundle {
            texture: assets.apple.clone(),
            ..default()
        },
        Tile,
    ));
}

#[derive(Component)]
pub(crate) struct Apple;

// TODO: decouple this logic into smaller units
// TODO: decouple spawning from eating
fn eat_apple(
    mut commands: Commands,
    mut snakes: Query<(&mut Snake, &MyColor)>,
    coordinates: Query<&Coordinate>,
    apples: Query<(Entity, &Coordinate), With<Apple>>,
    assets: Res<SceneAssets>,
) {
    let get_head = |snake: &Snake| {
        let &head = snake.segments.front()?;
        coordinates.get(head).ok()
    };

    for (mut snake, &color) in snakes.iter_mut() {
        for (apple, coord) in apples.iter() {
            if coord == get_head(&snake).unwrap() {
                commands.entity(apple).despawn();
                spawn_apple(&mut commands, &assets);

                let tail = commands
                    .spawn((color, SnakeSegment, snake.trail.clone(), Tile))
                    .id();

                snake.segments.push_back(tail);
                return;
            }
        }
    }
}
