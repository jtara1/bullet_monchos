use bevy::prelude::*;
use crate::components::{Player, Health, Collider, Bullet, Shooter, Owner, Drone, Tag};
use crate::{DamageEvent, PLAYER_DIMENSIONS};

pub fn drone_spawner(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut damage_writer: EventWriter<DamageEvent>,
    mut query: Query<(&Player, &mut Transform, &mut Health, Entity)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_player, transform, mut _health, entity) in query.iter_mut() {
        let material = materials
            .add(asset_server.get_handle("sprites/playerShip1_blue.png").into());
        if keyboard_input.just_pressed(KeyCode::F) {
            let displacement: Vec3 = Vec3::new(0., PLAYER_DIMENSIONS.height, 0.);

            commands
                .spawn_bundle(SpriteBundle {
                    material: material.clone(),
                    transform: Transform {
                        translation: transform.translation + displacement,
                        scale: Vec3::new(0.5, 0.5, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Drone)
                .insert(Collider::Player)
                .insert(Health::new(2, 2))
                .insert(Shooter::new(Bullet::new(
                    Owner::Player,
                    Vec3::new(0., 600., 0.),
                    String::from("sprites/laserBlue16.png"),
                )))
                .insert(Tag::new(Owner::Player));

            damage_writer.send(DamageEvent { entity });
        }
    }
}
