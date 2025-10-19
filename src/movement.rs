use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{coordinate::Coordinate, game_state, snake::Snake, Direction, Id};

const SNAKE_TICK_SECONDS: f32 = 0.1;

pub(crate) struct SnakeMovementPlugin;

impl Plugin for SnakeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(
            SNAKE_TICK_SECONDS,
            TimerMode::Repeating,
        )))
        .add_plugins(InputManagerPlugin::<Direction>::default())
        .add_message::<ProposeDirection>()
        .add_message::<Tick>()
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

pub(crate) struct Tick;

impl bevy::ecs::message::Message for Tick {}

fn tick(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut query: Query<&mut Snake>,
    mut entity_query: Query<&mut Coordinate>,
    mut tick_writer: MessageWriter<Tick>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        tick_writer.write(Tick);
        query
            .iter_mut()
            .flat_map(|mut snake| {
                snake.input_blocked = false;
                let &tail_entity = snake.segments.back()?;
                let &head_entity = snake.segments.front()?;

                let head = entity_query.get_mut(head_entity).unwrap();

                let head_translation = head.0;

                if let Ok(mut tail) = entity_query.get_mut(tail_entity) {
                    snake.trail = Coordinate(tail.0); // TODO: remove double conversion
                    tail.0 = head_translation + Into::<Vec2>::into(snake.direction.clone());
                    snake.segments.rotate_right(1);
                }
                Some(())
            })
            .for_each(|_| ());
    }
}

/// This event proposes a direction for the snake
/// Then its up to the handler to decide if that direction is valid
pub(crate) struct ProposeDirection {
    pub(crate) id: Id,
    pub(crate) direction: Direction,
}

impl bevy::ecs::message::Message for ProposeDirection {}

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
        if let Ok(mut entity) = commands.get_entity(entity) {
            // VIM ordering
            let directions = [
                Direction::Left,
                Direction::Down,
                Direction::Up,
                Direction::Right,
            ];

            let player_keys = match snake.player_number.0 {
                1 => [
                    KeyCode::ArrowLeft,
                    KeyCode::ArrowDown,
                    KeyCode::ArrowUp,
                    KeyCode::ArrowRight,
                ],
                2 => [KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyW, KeyCode::KeyD],
                3 => [KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyI, KeyCode::KeyL],
                4 => [
                    KeyCode::Numpad4,
                    KeyCode::Numpad5,
                    KeyCode::Numpad8,
                    KeyCode::Numpad6,
                ],
                other => panic!("Invalid player number {}, only 1-4 supported", other),
            };

            // TODO: handle gamepad controls for more players
            // Currently only player 1 is supported
            // You'll also need to add logic to handle the Gamepad Id in the input map below
            let player_gamepad = match snake.player_number.0 {
                1 => [
                    vec![GamepadButton::DPadLeft],
                    vec![GamepadButton::DPadDown],
                    vec![GamepadButton::DPadUp],
                    vec![GamepadButton::DPadRight],
                ],
                2 => [vec![], vec![], vec![], vec![]],
                3 => [vec![], vec![], vec![], vec![]],
                4 => [vec![], vec![], vec![], vec![]],
                other => panic!("Invalid player number {}, only 1-4 supported", other),
            };

            let mut input_map = InputMap::default();

            for (player_controls, direction) in std::iter::zip(player_keys, directions) {
                input_map.insert(direction, player_controls);
            }

            for (player_controls, direction) in std::iter::zip(player_gamepad, directions) {
                input_map.insert_one_to_many(direction, player_controls);
            }

            // TODO: Query for available gamepads and assign Entity for multiplayer
            // For now, this will accept input from any connected gamepad
            // To assign a specific gamepad, query for gamepad entities and use:
            // input_map.set_gamepad(gamepad_entity);

            entity.insert((
                // Stores "which actions are currently pressed"
                ActionState::<Direction>::default(),
                // Describes how to convert from player inputs into those actions
                input_map,
            ));
        }
    }
}

fn handle_snake_direction(
    mut snakes: Query<&mut Snake>,
    mut proposed_direction: MessageReader<ProposeDirection>,
) {
    for proposed_direction in proposed_direction.read() {
        for mut snake in snakes
            .iter_mut()
            .filter(|snake| snake.player_number == proposed_direction.id)
            .filter(|snake| !snake.input_blocked)
        {
            if snake.direction == !proposed_direction.direction {
                return;
            }
            if snake.direction == proposed_direction.direction {
                return;
            }
            snake.direction = proposed_direction.direction;
            snake.input_blocked = true;
        }
    }
}

fn input_snake_direction(
    mut query: Query<(&mut Snake, &ActionState<Direction>)>,
    mut propose_direction_writer: MessageWriter<ProposeDirection>,
) {
    for (snake, direction) in query.iter_mut().filter(|(snake, _)| !snake.input_blocked) {
        let direction = if direction.just_pressed(&Direction::Left) {
            Some(Direction::Left)
        } else if direction.just_pressed(&Direction::Right) {
            Some(Direction::Right)
        } else if direction.just_pressed(&Direction::Up) {
            Some(Direction::Up)
        } else if direction.just_pressed(&Direction::Down) {
            Some(Direction::Down)
        } else {
            None
        };

        if let Some(direction) = direction {
            propose_direction_writer.write(ProposeDirection {
                id: snake.player_number.clone(),
                direction,
            });
        }
    }
}
