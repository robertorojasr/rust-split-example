/// All of this is now in another module so we have to make them public (pub)

pub struct A {
    pub a_1: String,
    pub a_2: i16,
    pub a_3: i16,
}

// no `pub` in here, is implied from the struct
impl A {
    pub fn calculate_a(&self) -> i16 {
        self.a_2 * self.a_3
    }
}

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

