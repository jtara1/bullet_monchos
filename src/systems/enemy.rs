use bevy::prelude::*;
use rand::prelude::*;

use crate::{entities, WINDOW_DIMENSIONS};
use crate::entities::create_enemy;

pub struct SpawnerTimer(Timer);
impl Default for SpawnerTimer {
    fn default() -> Self {
        SpawnerTimer(Timer::from_seconds(2., true))
    }
}

pub fn enemy_spawner(
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    mut commands: Commands,
    enemy_material: Res<entities::EnemyMaterial>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let material = match enemy_material.0.clone() {
            Some(material) => material,
            None => return,
        } {
            let horizontal_limit = (WINDOW_DIMENSIONS.width / 2.) as i32;
            let vertical_limit = (WINDOW_DIMENSIONS.height / 2.) as i32;

            let x = rand::thread_rng().gen_range(-horizontal_limit..horizontal_limit);
            let y = rand::thread_rng().gen_range(0..vertical_limit);
            create_enemy(commands, material, Vec3::new(x as f32, y as f32, 1.));
        }
    }
}
