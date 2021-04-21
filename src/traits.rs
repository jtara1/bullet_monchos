use bevy::math::Vec3;


pub trait Velocity {
    fn get_velocity(&self) -> Vec3;
}