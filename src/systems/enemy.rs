use bevy::prelude::*;
use rand::prelude::*;

use crate::{Bullet, Owner, WINDOW_DIMENSIONS, entities::{self, Enemy}};
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

pub fn enemy_shooting (
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            let spawn_location = transform;

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
                    transform: *spawn_location,
                    sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                    ..Default::default()
                })
                .insert(Bullet { owner: Owner::Enemy, speed: -600.0 });
        }
    }
}
