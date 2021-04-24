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
        // player and input
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement.system())
                //.with_system(bullet_spawning.system())
                // .with_system(bullet_movement.system())
                .with_system(bullet_collision.system())
                .with_system(player_pickup.system())
                .with_system(damage_receiver.system())
                .with_system(impact_effect_removal.system())
                //.with_system(player_cloning.system())
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

struct ImpactTimer(Timer);
impl Default for ImpactTimer {
    fn default() -> Self {
        ImpactTimer(Timer::from_seconds(0.3, true))
    }
}

pub struct DamageEvent {
    entity: Entity
}

fn player_pickup(
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

fn impact_effect_removal(
    time: Res<Time>,
    mut timer: ResMut<ImpactTimer>,
    mut commands: Commands,
    impact_effect_query: Query<(Entity, &ImpactEffect)>
) {
    for (entity, _impact_effect) in impact_effect_query.iter() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn damage_receiver(
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