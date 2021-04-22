use bevy::prelude::*;
use crate::components::{Movement, Tag, Shooter};

use crate::{Collider, Health, Owner, Bullet};

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
        .insert(Shooter::new(Bullet {
            owner: Owner::Enemy,
            velocity: Vec3::new(0., -600., 0.),
            sprite_file_path: String::from("sprites/laserRed16.png"),
        }));
}
