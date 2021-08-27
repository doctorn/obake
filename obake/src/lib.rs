//! # Obake
//!
//! Obake is a procedural macro for declaring and maintaining versioned data-structures. The name
//! 'obake' is taken from the Japanese 'お化け (おばけ)', a class of supernatural beings in
//! Japanese folklore that shapeshift.
//!
//! When developing an application, configuration formats and internal data-structures typically evolve
//! between versions. However, maintaining backwards compatability between these versions requires
//! declaring a maintaining data-structures for legacy formats and code for migrating between them.
//! Obake aims to make this process effortless.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs, unused_imports)]

/// The whole point.
pub use obake_macros::versioned;

/// Automatically implemented for the latest version of a versioned data-structure.
pub trait Versioned: Sized {
    /// Aliases the versioned encoding of a versioned data-structure.
    type Versioned: From<Self> + Into<Self>;
}
