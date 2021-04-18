use bevy::prelude::*;

use crate::entities;

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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // entities::create_enemy(commands, asset_server, materials, Vec3::new(0., 0., 2.));
        let material = materials.add(asset_server.get_handle("sprites/enemyRed1.png").into());

        commands
            .spawn_bundle(SpriteBundle {
                material,
                transform: Transform {
                    // translation: Vec3::new(0., 0., 1.),
                    // scale: Vec3::new(0.8, 0.8, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(entities::Enemy::default());
    }
}
