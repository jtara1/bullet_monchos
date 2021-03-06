use bevy::prelude::*;
use rand::prelude::*;

use crate::{WINDOW_DIMENSIONS};
use crate::entities::create_enemy;

pub struct IntervalTimer1(pub(crate) Timer);
impl Default for IntervalTimer1 {
    fn default() -> Self {
        IntervalTimer1(Timer::from_seconds(1.5, true))
    }
}

pub fn enemy_spawner(
    time: Res<Time>,
    mut timer: ResMut<IntervalTimer1>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let material = materials
            .add(asset_server.get_handle("sprites/enemyRed1.png").into());

        let horizontal_limit = ((WINDOW_DIMENSIONS.width / 2.) - 40.) as i32;
        let vertical_limit = ((WINDOW_DIMENSIONS.height / 2.) - 150.) as i32;

        let x = rand::thread_rng().gen_range(-horizontal_limit..horizontal_limit);
        let y = rand::thread_rng().gen_range(0..vertical_limit);
        create_enemy(commands, material, Vec3::new(x as f32, y as f32, 1.));
    }
}
