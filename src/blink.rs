use bevy::prelude::*;

use super::game_state::AppState;

pub(crate) struct BlinkPlugin;

const BLINK_DURATION: f32 = 0.1;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlinkTimer(Timer::from_seconds(
            BLINK_DURATION,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (blink_tick,).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub(crate) struct Blinking;

#[derive(Resource)]
struct BlinkTimer(Timer);

fn blink_tick(
    time: Res<Time>,
    mut timer: ResMut<BlinkTimer>,
    mut blinking: Query<&mut Sprite, With<Blinking>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut sprite in blinking.iter_mut() {
            let current_alpha = sprite.color.alpha();
            sprite.color.set_alpha(1.0 - current_alpha);
        }
    }
}
