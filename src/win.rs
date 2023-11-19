use bevy::prelude::*;

use crate::game_state::AppState;
use crate::Snake;

const LENGTH_TO_WIN: usize = 15;

pub(crate) struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Won>()
            .add_systems(Update, win.run_if(in_state(AppState::InGame)));
    }
}

// TODO: should this event get injected from main into this plugin?
#[derive(Event)]
pub(crate) struct Won(pub(crate) String);

fn win(snakes: Query<&Snake, Changed<Snake>>, mut won: EventWriter<Won>) {
    let mut snakes = Vec::from_iter(snakes.iter());
    snakes.sort_by_key(|snake| snake.segments.len() as i8);

    let first = snakes.pop();
    let second = snakes.pop();

    if let (Some(first), Some(second)) = (first, second) {
        if first.segments.len() >= LENGTH_TO_WIN && first.segments.len() > second.segments.len() {
            won.send(Won(first.name.clone()));
        }
    }
}
