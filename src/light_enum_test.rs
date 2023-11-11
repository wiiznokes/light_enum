#![allow(dead_code)]

use light_enum::LightEnum;

#[derive(LightEnum)]
enum MyEnum {
    A(i32),
    B(i32),
    C(i32),
}

#[test]
fn light_enum() {
    let heavy = MyEnum::A(0);
    let light = heavy.to_light();

    assert!(light == MyEnumLight::A);
}
