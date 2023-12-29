use bevy::prelude::*;

use super::coordinate::Coordinate;

use super::game_state::AppState;
use super::Snake;

pub(crate) struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (collision).run_if(in_state(AppState::InGame)));
    }
}

const INMORTAL_TICKS: u8 = 10;
const PROPORTION_LOST_PER_HIT: f32 = 0.3;

#[derive(Event)]
struct Collision(Entity);

fn collision(
    mut commands: Commands,
    mut snake_query: Query<(Entity, &mut Snake)>,
    query: Query<&Coordinate>,
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
            for _ in 0..(snake.segments.len() as f32 * PROPORTION_LOST_PER_HIT).ceil() as i8 {
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
