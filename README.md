# light_enum


[![crates.io](https://img.shields.io/crates/v/auto_enums?style=flat-square&logo=rust)](https://crates.io/crates/auto_enums)
[![docs.rs](https://img.shields.io/badge/docs.rs-auto__enums-blue?style=flat-square&logo=docs.rs)](https://docs.rs/auto_enums)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)

A crate providing a derive keyword to generate a light enum.

## Usage

```shell
cargo add light_enum
```

Add `#[derive(LightEnum)]` on top of your `enum`.

## Example

```rust
use light_enum::LightEnum;

#[derive(LightEnum)]
enum MyEnum {
    A(i32),
    B(i32),
    C(i32),
}

pub fn main() {
    let heavy = MyEnum::A(0);
    let light = heavy.to_light();

    assert!(light == MyEnumLight::A);
}
```

`MyEnumLight` will be generated:

```rust
enum MyEnumLight {
    A,
    B,
    C,
}
```

## Note

Having a field without content in your enum will not compile. The macro doesn't support it.

```rust
// this code will not compile
#[derive(LightEnum)]
enum MyEnum {
    A,
}
```

## Dev

The `cargo expand` utility is useful to see what is generated.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.