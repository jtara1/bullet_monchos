use bevy::prelude::*;
use bevy::sprite::collide_aabb::{Collision, collide};

use crate::components::{Owner, ImpactEffect, Collider, Bullet};
use crate::{DamageEvent};

pub fn bullet_collision(
    mut commands: Commands,
    mut damage_writer: EventWriter<DamageEvent>,
    mut bullet_query: Query<(Entity, &mut Bullet, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bullet_entity, mut bullet, bullet_transform, sprite) in bullet_query.iter_mut() {
        let bullet_size = sprite.size;

        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let mut collision_size = Vec2::new(0.,0.);

            match collider {
                Collider::Enemy => collision_size = sprite.size - Vec2::new(40.0, 60.0),
                Collider::Player => collision_size = sprite.size - Vec2::new(66.0, 60.0),
                _ => return,
            }

            let collision: Option<Collision> = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                collision_size,
            );

            if let Some(_) = collision {
                let mut should_damage = false;
                let (bullet_owner, _, _) = bullet.get();

                if let Owner::Player = bullet_owner {
                    if let Collider::Enemy = collider {
                        should_damage = true;
                    }
                }

                if let Owner::Enemy = bullet_owner {
                    if let Collider::Player = collider {
                        should_damage = true;
                    }
                }

                if should_damage {
                    damage_writer.send(DamageEvent { entity: collider_entity });
                    commands.entity(bullet_entity).despawn();

                    let material = materials
                        .add(asset_server.get_handle("sprites/laserOrange16.png").into());

                    commands
                        .spawn_bundle(SpriteBundle {
                            material: material.clone(),
                            transform: *bullet_transform,
                            sprite: Sprite::new(Vec2::new(37.0, 37.0)),
                            ..Default::default()
                        })
                        .insert(ImpactEffect);
                }
            }
        }
    }
}
