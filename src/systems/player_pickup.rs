use bevy::prelude::*;
use bevy::sprite::collide_aabb::{Collision, collide};

use crate::components::{Pickup, Player, Health};

pub(crate) fn player_pickup(
    mut commands: Commands,
    mut pickup_query: Query<(Entity, &Pickup, &Transform, &Sprite)>,
    mut player_query: Query<(&Player, &Transform, &Sprite, &mut Health)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (entity, _pickup, pickup_transform, pickup_sprite) in pickup_query.iter_mut() {
        let pickup_size = pickup_sprite.size;
        for (_player, player_transform, player_sprite, mut health) in player_query.iter_mut() {
            let mut player_size = player_sprite.size;

            let collision: Option<Collision> = collide(
                pickup_transform.translation,
                pickup_size,
                player_transform.translation,
                player_size,
            );

            if let Some(_) = collision {
                commands.entity(entity).despawn();
                let sfx = asset_server.load("sounds/sfx_shieldUp.mp3");
                audio.play(sfx);
                health.add(5);
            }
        }
    }
}
