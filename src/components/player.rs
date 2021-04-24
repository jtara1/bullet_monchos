pub struct Player {
    speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Player { speed }
    }

    pub fn speed(&self) -> &f32 {
        &self.speed
    }
}