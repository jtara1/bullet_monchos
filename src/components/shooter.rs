use crate::components::Bullet;


pub struct Shooter {
    bullet: Bullet
}
impl Shooter {
    pub fn new(bullet: Bullet) -> Self {
        Shooter { bullet }
    }

    pub fn bullet(&self) -> &Bullet {
        &self.bullet
    }
}

