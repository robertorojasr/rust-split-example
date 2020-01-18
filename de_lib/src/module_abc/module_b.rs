/// All of this is now in another module now

pub struct B {
    pub b_1: String,
    pub b_2: i16,
    pub b_3: i16,
}

impl B {
    pub fn calculate_b(&self) -> i16 {
        self.b_2 * self.b_3
    }
}
