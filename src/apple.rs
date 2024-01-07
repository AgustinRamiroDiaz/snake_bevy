use bevy::prelude::*;
use rand::Rng;

use crate::snake::Tile;

use super::{
    asset_loader::SceneAssets,
    coordinate::Coordinate,
    game_state::AppState,
    schedule::InGameSet,
    snake::{Depth, Snake},
    HALF_LEN,
};

pub(crate) struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AppleEaten>()
            .add_systems(Startup, setup)
            .add_systems(
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

#[derive(Event)]
pub(crate) struct AppleEaten(pub(crate) Entity);

fn eat_apple(
    mut commands: Commands,
    mut snakes: Query<(Entity, &mut Snake)>,
    coordinates: Query<&Coordinate>,
    apples: Query<(Entity, &Coordinate), With<Apple>>,
    assets: Res<SceneAssets>,
    mut apple_eaten: EventWriter<AppleEaten>,
) {
    let get_head = |snake: &Snake| {
        let &head = snake.segments.front()?;
        coordinates.get(head).ok()
    };

    for (entity, snake) in snakes.iter_mut() {
        for (apple, coord) in apples.iter() {
            if Some(coord) == get_head(&snake) {
                // The despawn and spawn could be handled by events, but that would require configuring ordering in order to make sure we don't get to an inconsistent state. https://bevy-cheatbook.github.io/programming/events.html#possible-pitfalls
                commands.entity(apple).despawn();
                spawn_apple(&mut commands, &assets);
                apple_eaten.send(AppleEaten(entity));
                return;
            }
        }
    }
}
