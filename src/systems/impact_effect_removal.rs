use bevy::prelude::*;

use crate::{ImpactTimer};
use crate::components::{ImpactEffect};


pub fn impact_effect_removal(
    time: Res<Time>,
    mut timer: ResMut<ImpactTimer>,
    mut commands: Commands,
    impact_effect_query: Query<(Entity, &ImpactEffect)>
) {
    for (entity, _impact_effect) in impact_effect_query.iter() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
