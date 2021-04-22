use bevy::prelude::*;

use crate::components::{Shooter, Tag, Movement};
// use crate::entities::create_bullet;
use crate::systems::enemy::TwoSecondIntervalTimer;
use crate::{Bullet, Owner};


pub fn interval_linear_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TwoSecondIntervalTimer>,
    mut query: Query<(&Shooter, &Transform, &Tag)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // audio playing
        let sfx = asset_server.load("sounds/tir.mp3");
        audio.play(sfx);

        for (shooter, transform, tag) in query.iter() {
            let spawn_location = transform;
            let bullet = shooter.bullet().clone();
            let movement = Movement::from_component(&bullet);

            let (_, _, material_file_path) = bullet.get();
            let material = materials
                .add(asset_server.get_handle(material_file_path.as_str()).into());

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
