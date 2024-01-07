use bevy::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::apple::Apple;
use crate::coordinate::Coordinate;
use crate::snake::Snake;
use crate::Direction;
use crate::Id;
use crate::ProposeDirection;
use crate::HALF_LEN;

use rand;

// TODO:
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

            let mut direction_x = if snake_head.0.x > apple.0.x {
                Some(Direction::Left)
            } else if snake_head.0.x < apple.0.x {
                Some(Direction::Right)
            } else {
                None
            };

            if (snake_head.0.x - apple.0.x).abs() > HALF_LEN as f32 {
                direction_x = direction_x.map(|d| !d);
            }

            let mut direction_y = if snake_head.0.y > apple.0.y {
                Some(Direction::Down)
            } else if snake_head.0.y < apple.0.y {
                Some(Direction::Up)
            } else {
                None
            };

            if (snake_head.0.y - apple.0.y).abs() > HALF_LEN as f32 {
                direction_y = direction_y.map(|d| !d);
            };

            // Randomize decision so all snakes don't do the same
            let weights = [1, 1, 20];
            let choices = [direction_x, direction_y, None];
            let direction = &choices[WeightedIndex::new(weights)
                .unwrap()
                .sample(&mut rand::thread_rng())];

            if let Some(direction) = direction.to_owned() {
                propose_direction.send(ProposeDirection {
                    id: snake.player_number.clone(),
                    direction,
                });
            }
        }
    }
}
