
extern mod rustcheck;

use rustcheck::{gen_int,for_all};



fn prop_even(x : int) -> bool {
    return x % 2 == 0;
}

fn gen_even() -> int {
    let i : int = rustcheck::gen_int();

    if i % 2 == 0 {
        i
    }
    else {
        i + 1
    }
}

#[test]
fn main() {
    let ints = || rustcheck::gen_int();
    let evens = || gen_even();

    assert!(!rustcheck::for_all(prop_even, &[ints]));
    assert!(rustcheck::for_all(prop_even, &[evens]));
}
