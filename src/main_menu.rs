use bevy::prelude::*;

use crate::win::Won;

use super::game_state::AppState;

use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(crate) struct MainMenu {
    pub(crate) max_number_of_players: usize,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(MaxNumberOfPlayers(self.max_number_of_players))
            .insert_resource(NumberOfPlayersSelected(self.max_number_of_players))
            .add_systems(Update, (selection).run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, (winner_text, how_to_play))
            .add_systems(OnExit(AppState::MainMenu), remove_winner_text);
    }
}

fn how_to_play(mut contexts: EguiContexts) {
    egui::Window::new("How to play").show(contexts.ctx_mut(), |ui| {
        ui.label("`Esc` escape key to open the menu");
        ui.label("`Esc` escape key to get back into the game");
        ui.label("`Arrow keys` to move player 1");
        ui.label("`WASD` to move player 2");
        ui.label("`IJKL` to move player 3");
        ui.label("`Numpad 8456` to move player 4");
    });
}

fn selection(
    mut contexts: EguiContexts,
    mut number_of_players_selected: ResMut<NumberOfPlayersSelected>,
    max_number_of_players: Res<MaxNumberOfPlayers>,
) {
    egui::Window::new("Number of players selection").show(contexts.ctx_mut(), |ui| {
        ui.heading("Choose");
        ui.add(
            egui::Slider::new(
                &mut number_of_players_selected.0,
                1..=max_number_of_players.0,
            )
            .text("Number of players"),
        );
        ui.label(format!("{} players selected", number_of_players_selected.0));
    });
}

#[derive(Resource)]
struct MaxNumberOfPlayers(usize);

#[derive(Resource)]
pub struct NumberOfPlayersSelected(pub usize);

#[derive(Component)]
struct WinnerText;

fn winner_text(
    mut commands: Commands,
    mut event_reader: EventReader<Won>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if !event_reader.is_empty() {
        app_state_next_state.set(AppState::MainMenu);
    }

    for event in event_reader.read() {
        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                format!("Player {} won", event.0),
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ) // Set the alignment of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
            WinnerText,
        ));
    }
}

fn remove_winner_text(mut commands: Commands, query: Query<Entity, With<WinnerText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
