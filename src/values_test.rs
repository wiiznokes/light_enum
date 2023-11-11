#![allow(dead_code)]

use light_enum::Values;

#[derive(Values, PartialEq, Eq)]
enum Vals {
    A,
    B,
    C,
}

#[test]
fn values() {
    let values = Vals::VALUES;

    assert!(values.len() == 3);
    assert!(values.contains(&Vals::A));
    assert!(values.contains(&Vals::B));
    assert!(values.contains(&Vals::C));
}
