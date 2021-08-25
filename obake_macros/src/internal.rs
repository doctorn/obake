use syn::Token;

pub use proc_macro::TokenStream;
pub use proc_macro2::{Span, TokenStream as TokenStream2};

pub use semver::{Version, VersionReq};

pub struct VersionAttr {
    pub version: Version,
    pub span: Span,
    pub migration: Option<syn::Path>,
}

pub struct RequiresAttr {
    pub requirement: VersionReq,
    pub span: Span,
}

pub struct InheritAttr {
    pub span: Span,
}

pub enum OkabeAttribute {
    Version(VersionAttr),
    Requires(RequiresAttr),
    Inherit(InheritAttr),
}

pub struct VersionedField {
    pub attrs: VersionedAttributes,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub colon_token: Token![:],
    pub inherit: Option<InheritAttr>,
    pub ty: syn::Type,
}

pub struct VersionedFields {
    pub brace_token: syn::token::Brace,
    pub versioned: syn::punctuated::Punctuated<VersionedField, Token![,]>,
}

pub enum VersionedAttribute {
    Okabe(OkabeAttribute),
    Attribute(syn::Attribute),
}

pub struct VersionedAttributes {
    pub attrs: Vec<VersionedAttribute>,
}

impl OkabeAttribute {
    pub fn version(&self) -> Option<&VersionAttr> {
        match &self {
            OkabeAttribute::Version(version) => Some(version),
            _ => None,
        }
    }
}

impl VersionedAttribute {
    pub fn okabe(&self) -> Option<&OkabeAttribute> {
        match &self {
            VersionedAttribute::Okabe(okabe) => Some(okabe),
            _ => None,
        }
    }

    pub fn attr(&self) -> Option<&syn::Attribute> {
        match self {
            VersionedAttribute::Attribute(attr) => Some(attr),
            _ => None,
        }
    }
}

impl VersionedAttributes {
    pub fn versions(&self) -> impl Iterator<Item = &VersionAttr> + '_ {
        self.attrs
            .iter()
            .filter_map(VersionedAttribute::okabe)
            .filter_map(OkabeAttribute::version)
    }
}

pub struct VersionedStruct {
    pub attrs: VersionedAttributes,
    pub vis: syn::Visibility,
    pub struct_token: Token![struct],
    pub ident: syn::Ident,
    pub fields: VersionedFields,
}

impl PartialEq for VersionAttr {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl Eq for VersionAttr {}

impl PartialOrd for VersionAttr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for VersionAttr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}
