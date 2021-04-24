use bevy::prelude::*;
use rand::Rng;

use crate::components::{Score, Pickup, Health};
use crate::{DamageEvent};

pub fn damage_receiver(
    mut commands: Commands,
    mut damage_readers: EventReader<DamageEvent>,
    mut health_query: Query<(&mut Health, &Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>
) {
    for event in damage_readers.iter() {
        if let Ok((mut health, transform)) = health_query.get_mut(event.entity) {
            health.add(-1);
            if *health.current() <= 0 {
                // spawning a pickup
                let rand = rand::thread_rng().gen_range(0..10);
                if rand <= 2 {
                    let material = materials
                        .add(asset_server.get_handle("sprites/shield_bronze.png").into());

                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: *transform,
                            material: material.clone(),
                            ..Default::default()
                        })
                        .insert(Pickup);
                }

                // remove this entity
                commands.entity(event.entity).despawn();

                // play sound
                let sfx = asset_server.load("sounds/Explosion.mp3");
                audio.play(sfx);

                // incr score
                score.0 = score.0 + 1;
            }
        } else {
            println!("Query for Health, Transform failed");
        }
    }
}