pub struct Health {
    max: i32,
    current: i32,
}

impl Health {
    pub fn new(max: i32, current: i32) -> Self {
        Health { max, current }
    }

    pub fn current(&self) -> &i32 {
        &self.current
    }

    pub fn add(&mut self, amount: i32) {
        self.current += amount;
    }
}
