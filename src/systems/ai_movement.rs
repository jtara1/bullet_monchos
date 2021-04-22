use bevy::prelude::*;
use crate::{TIME_STEP, WINDOW_DIMENSIONS};
use crate::components::Movement;

pub fn linear_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Movement, &mut Transform)>,
) {
    query.for_each_mut(|(entity, movement, mut transform)| {
        let (direction, speed) = movement.get();
        let translation = &mut transform.translation;

        translation.x += direction.x * speed * TIME_STEP;
        translation.y += direction.y * speed * TIME_STEP;

        let vertical_limit = WINDOW_DIMENSIONS.height / 2.;
        let horizontal_limit = WINDOW_DIMENSIONS.width / 2.;

        let out_of_bounds1 = translation.x > horizontal_limit || translation.x < -horizontal_limit;
        let out_of_bounds2 = translation.y > vertical_limit || translation.y < -vertical_limit;
        if out_of_bounds1 || out_of_bounds2 {
            commands.entity(entity).despawn();
        }
    });
}
