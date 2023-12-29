use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    SpawnDespawnEntities,
    Last,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::SpawnDespawnEntities,
                // Flush commands (i.e. `apply_deferred` runs)
                InGameSet::Last,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::SpawnDespawnEntities)
                .before(InGameSet::Last),
        );
    }
}
