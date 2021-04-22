use bevy::prelude::*;
use crate::components::{Movement, Tag, Shooter, Bullet};

use crate::{Collider, Health, Owner};

pub struct Enemy;
pub struct EnemyMaterial(pub Option<Handle<ColorMaterial>>);
pub struct EnemyBulletMaterial(pub Option<Handle<ColorMaterial>>);

pub fn create_enemy(
    mut commands: Commands,
    material: Handle<ColorMaterial>,
    translation: Vec3,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material,
            transform: Transform {
                translation,
                scale: Vec3::new(0.8, 0.8, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Movement::new(Vec3::new(0., -1., 0.), 0.))
        .insert(Collider::Enemy)
        .insert(Health { max: 10, current: 10 })
        .insert(Tag::new(Owner::Enemy))
        .insert(Shooter::new(Bullet::new(
            Owner::Enemy,
            Vec3::new(0., -600., 0.),
            String::from("sprites/laserRed16.png"),
        )));
}
