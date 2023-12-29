use bevy::prelude::*;

use super::coordinate::Coordinate;

use super::game_state::AppState;
use super::Snake;

pub(crate) struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Collision>()
            .add_event::<RemoveChunks>()
            .add_systems(
                Update,
                (collision_detection, collision_handling, remove_chunks)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

const INMORTAL_TICKS: u8 = 10;
const PROPORTION_LOST_PER_HIT: f32 = 0.3;

#[derive(Event)]
/// Represents the snake entity that has hit its head against something
struct Collision(Entity);

fn collision_detection(
    mut snake_query: Query<(Entity, &mut Snake)>,
    query: Query<&Coordinate>,
    changed_coordinates: Query<Entity, Changed<Coordinate>>,
    mut collision: EventWriter<Collision>,
) {
    if changed_coordinates.iter().count() == 0 {
        // This is an efficiency hack to just evaluate collisions when the state has changed
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

    for (entity, snake) in snake_query
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
            collision.send(Collision(entity));
        }
    }
}

fn collision_handling(
    mut collision: EventReader<Collision>,
    mut remove_chunks: EventWriter<RemoveChunks>,
) {
    for Collision(entity) in collision.read() {
        remove_chunks.send(RemoveChunks(*entity));
    }
}

#[derive(Event)]
struct RemoveChunks(Entity);

fn remove_chunks(
    mut commands: Commands,
    mut event_reader: EventReader<RemoveChunks>,
    mut query: Query<(Entity, &mut Snake)>,
) {
    for RemoveChunks(entity) in event_reader.read() {
        if let Ok((_, mut snake)) = query.get_mut(*entity) {
            let chunks_to_remove = std::cmp::min(
                snake.segments.len() - 1,
                (snake.segments.len() as f32 * PROPORTION_LOST_PER_HIT).ceil() as usize,
            );
            for _ in 0..chunks_to_remove {
                if let Some(entity) = snake.segments.pop_back() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            snake.inmortal_ticks = INMORTAL_TICKS;
        }
    }
}
