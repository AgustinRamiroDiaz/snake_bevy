use bevy::prelude::*;

use super::game_state::AppState;

pub(crate) struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnEnter(AppState::MainMenu), spawn_buttons)
            .add_systems(OnExit(AppState::MainMenu), despawn_buttons);
    }
}

#[derive(Component)]
struct NumberOfPlayersButton;

fn button_system(mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                dbg!("Pressed");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn spawn_buttons(mut commands: Commands) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    // width: Val::Px(150.0),
                    // height: Val::Px(65.0),
                    // border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            NumberOfPlayersButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}

fn despawn_buttons(mut commands: Commands, buttons: Query<Entity, With<NumberOfPlayersButton>>) {
    buttons
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
}
