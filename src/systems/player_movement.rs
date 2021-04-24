use bevy::prelude::*;

use crate::components::{Player};
use crate::{TIME_STEP, PLAYER_CLAMP};


pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let mut direction: Vec3 = Vec3::new(0.0,0.0,0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::new(0.0,0.0,0.0) {
            direction = direction.normalize();
        }

        let translation = &mut transform.translation;
        // move the player
        translation.x += direction.x * player.speed() * TIME_STEP;
        translation.y += direction.y * player.speed() * TIME_STEP;

        // clamp
        translation.x = translation.x.min(PLAYER_CLAMP.x).max(-PLAYER_CLAMP.x);
        translation.y = translation.y.min(PLAYER_CLAMP.y).max(-PLAYER_CLAMP.y);
    }
}
