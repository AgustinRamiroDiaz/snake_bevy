use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub(crate) enum AppState {
    MainMenu,
    #[default]
    InGame,
}

pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Update, game_state_transition);
    }
}

fn game_state_transition(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::MainMenu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                app_state_next_state.set(AppState::InGame);
            }
        }
        AppState::InGame => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                app_state_next_state.set(AppState::MainMenu);
            }
        }
    }
}
