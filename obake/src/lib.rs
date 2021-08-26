#![deny(clippy::all, clippy::pedantic)]
#![no_std]

pub use obake_macros::versioned;

pub trait Versioned: Sized {
    type Versioned: Into<Self>;
}
