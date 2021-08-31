//! # Obake
//!
//! Obake is a procedural macro for declaring and maintaining versioned data-structures. The name
//! 'obake' is taken from the Japanese 'お化け (おばけ)', a class of supernatural beings in
//! Japanese folklore that shapeshift.
//!
//! When developing an application, configuration formats and internal data-structures typically evolve
//! between versions. However, maintaining backwards compatibility between these versions requires
//! declaring a maintaining data-structures for legacy formats and code for migrating between them.
//! Obake aims to make this process effortless.
//!
//! ## Example
//!
//! ```
//! #[obake::versioned]                 // create a versioned data-structure
//! #[obake(version("0.1.0"))]          // declare some versions
//! #[obake(version("0.2.0"))]
//! #[derive(Debug, PartialEq, Eq)]     // additional attributes are applied to all versions
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
//! // an enumeration of all versions of `Foo` is accessed using the `obake::AnyVersion` type
//! // alias.
//! let versioned_example: obake::AnyVersion<Foo> = (Foo { bar: 42 }).into();
//!
//! // this enumeration implements `Into<Foo>`, where `Foo` is the latest declared
//! // version of `Foo` (in this case, `Foo!["0.2.0"]`)
//! let example: Foo = versioned_example.into();
//!
//! assert_eq!(example, Foo { bar: 42 });
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

/// The core macro of the library. Used to declare versioned data-structures.
///
/// ### Supported attributes:
///
/// - `#[obake(version("x.y.z"))]` - Declares a possible version of the data-structure.
/// - `#[obake(cfg(...))]` - Specifies a semantic version constraints for a particular field or
///    variant.
///   - `cfg` can contain any number of comma-separated semantic version constraints (e.g.,
///     `#[obake(cfg(">=0.3, <=0.1"))]`).
///   - A field or variant marked with a `cfg` attribute will only appear in a particular version
///     of the data-structure type all of the attributes constraints are satisfied by that
///     version.
///   - In the presence of multiple `cfg` attributes, any matching `cfg` will result in a match
///     (i.e., while comman-seperated constraints are treated as a conjunctively, multiple `cfg`
///     attributes are treated as a disjunctively).
/// - `#[obake(derive(...))]` - Apply a derive to the version-tagged enum generated for the
///    data-structre.
/// - `#[obake(inherit)]` - Marks a field as having an inherited version (i.e., given a field of
///   type `Bar`, when marked with `inherit`, this field will be expanded to a field of type
///   `Bar![{version}]` in every version).
// TODO(@doctorn) document generated types and trait implementations
pub use obake_macros::versioned;

/// Automatically implemented for the latest version of a versioned data-structure.
///
/// ## Note
///
/// Not intended to be hand-implemented, use [`versioned`] to derive it.
pub trait Versioned: Sized {
    /// The associated type, `Versioned`, points to the version-tagged representation of this
    /// data-structure.
    type Versioned: VersionTagged<Self>;
}

/// Automatically implemented by the generated version-tagged encoding of a [`versioned`]
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

/// Automatically implemented for all declared versions of a versioned data-structure.
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
    /// ## Errors
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

/// A struct representing a mismatch of versions.
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
