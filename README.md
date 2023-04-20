[![Build](https://github.com/doctorn/obake/actions/workflows/obake.yml/badge.svg)](https://github.com/doctorn/obake/actions/workflows/obake.yml)
[![Issues][issues-shield]][issues-url]
[![crates.io][crates-io-shield]][crates-io-url]
[![License][license-shield]][license-url]

<br />
<p align="center">
  <h1 align="center">お化け</h1>
  <h3 align="center">Obake</h3>

  <p align="center">
    Versioned data-structures for Rust.
    <br />
    <a href="https://docs.rs/obake/"><strong>View on docs.rs »</strong></a>
  </p>
</p>

## About

Obake is a procedural macro for declaring and maintaining versioned data-structures. The name
'obake' is taken from the Japanese 'お化け (おばけ)', a class of supernatural beings in
Japanese folklore that shapeshift.

When developing an application, configuration formats and internal data-structures typically evolve
between versions. However, maintaining backwards compatibility between these versions requires
declaring and maintaining data-structures for legacy formats and code for migrating between them.
Obake aims to make this process effortless.

## Getting Started

To get started, add the following to your `Cargo.toml` file:

```toml
[dependencies]
obake = "1.0"
```

## Example

```rust
#[obake::versioned]                 // create a versioned data-structure
#[obake(version("0.1.0"))]          // declare some versions
#[obake(version("0.2.0"))]
#[derive(Debug, PartialEq, Eq)]     // additional attributes are applied to all versions
struct Foo {
    #[obake(cfg("0.1.0"))]          // enable fields for specific versions with
    foo: String,                    // semantic version constraints
   
    #[obake(cfg(">=0.2, <=0.3.0"))] // any semantic version constraint can appear in
    bar: u32,                       // a `cfg` attribute 
   
    #[obake(cfg("0.1.0"))]          // multiple `cfg` attributes are treated as a
    #[obake(cfg(">=0.3"))]          // disjunction over version constraints
    baz: char,
}

// describe migrations between versions using the `From` trait
// and an automatically generated type-level macro for referring to
// specific versions of `Foo`
impl From<Foo!["0.1.0"]> for Foo!["0.2.0"] {
    fn from(foo: Foo!["0.1.0"]) -> Self {
        Self { bar: 0 }
    }
}

// an enumeration of all versions of `Foo` is accessed using the `obake::AnyVersion` type
// alias
let versioned_example: obake::AnyVersion<Foo> = (Foo { bar: 42 }).into();

// this enumeration implements `Into<Foo>`, where `Foo` is the latest declared
// version of `Foo` (in this case, `Foo!["0.2.0"]`)
let example: Foo = versioned_example.into();

assert_eq!(example, Foo { bar: 42 });
```

## Other Features

- `#[obake(inherit)]`: allows nesting of versioned data-structures.
- `#[obake(derive(...))]`: allows derive attributes to be applied to generated enums.
- `#[obake(serde(...))]`: allows [`serde`](https://serde.rs) attributes to be applied to
  generated `enum`s.
  - Note: requires the feature `serde`.

## Limitations

- Cannot be applied to tuple structs (or enum variants with unnamed fields).
- Cannot be applied to items with generic parameters.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Obake by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

[crates-io-shield]: https://img.shields.io/crates/v/obake
[crates-io-url]: https://crates.io/crates/obake
[issues-shield]: https://img.shields.io/github/issues/doctorn/obake.svg
[issues-url]: https://github.com/doctorn/obake/issues
[license-shield]: https://img.shields.io/crates/l/obake
[license-url]: https://github.com/doctorn/obake/blob/main/LICENSE-APACHE

