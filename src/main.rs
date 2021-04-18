use bevy::{
    core::FixedTimestep,
    prelude::*,
};

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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(movement.system())
                .with_system(bullet_spawning.system())
                .with_system(bullet_movement.system())
        )
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
    speed: f32,
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

    let player_material = materials.add(asset_server.get_handle("sprites/playerShip1_blue.png").into());
    let bg_material = materials.add(asset_server.get_handle("sprites/backgrounds/alt/black.png").into());

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
                .insert(Bullet { speed: 1200.0 });
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
        if translation.y > PLAYER_CLAMP.y {
            commands.entity(entity).despawn();
        }
    }
}