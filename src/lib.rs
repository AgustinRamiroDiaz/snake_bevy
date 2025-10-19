// Library exports for testing and potential reuse

pub mod coordinate;
pub mod direction;
pub mod main_menu;
pub mod game_state;
pub mod ai;
pub mod asset_loader;
pub mod movement;
pub mod score;
pub mod win;
pub mod apple;
pub mod collision;
pub mod blink;
pub mod schedule;
pub mod snake;

// Common constants
pub const SIZE: f32 = 0.8;
pub const HALF_LEN: i32 = 7;
pub const BOARD_LEN: i32 = 2 * HALF_LEN;
pub const PADDING: f32 = 1.0;
pub const BOARD_VIEWPORT_IN_WORLD_UNITS: f32 = BOARD_LEN as f32 + 2.0 * PADDING;
pub const MAX_NUMBER_OF_PLAYERS: usize = 4;

use bevy::prelude::*;

/// Initialize app with all game plugins for testing
/// Uses MinimalPlugins for headless testing (no rendering/window needed)
/// Note: Excludes MainMenu (EguiPlugin) and AssetLoaderPlugin as they require full asset system
pub fn init_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        // Use MinimalPlugins for headless testing (no rendering/window needed)
        MinimalPlugins,
        // Add AssetPlugin for asset loading (required by some game systems)
        bevy::asset::AssetPlugin::default(),
        // Add StatesPlugin (required for AppState)
        bevy::state::app::StatesPlugin,
        // Add InputPlugin (required for leafwing-input-manager)
        bevy::input::InputPlugin,
        // Add game plugins
        snake::SnakePlugin,
        movement::SnakeMovementPlugin,
        game_state::GameStatePlugin,
        score::ScorePlugin,
        win::WinPlugin,
        apple::ApplePlugin,
        collision::CollisionPlugin,
        schedule::SchedulePlugin,
    ));

    // Initialize NumberOfPlayersSelected resource (normally set by MainMenu)
    app.insert_resource(main_menu::NumberOfPlayersSelected(MAX_NUMBER_OF_PLAYERS));

    // Initialize SceneAssets with default values (normally loaded by AssetLoaderPlugin)
    app.init_resource::<asset_loader::SceneAssets>();

    app
}
