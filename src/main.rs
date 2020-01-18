
struct A {
    a_1: String,
    a_2: i16,
    a_3: i16,
}

impl A {
    fn calculate_a(&self) -> i16 {
        self.a_2 * self.a_3
    }
}

struct B {
    b_1: String,
    b_2: i16,
    b_3: i16,
}

impl B {
    fn calculate_b(&self) -> i16 {
        self.b_2 * self.b_3
    }
}

struct C {
    c_1: String,
    c_2: i16,
    c_3: i16,
}

impl C {
    fn calculate_c(&self) -> i16 {
        self.c_2 * self.c_3
    }
}

fn main() {
    let first = A {
        a_1: "first one".to_string(),
        a_2: 2,
        a_3: -1,
    };

    let second = B {
        b_1: "second one".to_string(),
        b_2: 4,
        b_3: -2,
    };

    let third = C {
        c_1: "third one".to_string(),
        c_2: 8,
        c_3: -4,
    };

    println!("{}: {}, {} = {}", first.a_1, first.a_2, first.a_3, first.calculate_a());
    println!("{}: {}, {} = {}", second.b_1, second.b_2, second.b_3, second.calculate_b());
    println!("{}: {}, {} = {}", third.c_1, third.c_2, third.c_3, third.calculate_c());
}
