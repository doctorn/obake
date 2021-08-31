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
