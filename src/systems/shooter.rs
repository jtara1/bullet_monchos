use bevy::prelude::*;

use crate::components::{Shooter, Tag};
use crate::entities::create_bullet;
use crate::systems::enemy::TwoSecondIntervalTimer;


pub fn interval_linear_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TwoSecondIntervalTimer>,
    mut query: Query<(&Shooter, &Transform, &Tag)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_, transform, tag) in query.iter() {
        if timer.0.tick(time.delta()).just_finished() {
            let material = materials
                .add(asset_server.get_handle("sprites/laserRed16.png").into());
            let spawn_location = transform;
            create_bullet(tag.owner(), material, spawn_location, *commands);
        }
    }
}
