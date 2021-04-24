mod entities;
mod systems;
mod components;
mod traits;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::collide_aabb::collide};

use crate::entities::*;
use crate::systems::*;
use bevy::sprite::collide_aabb::Collision;
use std::borrow::Cow::Owned;
use crate::traits::Velocity;
use crate::components::{Shooter, Tag, Movement, Bullet, Player, Health, Drone, Collider};
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
        .add_system(bullet_spawning.system())
        .add_system(player_cloning.system())
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


struct PlayerBulletMaterial(pub Option<Handle<ColorMaterial>>);
struct BulletHitMaterial(pub Option<Handle<ColorMaterial>>);
struct PowerUpMaterial(pub Option<Handle<ColorMaterial>>);

struct ImpactEffect;

struct ImpactTimer(Timer);
impl Default for ImpactTimer {
    fn default() -> Self {
        ImpactTimer(Timer::from_seconds(0.3, true))
    }
}

pub enum Owner {
    Player,
    Enemy,
}
impl Clone for Owner {
    fn clone(&self) -> Self {
        match self {
            Owner::Player => Owner::Player,
            Owner::Enemy => Owner::Enemy,
            _ => panic!("Owner clone() needs to implement for the give type"),
        }
    }
}

struct DamageEvent {
    entity: Entity
}

struct Pickup;

pub struct Score(u32);
impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    audio: Res<Audio>,
) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(WINDOW_DIMENSIONS.width, WINDOW_DIMENSIONS.height);

    asset_server.load_folder("sounds").expect("sounds not found");
    asset_server.load_folder("sprites").expect("sprites not found");
    asset_server.load_folder("fonts").expect("fonts not found");

    let bg_material = materials.add(asset_server.get_handle("sprites/backgrounds/alt/black.png").into());

    let player_material = materials.add(asset_server.get_handle("sprites/playerShip1_blue.png").into());
    let player_bullet_material = materials.add(asset_server.get_handle("sprites/laserBlue16.png").into());

    let enemy_material = materials.add(asset_server.get_handle("sprites/enemyRed1.png").into());
    let enemy_bullet_material = materials.add(asset_server.get_handle("sprites/laserRed16.png").into());

    let bullet_hit_material = materials.add(asset_server.get_handle("sprites/laserOrange16.png").into());
    let powerup_material = materials.add(asset_server.get_handle("sprites/shield_bronze.png").into());

    // commands.insert_resource(PlayerBulletMaterial(Some(player_bullet_material)));
    commands.insert_resource(EnemyMaterial(Some(enemy_material)));
    commands.insert_resource(EnemyBulletMaterial(Some(enemy_bullet_material)));
    commands.insert_resource(BulletHitMaterial(Some(bullet_hit_material)));
    commands.insert_resource(PowerUpMaterial(Some(powerup_material)));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // spawn background sprite
    commands
        .spawn_bundle(SpriteBundle {
            material: bg_material.clone(),
            transform: Transform {
                scale: Vec3::new(6., 6., 1.),
                ..Default::default()
            },
            ..Default::default()
        });

    // spawn player ship sprite
    commands
        .spawn_bundle(SpriteBundle {
            material: player_material.clone(),
            transform: Transform {
                translation: Vec3::new(0., -WINDOW_DIMENSIONS.height / 4., 1.),
                scale: Vec3::new(0.8, 0.8, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player::new(400.))
        .insert(Collider::Player)
        .insert(Health::new(10, 10));

    // play music
    let music = asset_server.load("sounds/DST-RailJet-LongSeamlessLoop.mp3");
    audio.play(music);

    // spawn ui
    create_labels(commands, asset_server);
}

fn player_cloning(
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

fn bullet_spawning(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>
) {
    if let Ok((_player, transform)) = query.single_mut() {
        let sprite_file_path = "sprites/laserBlue16.png";

        let material = materials
            .add(asset_server.get_handle(sprite_file_path).into());

        if keyboard_input.just_pressed(KeyCode::Space) {
            let bullet = Bullet::new(
                Owner::Player,
                Vec3::new(0., 600., 0.),
                String::from(sprite_file_path),
            );

            commands
                .spawn_bundle(SpriteBundle {
                    material: material.clone(),
                    transform: *transform,
                    sprite: Sprite::new(Vec2::new(13.0, 54.0)),
                    ..Default::default()
                })
                .insert(bullet.clone())
                .insert(Movement::from_component(&bullet));

            let rand = rand::thread_rng().gen_range(0..4);
            let blast_sfx = match rand {
                0..=1 => asset_server.load("sounds/laser1.mp3"),
                2 => asset_server.load("sounds/laser4.mp3"),
                3 => asset_server.load("sounds/laser5.mp3"),
                _ => asset_server.load("sounds/laser1.mp3")
            };
            audio.play(blast_sfx)
        }
    }
}

fn bullet_collision(
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