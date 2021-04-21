use bevy::prelude::*;

use crate::components::{Shooter, Tag, Movement};
// use crate::entities::create_bullet;
use crate::systems::enemy::TwoSecondIntervalTimer;
use crate::Bullet;


pub fn interval_linear_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TwoSecondIntervalTimer>,
    mut query: Query<(&Shooter, &Transform, &Tag)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("shooting - timer finished");

        for (shooter, transform, tag) in query.iter() {
            println!("shooting - adding bullet entity {:?}", transform);

            let material = materials
                .add(asset_server.get_handle("sprites/laserRed16.png").into());
            let spawn_location = transform;
            // create_bullet(tag.owner(), material, spawn_location, commands);
            let bullet = shooter.bullet().clone();
            let movement = Movement::from_component(&bullet);

            commands
                .spawn_bundle(SpriteBundle {
                    material,
                    transform: *spawn_location,
                    ..Default::default()
                })
                .insert(bullet)
                .insert(movement)
                .insert(Tag::new(tag.owner().clone()));
        }
    }
}
