use bevy::prelude::*;

pub struct Enemy { speed: f32, }
impl Default for Enemy {
    fn default() -> Self {
        Enemy { speed: 300. }
    }
}

pub fn create_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
) {
    let material = materials.add(asset_server.get_handle("sprites/enemyRed1.png").into());

    commands
        .spawn_bundle(SpriteBundle {
            material,
            transform: Transform {
                translation: Vec3::new(0., 0., 3.),
                scale: Vec3::new(0.8, 0.8, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy::default());

    println!("spawned");
}
