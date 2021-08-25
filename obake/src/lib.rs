#![deny(clippy::all, clippy::pedantic)]
// Ignored clippy lints
#![allow(
    clippy::used_underscore_binding,
    clippy::wildcard_in_or_patterns
)]
// Ignored clippy_pedantic lints
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::similar_names,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]

#![no_std]

/*
pub use obake_macros::versioned;

#[versioned]
#[version("0.1.0")]
#[version("0.2.0", migrate_to_0_2_0)]
#[version("0.3.0", migrate_to_0_3_0)]
struct Foo {
    #[requires(">=0.2")]
    field_1: i32,
    #[requires("0.2")]
    field_2: u64,
    #[requires(">0.2")]
    field_3: u32,
}

fn migrate_to_0_2_0(foo: Foo!["0.1.0"]) -> Foo!["0.2.0"] {
    type X = Foo!["0.2.0"];
    X {
        field_1: 0,
        field_2: 0,
    }
}

fn migrate_to_0_3_0(foo: Foo!["0.2.0"]) -> Foo!["0.3.0"] {
    let x: Foo!["0.1.0"] = unimplemented!();
    // obake_macros::ver_internal![Foo["0.3.0"]] {
    //     field_0: foo.field_0,
    //     field_1: foo.field_1,
    //     field_3: 0,
    // }
    unimplemented!()
}
*/
