mod greet;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const TOP_RESTRICTION: f32 = 500.0;
const LEFT_RESTRICTION: f32 = -600.0;
const RIGHT_RESTRICTION: f32 = 600.0;

use crate::greet::*;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    // .add_plugin(greet::GreetPlugin)
    .insert_resource(ClearColor(Color::rgb(0.0, 0.9, 0.0)))
    .add_startup_system(setup.system())
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(movement.system())
            .with_system(bullet_spawning.system())
            .with_system(bullet_movement.system()),
    )
    .run();
}

struct Player {
    speed: f32,
}

struct Bullet {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Player { speed: 300.0});
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
        translation.x = translation.x.min(RIGHT_RESTRICTION).max(LEFT_RESTRICTION);
        translation.y += direction.y * player.speed * TIME_STEP;
    }
}

fn bullet_spawning(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    if let Ok((_player, transform)) = query.single_mut() {
        let spawn_location = transform;
        if keyboard_input.pressed(KeyCode::Space) {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
                    transform: *spawn_location,
                    sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                    ..Default::default()
                })
                .insert(Bullet { speed: 1200.0});
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
        if translation.y > TOP_RESTRICTION {
            commands.entity(entity).despawn();
        }
    }
}