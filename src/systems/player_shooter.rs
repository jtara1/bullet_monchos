use bevy::prelude::*;
use rand::Rng;

use crate::components::{Player, Movement, Bullet, Owner};

pub fn player_shooter(
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
