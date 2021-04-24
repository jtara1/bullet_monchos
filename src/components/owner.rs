pub enum Owner {
    Player,
    Enemy,
}

impl Clone for Owner {
    fn clone(&self) -> Self {
        match self {
            Owner::Player => Owner::Player,
            Owner::Enemy => Owner::Enemy,
            _ => panic!("Owner clone() needs to implement for the give type"),
        }
    }
}
