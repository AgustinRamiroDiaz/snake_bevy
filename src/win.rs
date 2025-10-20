use bevy::prelude::*;

use crate::game_state::AppState;
use crate::snake::{MyColor, Snake};

const LENGTH_TO_WIN: usize = 10;
const HOLD_TIME_TO_WIN: f32 = 10.0;

pub(crate) struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinnerHoldTimer(Timer::from_seconds(
            HOLD_TIME_TO_WIN,
            TimerMode::Repeating,
        )))
        .insert_resource(CurrentFirst(None))
        .add_event::<Won>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (set_first, update_timer_text, win).run_if(in_state(AppState::InGame)),
        );
    }
}

/// Defines the time that the winner must hold the position to win
#[derive(Resource)]
struct WinnerHoldTimer(Timer);

#[derive(Resource)]
struct CurrentFirst(Option<(String, Color)>);

// TODO: should this event get injected from main into this plugin?
#[derive(Event)]
pub(crate) struct Won(pub(crate) String);

fn set_first(
    snakes: Query<(&Snake, &MyColor), Changed<Snake>>,
    mut current_winner: ResMut<CurrentFirst>,
) {
    let mut snakes = Vec::from_iter(snakes.iter());
    snakes.sort_by_key(|(snake, _)| snake.segments.len() as i8);

    let first = snakes.pop();
    let second = snakes.pop();

    if let (Some((first_snake, first_color)), Some((second_snake, _))) = (first, second) {
        // TODO: remove clones
        if first_snake.segments.len() > second_snake.segments.len()
            && first_snake.segments.len() >= LENGTH_TO_WIN
        {
            current_winner.0 = Some((first_snake.name.clone(), first_color.0.clone()));
        } else {
            current_winner.0 = None;
        }
    }
}

fn win(
    mut won: EventWriter<Won>,
    current_winner: Res<CurrentFirst>,
    mut timer: ResMut<WinnerHoldTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if let Some((name, _)) = &current_winner.0 {
        if timer.0.just_finished() {
            won.send(Won(name.clone()));
        }
    } else {
        timer.0.reset();
    }
}

#[derive(Component)]
struct TimerText;

fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        TimerText,
    ));
}

fn update_timer_text(
    mut query: Query<(&mut Text, &TimerText)>,
    timer: Res<WinnerHoldTimer>,
    current_winner: Res<CurrentFirst>,
) {
    if let Some((name, _color)) = &current_winner.0 {
        for (mut text, _) in query.iter_mut() {
            *text = Text::new(format!(
                "{} wins in {:.2}",
                name,
                timer.0.duration().as_secs_f32() - timer.0.elapsed_secs()
            ));
        }
    } else {
        for (mut text, _) in query.iter_mut() {
            *text = Text::new("");
        }
    }
}
