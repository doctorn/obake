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

/// A trait, automatically implemented for the latest version of a versioned data-structure.
///
/// ## Note
///
/// Not intended to be hand-implemented, use [`versioned`] to derive it.
pub trait Versioned: Sized {
    /// The associated type, `Versioned`, points to the version-tagged representation of this
    /// data-structure.
    type Versioned: VersionTagged<Self>;
}

/// A trait, automatically implemented by the generated version-tagged encoding of a [`versioned`]
/// data-structure.
///
/// ## Note
///
/// Not intended to be hand-implemented, use [`versioned`] to derive it.
pub trait VersionTagged<T>: From<T> + Into<T> {
    /// The semantic version number corresponding to the tag of a particular instance.
    fn version_str(&self) -> &'static str;
}

/// Short-hand for referring to the version-tagged representation of a [`versioned`] data-structre.
pub type AnyVersion<T> = <T as Versioned>::Versioned;

/// A trait, automatically implemented for all declared versions of a versioned data-structure.
///
/// ## Note
///
/// Not intended to be hand-implemented, use [`versioned`] to derive it.
pub trait VersionOf<T>: Into<AnyVersion<T>>
where
    T: Versioned,
{
    /// The semantic version number of this version.
    const VERSION: &'static str;

    /// Trys to convert the version-tagged representation of `T` into this particular version.
    ///
    /// If `tagged.version_str() != Self::VERSION`, this conversion will fail and report a
    /// corresponding [`VersionMismatch`].
    ///
    /// ```
    /// use obake::VersionOf;
    ///
    /// #[obake::versioned]
    /// #[obake(version("0.1.0"))]
    /// #[obake(version("0.2.0"))]
    /// # #[derive(PartialEq, Eq, Debug)]
    /// struct Foo {}
    ///
    /// # impl From<Foo!["0.1.0"]> for Foo!["0.2.0"] {
    /// #     fn from(_: Foo!["0.1.0"]) -> Self {
    /// #         Self {}
    /// #     }
    /// # }
    ///
    /// let x: obake::AnyVersion<Foo> = (Foo {}).into();
    /// assert_eq!(
    ///     <Foo!["0.1.0"]>::try_from_versioned(x),
    ///     Err(obake::VersionMismatch {
    ///         expected: "0.1.0",
    ///         found: "0.2.0",
    ///     }),
    /// );
    /// 
    /// let x: obake::AnyVersion<Foo> = (Foo {}).into();
    /// assert_eq!(
    ///     <Foo!["0.2.0"]>::try_from_versioned(x),
    ///     Ok(Foo {}),
    /// );
    /// ```
    fn try_from_versioned(tagged: AnyVersion<T>) -> Result<Self, VersionMismatch>;
}

/// A struct representing a mismatch in versions.
///
/// Such a mismatch can occur when trying to convert a version-tagged representation of a piece
/// of data into a particular version.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct VersionMismatch {
    /// The expected version.
    pub expected: &'static str,
    /// The version found.
    pub found: &'static str,
}
