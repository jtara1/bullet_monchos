use bevy::prelude::*;
use crate::{TIME_STEP, WINDOW_DIMENSIONS};
use crate::components::Movement;

pub fn linear_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Movement, &mut Transform)>,
) {
    for (entity, movement, mut transform) in query.iter_mut() {
        let (direction, speed) = movement.get();
        let translation = &mut transform.translation;

        translation.x += direction.x * speed * TIME_STEP;
        translation.y += direction.y * speed * TIME_STEP;

        let vertical_limit = WINDOW_DIMENSIONS.height / 2.;
        if translation.y > vertical_limit || translation.y < -vertical_limit {
            commands.entity(entity).despawn();
        }
    }
}
