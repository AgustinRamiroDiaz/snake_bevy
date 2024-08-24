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
        app.init_state::<AppState>()
            .add_systems(Update, game_state_transition);
    }
}

fn game_state_transition(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state_next_state.set(match app_state.get() {
            AppState::MainMenu => AppState::InGame,
            AppState::InGame => AppState::MainMenu,
        })
    }
}
