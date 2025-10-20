#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{MonitorSelection, WindowMode};
use bevy::{asset::AssetMetaCheck, prelude::*};
use movement::ProposeDirection;

mod coordinate;

mod direction;
use direction::Direction;

mod main_menu;
use main_menu::MainMenu;

mod game_state;
use game_state::GameStatePlugin;

mod ai;
use ai::AIPlugin;

mod asset_loader;
use asset_loader::AssetLoaderPlugin;

mod movement;
use movement::SnakeMovementPlugin;

mod score;
use score::ScorePlugin;

mod win;
use win::WinPlugin;

mod apple;
use apple::ApplePlugin;

mod collision;
use collision::CollisionPlugin;

mod blink;

mod schedule;
use schedule::SchedulePlugin;

mod snake;
use snake::{Id, SnakePlugin};

use std::env;

const SIZE: f32 = 0.8;
const HALF_LEN: i32 = 7;
const BOARD_LEN: i32 = 2 * HALF_LEN;
const PADDING: f32 = 1.0;
const BOARD_VIEWPORT_IN_WORLD_UNITS: f32 = BOARD_LEN as f32 + 2.0 * PADDING;

const MAX_NUMBER_OF_PLAYERS: usize = 4;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never, // https://github.com/bevyengine/bevy/issues/10157#issuecomment-2217168402
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // Borderless looks distorted when running in the web
                    #[cfg(not(target_arch = "wasm32"))]
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
        SnakePlugin,
        SnakeMovementPlugin,
        MainMenu {
            max_number_of_players: MAX_NUMBER_OF_PLAYERS,
        },
        GameStatePlugin,
        AssetLoaderPlugin,
        ScorePlugin,
        WinPlugin,
        ApplePlugin,
        CollisionPlugin,
        SchedulePlugin,
    ));

    if env::var("AI").unwrap_or("false".to_string()) == "true" {
        app.add_plugins(AIPlugin {
            player_numbers: vec![Id(1), Id(2), Id(3), Id(4)],
        });
    }

    app.run();
}
