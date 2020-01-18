/// All of this is now in another module now

pub struct C {
    pub c_1: String,
    pub c_2: i16,
    pub c_3: i16,
}

impl C {
    pub fn calculate_c(&self) -> i16 {
        self.c_2 * self.c_3
    }
}

