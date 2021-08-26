#![deny(clippy::all, clippy::pedantic)]
// Ignored clippy lints
#![allow(clippy::used_underscore_binding, clippy::wildcard_in_or_patterns)]
// Ignored clippy_pedantic lints
#![allow(
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::similar_names,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]
#![no_std]

pub use obake_macros::versioned;

pub trait Versioned: Sized {
    type Versioned: Into<Self>;
}
