#![allow(dead_code)]

use light_enum::LightEnum;

#[derive(LightEnum)]
enum MyEnum {
    A(i32, i32),
    B(i32),
    C,
}

fn main() {
    let heavy = MyEnum::A(0, 0);
    let light = heavy.to_light();

    assert!(light == MyEnumLight::A);
}
