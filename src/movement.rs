use bevy::prelude::*;

use crate::{game_state, Direction, Id, Snake};

pub(crate) struct SnakeMovementPlugin;

impl Plugin for SnakeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProposeDirection>().add_systems(
            Update,
            (handle_snake_direction).run_if(in_state(game_state::AppState::InGame)),
        );
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
