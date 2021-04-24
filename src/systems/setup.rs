use bevy::prelude::*;

use crate::components::{Player, Collider, Health};
use crate::entities::{create_labels};
use crate::{WINDOW_DIMENSIONS};

struct PlayerBulletMaterial(pub Option<Handle<ColorMaterial>>);
struct BulletHitMaterial(pub Option<Handle<ColorMaterial>>);
struct PowerUpMaterial(pub Option<Handle<ColorMaterial>>);
struct EnemyMaterial(pub Option<Handle<ColorMaterial>>);
struct EnemyBulletMaterial(pub Option<Handle<ColorMaterial>>);

pub(crate) fn setup(
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
