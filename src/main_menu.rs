use bevy::prelude::*;

use super::game_state::AppState;

pub(crate) struct MainMenu {
    pub(crate) max_number_of_players: usize,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaxNumberOfPlayers(self.max_number_of_players))
            .insert_resource(NumberOfPlayersSelected(self.max_number_of_players))
            .add_systems(Update, (selection).run_if(in_state(AppState::MainMenu)));
    }
}

use bevy_egui::{egui, EguiContexts};

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
