use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{coordinate::Coordinate, game_state, Direction, Id, Snake};

const SNAKE_TICK_SECONDS: f32 = 0.1;

pub(crate) struct SnakeMovementPlugin;

impl Plugin for SnakeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(
            SNAKE_TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_plugins(InputManagerPlugin::<Direction>::default())
        .add_event::<ProposeDirection>()
        .add_event::<Tick>()
        .add_systems(
            Update,
            (
                tick,
                input_snake_direction,
                handle_snake_direction,
                add_snake_input_handler,
            )
                .run_if(in_state(game_state::AppState::InGame)),
        );
    }
}

#[derive(Resource)]
struct SnakeTimer(Timer);

#[derive(Event)]
pub(crate) struct Tick;

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Snake>,
    mut entity_query: Query<&mut Coordinate>,
    mut tick: EventWriter<Tick>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        tick.send(Tick);
        for mut snake in query.iter_mut() {
            snake.input_blocked = false;
            // TODO: don't unwrap
            let &tail_entity = snake.segments.back().unwrap();
            let &head_entity = snake.segments.front().unwrap();

            let head = entity_query.get_mut(head_entity).unwrap();

            let head_translation = head.0;

            if let Ok(mut tail) = entity_query.get_mut(tail_entity) {
                snake.trail = Coordinate(tail.0); // TODO: remove double conversion
                tail.0 = head_translation + Into::<Vec2>::into(snake.direction.clone());
                snake.segments.rotate_right(1);
            }
        }
    }
}

/// This event proposes a direction for the snake
/// Then its up to the handler to decide if that direction is valid
#[derive(Event)]
pub(crate) struct ProposeDirection {
    pub(crate) id: Id,
    pub(crate) direction: Direction,
}

// mut query: Query<(Entity, &MyColor), (Changed<Coordinate>, Without<Transform>)>,

fn add_snake_input_handler(
    mut commands: Commands,
    snakes: Query<
        (Entity, &Snake),
        (
            Without<InputMap<Direction>>,
            Without<ActionState<Direction>>,
        ),
    >,
) {
    for (entity, snake) in snakes.iter() {
        if let Some(mut entity) = commands.get_entity(entity) {
            let input_map = match snake.player_number.0 {
                1 => [
                    (KeyCode::Left, Direction::Left),
                    (KeyCode::Right, Direction::Right),
                    (KeyCode::Up, Direction::Up),
                    (KeyCode::Down, Direction::Down),
                ],
                2 => [
                    (KeyCode::A, Direction::Left),
                    (KeyCode::D, Direction::Right),
                    (KeyCode::W, Direction::Up),
                    (KeyCode::S, Direction::Down),
                ],
                3 => [
                    (KeyCode::J, Direction::Left),
                    (KeyCode::L, Direction::Right),
                    (KeyCode::I, Direction::Up),
                    (KeyCode::K, Direction::Down),
                ],
                4 => [
                    (KeyCode::Numpad4, Direction::Left),
                    (KeyCode::Numpad6, Direction::Right),
                    (KeyCode::Numpad8, Direction::Up),
                    (KeyCode::Numpad5, Direction::Down),
                ],
                other => panic!("Invalid player number {}, only 1-4 supported", other),
            };

            entity.insert(InputManagerBundle::<Direction> {
                // Stores "which actions are currently pressed"
                action_state: ActionState::default(),
                // Describes how to convert from player inputs into those actions
                input_map: InputMap::new(input_map),
            });
        }
    }
}

// TODO: can we remove the `clone`s?
fn handle_snake_direction(
    mut snakes: Query<&mut Snake>,
    mut proposed_direction: EventReader<ProposeDirection>,
) {
    for direction in proposed_direction.read() {
        for mut snake in snakes
            .iter_mut()
            .filter(|snake| snake.player_number == direction.id)
            .filter(|snake| !snake.input_blocked)
        {
            if snake.direction == !direction.direction.clone() {
                return;
            }
            if snake.direction == direction.direction.clone() {
                return;
            }
            snake.direction = direction.direction.clone();
            snake.input_blocked = true;
        }
    }
}

fn input_snake_direction(
    mut query: Query<(&mut Snake, &ActionState<Direction>)>,
    mut propose_direction: EventWriter<ProposeDirection>,
) {
    for (snake, direction) in query.iter_mut().filter(|(snake, _)| !snake.input_blocked) {
        let direction = if direction.just_pressed(Direction::Left) {
            Some(Direction::Left)
        } else if direction.just_pressed(Direction::Right) {
            Some(Direction::Right)
        } else if direction.just_pressed(Direction::Up) {
            Some(Direction::Up)
        } else if direction.just_pressed(Direction::Down) {
            Some(Direction::Down)
        } else {
            None
        };

        if let Some(direction) = direction {
            propose_direction.send(ProposeDirection {
                id: snake.player_number.clone(),
                direction,
            });
        }
    }
}
