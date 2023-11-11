# light_enum


[![crates.io](https://img.shields.io/crates/v/light_enum?style=flat-square&logo=rust)](https://crates.io/crates/light_enum)
[![docs.rs](https://img.shields.io/badge/docs.rs-light_enum-blue?style=flat-square&logo=docs.rs)](https://docs.rs/light_enum)
[![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#license)

A crate providing a derive keyword to generate a light enum.

## Usage

```shell
cargo add light_enum
```

This crate provide two derive keywords:
- `LightEnum` will generate a new enum without the content of each field
- `Values` will generate a vector containing each field of the enum

See examples bellow to understand what they do.

## LightEnum

```rust
use light_enum::LightEnum;

#[derive(LightEnum)]
enum MyEnum {
    A(i32),
    B(i32),
    C(i32),
}

let heavy = MyEnum::A(0);
let light = heavy.to_light();
assert!(light == MyEnumLight::A);
```

`MyEnumLight` will be generated:

```rust
enum MyEnumLight {
    A,
    B,
    C,
}
```

## Values

```rust
use light_enum::Values;

#[derive(Values, PartialEq, Eq)]
enum Vals {
    A,
    B,
    C,
}

let values = Vals::VALUES;
assert!(values.len() == 3);
assert!(values.contains(&Vals::A));
assert!(values.contains(&Vals::B));
assert!(values.contains(&Vals::C));
```

Generated code:

```rust
impl Vals {
    const VALUES: [Vals; 3usize] = [Vals::A, Vals::B, Vals::C];
}
```

## Limitations

#### LightEnum
Having a field with no content will cause an error.

```rust
// this code will not compile
#[derive(LightEnum)]
enum MyEnum {
    A,
}
```
#### Values
Having a field with content will cause an error.

```rust
// this code will not compile
#[derive(Values)]
enum MyEnum {
    A(i32),
}
```

## Dev

The `cargo expand` utility is useful to see what is generated. It can be installed with `cargo install cargo-expand`.
