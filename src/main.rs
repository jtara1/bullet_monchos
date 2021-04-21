mod entities;
mod systems;
mod components;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::collide_aabb::collide};

use crate::entities::*;
use crate::systems::*;
use bevy::sprite::collide_aabb::Collision;
use std::borrow::Cow::Owned;

const TIME_STEP: f32 = 1.0 / 60.0;
const WINDOW_DIMENSIONS: WindowDimensions = WindowDimensions { width: 700., height: 1400. };
const PLAYER_DIMENSIONS: PlayerDimensions = PlayerDimensions { width: 99., height: 75. };
const PLAYER_CLAMP: PlayerPositionClamp = PlayerPositionClamp {
    x: WINDOW_DIMENSIONS.width * 0.5 - (PLAYER_DIMENSIONS.width / 4.),
    y: WINDOW_DIMENSIONS.height * 0.5 - (PLAYER_DIMENSIONS.height / 4.),
};


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // bg color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
                .with_system(movement.system())
                .with_system(bullet_spawning.system())
                .with_system(bullet_movement.system())
                .with_system(bullet_collision.system())
                .with_system(damage_receiver.system())
                .with_system(impact_effect_removal.system())
                .with_system(player_cloning.system())
        )
        // enemy
        .insert_resource(TwoSecondIntervalTimer::default())
        .add_system(enemy_spawner.system())
        .add_system(linear_movement.system())
        .add_system(interval_linear_shooting.system())
        // .add_system((interval_linear_shooting<Owner::Enemy>).system())
        // .add_system(enemy_shooting.system())
        .run();
}

struct WindowDimensions {
    width: f32,
    height: f32,
}

struct Player {
    speed: f32,
}

struct PlayerDimensions {
    width: f32,
    height: f32,
}

struct PlayerPositionClamp {
    x: f32,
    y: f32,
}

struct Bullet {
    owner: Owner,
    speed: f32,
}

pub struct Health {
    max: i32,
    current: i32,
}

struct Drone;

struct PlayerBulletMaterial(pub Option<Handle<ColorMaterial>>);
struct BulletHitMaterial(pub Option<Handle<ColorMaterial>>);

struct ImpactEffect;

struct ImpactTimer(Timer);
impl Default for ImpactTimer {
    fn default() -> Self {
        ImpactTimer(Timer::from_seconds(0.3, true))
    }
}

enum Collider {
    Bullet,
    Player,
    Enemy,
    TestWall,
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(WINDOW_DIMENSIONS.width, WINDOW_DIMENSIONS.height);

    asset_server.load_folder("sprites/backgrounds/alt").expect("sprite bgs not found");
    asset_server.load_folder("sprites").expect("sprites not found");
    asset_server.load_folder("fonts").expect("fonts not found");

    let bg_material = materials.add(asset_server.get_handle("sprites/backgrounds/alt/black.png").into());

    let player_material = materials.add(asset_server.get_handle("sprites/playerShip1_blue.png").into());
    let player_bullet_material = materials.add(asset_server.get_handle("sprites/laserBlue16.png").into());

    let enemy_material = materials.add(asset_server.get_handle("sprites/enemyRed1.png").into());
    let enemy_bullet_material = materials.add(asset_server.get_handle("sprites/laserRed16.png").into());

    let bullet_hit_material = materials.add(asset_server.get_handle("sprites/laserOrange16.png").into());

    commands.insert_resource(PlayerBulletMaterial(Some(player_bullet_material)));
    commands.insert_resource(EnemyMaterial(Some(enemy_material)));
    commands.insert_resource(EnemyBulletMaterial(Some(enemy_bullet_material)));
    commands.insert_resource(BulletHitMaterial(Some(bullet_hit_material)));

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
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(0.8, 0.8, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 500. })
        .insert(Collider::Player)
        .insert(Health {max: 30, current: 30});

    // spawn ui
    create_labels(commands, asset_server);
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let mut direction: Vec3 = Vec3::new(0.0,0.0,0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction != Vec3::new(0.0,0.0,0.0) {
            direction = direction.normalize();
        }

        let translation = &mut transform.translation;
        // move the player
        translation.x += direction.x * player.speed * TIME_STEP;
        translation.y += direction.y * player.speed * TIME_STEP;

        // clamp
        translation.x = translation.x.min(PLAYER_CLAMP.x).max(-PLAYER_CLAMP.x);
        translation.y = translation.y.min(PLAYER_CLAMP.y).max(-PLAYER_CLAMP.y);
    }
}

fn player_cloning(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_player, transform) in query.iter_mut() {
        let material = materials
            .add(asset_server.get_handle("sprites/playerShip1_blue.png").into());
        if keyboard_input.just_pressed(KeyCode::F) {
            let multiplier = if rand::random() { -1. } else { 1. };
            let displacement: Vec3 = Vec3::new(multiplier * 35., 0., 0.);

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
                .insert(Health {max: 5, current: 5});
        }
    }
}

fn bullet_spawning(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok((_player, transform)) = query.single_mut() {
        let material = materials
            .add(asset_server.get_handle("sprites/laserBlue16.png").into());

        if keyboard_input.just_pressed(KeyCode::Space) {
            commands
                .spawn_bundle(SpriteBundle {
                    material: material.clone(),
                    transform: *transform,
                    sprite: Sprite::new(Vec2::new(13.0, 54.0)),
                    ..Default::default()
                })
                .insert(Bullet { owner: Owner::Player, speed: 600.0 });
        }
    }
}

fn bullet_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Bullet, &mut Transform)>,
) {
    for (entity, bullet, mut transform) in query.iter_mut() {
        let direction: Vec3 = Vec3::new(0.0,1.0,0.0);

        let translation = &mut transform.translation;
        // move the bullet vertically
        translation.x += direction.x * bullet.speed * TIME_STEP;
        translation.y += direction.y * bullet.speed * TIME_STEP;
        if translation.y > PLAYER_CLAMP.y || translation.y < -PLAYER_CLAMP.y {
            commands.entity(entity).despawn();
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
            let collision: Option<Collision> = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                sprite.size,
            );

            if let Some(_) = collision {
                let mut should_damage = false;

                if let Owner::Player = bullet.owner {
                    if let Collider::Enemy = collider {
                        should_damage = true;
                    }
                }

                if let Owner::Enemy = bullet.owner {
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
    mut damage_reader: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>
) {
    for event in damage_reader.iter() {
        if let Ok(mut health) = health_query.get_mut(event.entity) {
            health.current = health.current - 1;
            //println!("damage_receiver(): Health is {} for entity {:?}", health.current, event.entity);
            if health.current <= 0 {
                commands.entity(event.entity).despawn();
            }
        } else {
            println!("Does not have health component");
        }
    }
}