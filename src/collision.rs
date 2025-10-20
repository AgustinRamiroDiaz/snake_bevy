use bevy::prelude::*;

use super::coordinate::Coordinate;

use super::game_state::AppState;
use super::snake::Snake;

use super::blink::{BlinkPlugin, Blinking};
use super::movement::Tick;

pub(crate) struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlinkPlugin)
            .add_message::<Collision>()
            .add_message::<RemoveChunks>()
            .add_message::<SetInmortal>()
            .add_systems(
                Update,
                (
                    collision_detection,
                    collision_handling,
                    remove_chunks,
                    set_inmortal,
                    update_inmortal_ticks,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

const INMORTAL_TICKS: u8 = 10;
const PROPORTION_LOST_PER_HIT: f32 = 0.3;

#[derive(Message)]
/// Represents the snake entity that has hit its head against something
struct Collision(Entity);

fn collision_detection(
    mut snake_query: Query<(Entity, &mut Snake)>,
    query: Query<&Coordinate>,
    changed_coordinates: Query<Entity, Changed<Coordinate>>,
    mut collision: MessageWriter<Collision>,
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
            collision.write(Collision(entity));
        }
    }
}

fn collision_handling(
    mut collision: MessageReader<Collision>,
    mut remove_chunks: MessageWriter<RemoveChunks>,
    mut set_inmortal: MessageWriter<SetInmortal>,
) {
    for &Collision(entity) in collision.read() {
        remove_chunks.write(RemoveChunks(entity));
        set_inmortal.write(SetInmortal(entity));
    }
}

#[derive(Message)]
struct RemoveChunks(Entity);

fn remove_chunks(
    mut commands: Commands,
    mut event_reader: MessageReader<RemoveChunks>,
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
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[derive(Message)]
struct SetInmortal(Entity);

fn set_inmortal(
    mut commands: Commands,
    mut event_reader: MessageReader<SetInmortal>,
    mut query: Query<&mut Snake>,
) {
    for &SetInmortal(entity) in event_reader.read() {
        if let Ok(mut snake) = query.get_mut(entity) {
            snake.inmortal_ticks = INMORTAL_TICKS;
            // TODO: how can we decouple the blinking from the inmortality?
            for &segment in snake.segments.iter() {
                commands.entity(segment).insert(Blinking);
            }
        }
    }
}

fn update_inmortal_ticks(
    mut commands: Commands,
    mut query: Query<&mut Snake>,
    mut tick: MessageReader<Tick>,
) {
    for _ in tick.read() {
        for mut snake in query.iter_mut() {
            snake.inmortal_ticks = snake.inmortal_ticks.saturating_sub(1);

            if snake.inmortal_ticks == 0 {
                for &segment in snake.segments.iter() {
                    commands.entity(segment).remove::<Blinking>();
                }
            }
        }
    }
}
