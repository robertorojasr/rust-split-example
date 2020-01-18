/// Single executable, single module

mod module_abc;

use crate::module_abc::*;

fn main() {
    // we could also call `A` as `crate::module_abc::A` but that would be silly
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
