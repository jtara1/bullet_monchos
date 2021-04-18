use bevy::prelude::*;

pub struct Enemy {
    speed: f32,
}
impl Default for Enemy {
    fn default() -> Self {
        Enemy { speed: 300. }
    }
}

pub struct EnemyMaterial(pub Option<Handle<ColorMaterial>>);

pub fn create_enemy(
    mut commands: Commands,
    material: Handle<ColorMaterial>,
    translation: Vec3,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material,
            transform: Transform {
                translation,
                scale: Vec3::new(0.8, 0.8, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy::default());
}
