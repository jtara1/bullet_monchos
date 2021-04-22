use bevy::math::Vec3;

use crate::Owner;
use crate::Velocity;

pub struct Bullet {
    owner: Owner,
    velocity: Vec3,
    sprite_file_path: String,
}

impl Bullet {
    pub fn new(owner: Owner, velocity: Vec3, sprite_file_path: String) -> Self {
        Bullet { owner, velocity, sprite_file_path }
    }

    pub fn get(&self) -> (&Owner, &Vec3, &String) {
        (&self.owner, &self.velocity, &self.sprite_file_path)
    }
}

impl Clone for Bullet {
    fn clone(&self) -> Self {
        Bullet::new(self.owner.clone(), self.velocity, self.sprite_file_path.clone())
    }
}

impl Velocity for Bullet {
    fn get_velocity(&self) -> Vec3 {
        self.velocity
    }
}

