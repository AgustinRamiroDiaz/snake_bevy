use bevy::prelude::*;
use std::iter;

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

fn setup(mut commands: Commands, number_of_players: Res<NumberOfPlayersSelected>) {
    commands.spawn((
        TextBundle::from_sections((0..number_of_players.0).map(|_| TextSection::default()))
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        Score,
    ));
}

fn update_score(snakes: Query<(&Snake, &MyColor)>, mut text: Query<&mut Text, With<Score>>) {
    let mut snakes = Vec::from_iter(snakes.iter());
    snakes.sort_by_key(|(snake, _)| -1 * snake.segments.len() as i8);

    text.single_mut()
        .sections
        .resize(snakes.len(), TextSection::default());

    for (text_section, (snake, color)) in
        iter::zip(text.single_mut().sections.iter_mut(), snakes.iter())
    {
        text_section.value = format!("{} {}\n", snake.segments.len(), snake.name);
        text_section.style = TextStyle {
            font_size: 60.0,
            color: color.0,
            ..default()
        }
    }
}
