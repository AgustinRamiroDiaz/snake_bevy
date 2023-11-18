use bevy::prelude::*;

use crate::coordinate::Coordinate;
use crate::Apple;
use crate::Direction;
use crate::Id;
use crate::ProposeDirection;
use crate::Snake;

use rand;

// TODO:
// - fix: this snake can go backwards into itself
// - improve: this snake doesn't do the shortest path accounting for the toroid

pub(crate) struct AIPlugin {
    pub(crate) player_numbers: Vec<Id>,
}

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayersToFollow(self.player_numbers.clone()))
            .add_systems(Update, go_to_apple);
    }
}

#[derive(Resource)]
struct PlayersToFollow(Vec<Id>);

fn go_to_apple(
    mut snakes: Query<&mut Snake>,
    players_to_follow: Res<PlayersToFollow>,
    apples: Query<(&Apple, &Coordinate)>,
    coordinates: Query<&Coordinate>,
    mut propose_direction: EventWriter<ProposeDirection>,
) {
    if let Some((_, apple)) = apples.iter().next() {
        for snake in snakes
            .iter_mut()
            .filter(|s| players_to_follow.0.contains(&s.player_number))
        {
            // TODO: don't unwrap
            let snake_head = coordinates.get(*snake.segments.front().unwrap()).unwrap();

            let direction_x = if snake_head.0.x > apple.0.x {
                Some(Direction::Left)
            } else if snake_head.0.x < apple.0.x {
                Some(Direction::Right)
            } else {
                None
            };

            let direction_y = if snake_head.0.y > apple.0.y {
                Some(Direction::Down)
            } else if snake_head.0.y < apple.0.y {
                Some(Direction::Up)
            } else {
                None
            };

            let direction = if rand::random() {
                direction_x
            } else {
                direction_y
            };

            if let Some(direction) = direction {
                propose_direction.send(ProposeDirection {
                    id: snake.player_number.clone(),
                    direction,
                });
            }
        }
    }
}