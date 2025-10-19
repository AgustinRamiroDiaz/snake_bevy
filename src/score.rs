use bevy::prelude::*;

use crate::{game_state, main_menu::NumberOfPlayersSelected, snake::MyColor, snake::Snake};

pub(crate) struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (update_score).run_if(in_state(game_state::AppState::InGame)),
        );
    }
}

#[derive(Component)]
struct Score;

fn setup(mut commands: Commands, _number_of_players: Res<NumberOfPlayersSelected>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        Node {
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        Score,
    ));
}

fn update_score(snakes: Query<(&Snake, &MyColor)>, mut text: Query<&mut Text, With<Score>>) {
    let mut snakes: Vec<_> = snakes.iter().collect();
    snakes.sort_by_key(|(snake, _)| -1 * snake.segments.len() as i8);

    let score_text = snakes
        .iter()
        .map(|(snake, _color)| format!("{} {}\n", snake.segments.len(), snake.name))
        .collect::<String>();

    if let Ok(mut text) = text.single_mut() {
        **text = score_text;
    }
}
