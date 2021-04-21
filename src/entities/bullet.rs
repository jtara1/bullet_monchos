use bevy::prelude::*;

use crate::{Bullet, Owner};
use crate::components::{Movement, Tag};


// pub fn create_bullet(
//     owner: &Owner,
//     material: Handle<ColorMaterial>,
//     spawn_location: &Transform,
//     mut commands: Commands,
// ) {
//     commands
//         .spawn_bundle(SpriteBundle {
//             material,
//             transform: *spawn_location,
//             ..Default::default()
//         })
//         .insert(Bullet { owner: owner.clone(), speed: -100. })
//         .insert(Movement::new(Vec3::new(0., 1., 0.), 600.))
//         .insert(Tag::new(owner.clone()));
// }