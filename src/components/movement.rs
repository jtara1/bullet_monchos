use bevy::prelude::*;
use crate::traits::Velocity;

pub struct Movement {
    direction: Vec3,
    speed: f32,
}

impl Movement {
    pub fn new(direction: Vec3, speed: f32) -> Movement {
        Movement { direction, speed }
    }

    pub fn get(&self) -> (Vec3, f32) {
        (self.direction, self.speed)
    }

    pub fn from_component<T: Velocity>(component: &T) -> Self {
        Movement { direction: component.get_velocity(), speed: 1.0 }
    }
}
