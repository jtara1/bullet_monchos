mod greet;

use bevy::{
    core::FixedTimestep,
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;

use crate::greet::*;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.0, 0.9, 0.0)))
    .add_startup_system(setup.system())
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(movement.system()),
    )
    .run();
}

struct Player {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    asset_server.load_folder("sprites/backgrounds/alt");
    asset_server.load_folder("sprites");

    let texture_handle = asset_server.get_handle("sprites/playerShip1_blue.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50., 50.), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            ..Default::default()
        })
        .insert(Player { speed: 300. });
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

        let translation = &mut transform.translation;
        // move the player horizontally
        translation.x += direction.x * player.speed * TIME_STEP;
        translation.y += direction.y * player.speed * TIME_STEP;
    }
}