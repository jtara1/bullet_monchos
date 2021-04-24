use bevy::prelude::*;

use crate::components::{Shooter, Tag, Movement};
use crate::systems::enemy::IntervalTimer1;
use crate::{Owner};
use bevy::tasks::TaskPool;


pub fn interval_linear_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<IntervalTimer1>,
    mut query: Query<(&Shooter, &Transform, &Tag)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut fired = false;

        // let pool = TaskPool::new();
        query.for_each(|(shooter, transform, tag)| {
        // query.par_for_each(&pool, 8, |(shooter, transform, tag)| {
            fired = true;
            let bullet_transform = transform;
            let bullet = shooter.bullet().clone();
            let movement = Movement::from_component(&bullet);

            let (_, _, material_file_path) = bullet.get();
            let material = materials
                .add(asset_server.get_handle(material_file_path.as_str()).into());

            commands
                .spawn_bundle(SpriteBundle {
                    material,
                    transform: *bullet_transform,
                    ..Default::default()
                })
                .insert(bullet)
                .insert(movement)
                .insert(Tag::new(tag.owner().clone()));

        });

        // audio playing
        if fired {
            let sfx = asset_server.load("sounds/tir.mp3");
            audio.play(sfx);
        }
    }
}
