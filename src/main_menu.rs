use bevy::prelude::*;

use super::game_state::AppState;

pub(crate) struct MainMenu {
    pub(crate) max_number_of_players: usize,
}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaxNumberOfPlayers(self.max_number_of_players))
            .insert_resource(NumberOfPlayersSelected(self.max_number_of_players))
            .add_systems(
                Update,
                (selection, border_updater).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnEnter(AppState::MainMenu), spawn_buttons)
            .add_systems(OnExit(AppState::MainMenu), despawn_buttons);
    }
}

#[derive(Resource)]
struct MaxNumberOfPlayers(usize);

#[derive(Component)]
struct MyButton;

#[derive(Component)]
struct ButtonNumber(usize);

#[derive(Resource)]
pub struct NumberOfPlayersSelected(pub usize);

const UNSELECTED_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SELECTED_COLOR: Color = Color::BLACK;

fn selection(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &ButtonNumber),
        (Changed<Interaction>, With<MyButton>),
    >,
    mut number_of_players_selected: ResMut<NumberOfPlayersSelected>,
) {
    for (interaction, mut border_color, button_number) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = SELECTED_COLOR;
                number_of_players_selected.0 = button_number.0;
            }
            Interaction::Hovered => {}
            Interaction::None => {
                // TODO: this does not belong here
                // border_color.0 = UNSELECTED_COLOR;
            }
        }
    }
}

fn border_updater(
    mut buttons: Query<(&mut BorderColor, &ButtonNumber), With<MyButton>>,
    changed_buttons: Query<(), Changed<MyButton>>,
    number_of_players_selected: Res<NumberOfPlayersSelected>,
) {
    if !number_of_players_selected.is_changed() && !changed_buttons.is_empty() {
        return;
    }

    for (mut border_color, button_number) in &mut buttons {
        border_color.0 = if button_number.0 == number_of_players_selected.0 {
            SELECTED_COLOR
        } else {
            UNSELECTED_COLOR
        }
    }
}

fn spawn_buttons(mut commands: Commands, max_number_of_players: Res<MaxNumberOfPlayers>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            (1..=max_number_of_players.0).for_each(|player_index| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                // width: Val::Px(150.0),
                                // height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(10.0)),

                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(UNSELECTED_COLOR),
                            ..default()
                        },
                        MyButton,
                        ButtonNumber(player_index),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{player_index} players"),
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ));
                    });
            });
        });
}

fn despawn_buttons(mut commands: Commands, buttons: Query<Entity, With<MyButton>>) {
    buttons
        .iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive());
}
