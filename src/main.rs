mod entities;
mod systems;
mod components;
mod traits;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::collide_aabb::collide};

use crate::entities::*;
use crate::systems::*;
use crate::components::*;
use crate::traits::Velocity;

use bevy::sprite::collide_aabb::Collision;
use std::borrow::Cow::Owned;
use rand::Rng;

pub const TIME_STEP: f32 = 1.0 / 60.0;
const WINDOW_DIMENSIONS: WindowDimensions = WindowDimensions { width: 700., height: 1400. };
const PLAYER_DIMENSIONS: PlayerDimensions = PlayerDimensions { width: 99., height: 75. };
pub const PLAYER_CLAMP: PlayerPositionClamp = PlayerPositionClamp {
    x: WINDOW_DIMENSIONS.width * 0.5 - (PLAYER_DIMENSIONS.width / 4.),
    y: WINDOW_DIMENSIONS.height * 0.5 - (PLAYER_DIMENSIONS.height / 4.),
};


fn main() {
    std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap());

    App::build()
        .add_plugins(DefaultPlugins)
        // bg color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Score::default())
        .add_startup_system(setup.system())
        // ui
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(update_labels.system())
        // ship
        .insert_resource(ImpactTimer::default())
        .add_event::<DamageEvent>()
        // with fixed timestamp
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement.system())
                .with_system(bullet_collision.system())
                .with_system(player_pickup.system())
                .with_system(damage_receiver.system())
                .with_system(impact_effect_removal.system())
        )
        // enemy
        .insert_resource(IntervalTimer1::default())
        .add_system(enemy_spawner.system())
        .add_system(linear_movement.system())
        .add_system(interval_linear_shooting.system())
        .add_system(player_shooter.system())
        .add_system(drone_spawner.system())
        .run();
}

struct WindowDimensions {
    width: f32,
    height: f32,
}

struct PlayerDimensions {
    width: f32,
    height: f32,
}

pub struct PlayerPositionClamp {
    x: f32,
    y: f32,
}

/**
 * Resources
 */
pub struct ImpactTimer(Timer);

impl Default for ImpactTimer {
    fn default() -> Self {
        ImpactTimer(Timer::from_seconds(0.3, true))
    }
}

/**
 * Events
 */
pub struct DamageEvent {
    entity: Entity
}
