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
//!
//! ## Example
//!
//! ```rust
//! #[obake::versioned]                 // create a versioned data-structure
//! #[obake(version("0.1.0"))]          // declare some versions
//! #[obake(version("0.2.0"))]
//! #[derive(Default, PartialEq, Hash)] // additional attributes are applied to all versions
//! struct Foo {
//!     #[obake(cfg("0.1.0"))]          // enable fields for specific versions with
//!     foo: String,                    // semantic version constraints
//!    
//!     #[obake(cfg(">=0.2, <=0.3.0"))] // any semantic version constraint can appear in
//!     bar: u32,                       // a `cfg` attribute 
//!    
//!     #[obake(cfg("0.1.0"))]          // multiple `cfg` attributes are treated as a
//!     #[obake(cfg(">=0.3"))]          // disjunction over version constraints
//!     baz: char,
//! }
//! 
//! // describe migrations between versions using the `From` trait
//! // and an automatically generated type-level macro for referring to
//! // specific versions of `Foo`
//! impl From<Foo!["0.1.0"]> for Foo!["0.2.0"] {
//!     fn from(foo: Foo!["0.1.0"]) -> Self {
//!         Self { bar: 0 }
//!     }
//! }
//! 
//! // an enumeration of all versions of `Foo` is accessed using the
//! // `obake::Versioned` trait:
//! # let _ = || {
//! let versioned_example: <Foo as obake::Versioned>::Versioned = unimplemented!();
//! # };
//! # let versioned_example = <Foo as obake::Versioned>::Versioned::Foo_v0_1_0(Default::default()); 
//! 
//! // this enumeration implements `Into<Foo>`, where `Foo` is the latest declared
//! // version of `Foo` (in this case, `Foo!["0.2.0"]`)
//! let example: Foo = versioned_example.into();
//! ```
//!
//! ## Other Features
//! 
//! - `#[obake(inherit)]`: allows nesting of versioned data-structures.
//! - `#[obake(derive(...))]`: allows derive attributes to be applied to generated `enum`s.
//! 
//! ## Limitations
//! 
//! - Cannot be applied to tuple `struct`s (or `enum` variants with unnamed fields).
//! - Cannot be applied to items with generic parameters.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![deny(missing_docs, unused_imports)]

/// The core macro of the library. Used to make a versioned data structure.
///
/// ### Supported attributes:
///
/// - `#[obake(version("x.y.z"))]` - Declare a possible version for the type
/// - `#[obake(cfg(...))]` - Specify a version for a given field
///   - `cfg` can contain any number of comma-separated semantic version constraints
///     - Example: `#[obake(version(">=0.3"))]`
///   - multiple `cfg` attributes are treated as a disjunction over version constraints (i.e.
///     true if any of the listed constraints holds true)
/// - `#[obake(derive(...))]` - Apply a derive to the [`Versioned`] enum generated for the type
///   - Note: This will behave as any derive applied to an enum would (for example if you derive
///   `Deserialize`, it will expect the enum to be [tagged] by `{name}_v{version}`)
/// - `#[obake(inherit)]` - Allows a field to be a nested versioned data structure. That is to say
/// that this field will be of type `{}`
///
/// [tagged]: https://serde.rs/enum-representations.html#externally-tagged
///
/// ### Generated types
///
/// - `struct {type_name}_v{major}_{minor}_{patch}` - A struct representing the type for the given
/// version
/// - `struct {type_name}` - A struct equivelant to the highest version declared
/// - `enum {type_name}Versioned` - An enum representing all possible versions of the struct
///     - Note: this should only be accessed via `<T as obake::Versioned>::Versioned`
///     - Variants:
///         - `{type_name}_v{major}_{minor}_{patch}` - a variant representing a versioned struct of
///         the type of the same name
///
/// ### Implemented traits
///
/// The type this macro is applied to will implement:
///
/// - `From<T>` and `Into<T>` where `T` is the `Versioned` enum for the given type.
/// - [`Versioned`]
pub use obake_macros::versioned;

/// Automatically implemented for the latest version of a versioned data-structure.
///
/// Not intended to be hand-implemented, use [`versioned`] to derive it.
pub trait Versioned: Sized {
    /// Aliases the versioned encoding of a versioned data-structure.
    type Versioned: From<Self> + Into<Self>;
}
