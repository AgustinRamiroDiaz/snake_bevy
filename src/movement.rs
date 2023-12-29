use bevy::prelude::*;

use crate::{coordinate::Coordinate, game_state, Direction, Id, Snake};

const SNAKE_TICK_SECONDS: f32 = 0.1;

pub(crate) struct SnakeMovementPlugin;

impl Plugin for SnakeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(
            SNAKE_TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_event::<ProposeDirection>()
        .add_event::<Tick>()
        .add_systems(
            Update,
            (tick, input_snake_direction, handle_snake_direction)
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

// Eventually we could use https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/multiplayer.rs for better input handling
fn input_snake_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Snake>,
    mut propose_direction: EventWriter<ProposeDirection>,
) {
    for snake in query.iter_mut().filter(|snake| !snake.input_blocked) {
        let direction = match snake.player_number.0 {
            1 => {
                if keyboard_input.pressed(KeyCode::Left) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::Right) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::Up) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::Down) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            2 => {
                if keyboard_input.pressed(KeyCode::A) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::D) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::W) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::S) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            3 => {
                if keyboard_input.pressed(KeyCode::J) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::L) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::I) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::K) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            4 => {
                if keyboard_input.pressed(KeyCode::Numpad4) {
                    Some(Direction::Left)
                } else if keyboard_input.pressed(KeyCode::Numpad6) {
                    Some(Direction::Right)
                } else if keyboard_input.pressed(KeyCode::Numpad8) {
                    Some(Direction::Up)
                } else if keyboard_input.pressed(KeyCode::Numpad5) {
                    Some(Direction::Down)
                } else {
                    None
                }
            }
            other => panic!("Invalid player number {}, only 1-4 supported", other),
        };

        if let Some(direction) = direction {
            propose_direction.send(ProposeDirection {
                id: snake.player_number.clone(),
                direction,
            });
        }
    }
}
