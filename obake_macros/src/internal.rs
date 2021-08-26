use syn::Token;

pub use proc_macro::TokenStream;
pub use proc_macro2::{Span, TokenStream as TokenStream2};

pub use semver::{Version, VersionReq};

#[derive(Clone)]
pub struct VersionAttr {
    pub version: Version,
    pub span: Span,
}

#[derive(Clone)]
pub struct CfgAttr {
    pub req: VersionReq,
    pub span: Span,
}

#[derive(Clone)]
pub struct InheritAttr {
    pub span: Span,
}

#[derive(Clone)]
pub enum ObakeAttribute {
    Version(VersionAttr),
    Cfg(CfgAttr),
    Inherit(InheritAttr),
}

#[derive(Clone)]
pub struct VersionedField {
    pub attrs: VersionedAttributes,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub colon_token: Token![:],
    pub ty: syn::Type,
}

#[derive(Clone)]
pub struct VersionedFields {
    pub brace_token: syn::token::Brace,
    pub fields: syn::punctuated::Punctuated<VersionedField, Token![,]>,
}

#[derive(Clone)]
pub enum VersionedAttribute {
    Obake(ObakeAttribute),
    Attribute(syn::Attribute),
}

#[derive(Clone)]
pub struct VersionedAttributes {
    pub attrs: Vec<VersionedAttribute>,
}

impl ObakeAttribute {
    pub fn version(&self) -> Option<&VersionAttr> {
        #![allow(clippy::match_wildcard_for_single_variants)]
        match &self {
            ObakeAttribute::Version(version) => Some(version),
            _ => None,
        }
    }

    pub fn cfg(&self) -> Option<&CfgAttr> {
        #![allow(clippy::match_wildcard_for_single_variants)]
        match &self {
            ObakeAttribute::Cfg(cfg) => Some(cfg),
            _ => None,
        }
    }

    pub fn inherit(&self) -> Option<&InheritAttr> {
        #![allow(clippy::match_wildcard_for_single_variants)]
        match &self {
            ObakeAttribute::Inherit(inherit) => Some(inherit),
            _ => None,
        }
    }
}

impl VersionedAttribute {
    pub fn obake(&self) -> Option<&ObakeAttribute> {
        #![allow(clippy::match_wildcard_for_single_variants)]
        match &self {
            VersionedAttribute::Obake(obake) => Some(obake),
            _ => None,
        }
    }

    pub fn attr(&self) -> Option<&syn::Attribute> {
        #![allow(clippy::match_wildcard_for_single_variants)]
        match self {
            VersionedAttribute::Attribute(attr) => Some(attr),
            _ => None,
        }
    }
}

impl VersionedAttributes {
    pub fn obake(&self) -> impl Iterator<Item = &ObakeAttribute> + '_ {
        self.attrs.iter().filter_map(VersionedAttribute::obake)
    }

    pub fn versions(&self) -> impl Iterator<Item = &VersionAttr> + '_ {
        self.obake().filter_map(ObakeAttribute::version)
    }

    pub fn cfgs(&self) -> impl Iterator<Item = &CfgAttr> + '_ {
        self.obake().filter_map(ObakeAttribute::cfg)
    }

    pub fn inherits(&self) -> impl Iterator<Item = &InheritAttr> + '_ {
        self.obake().filter_map(ObakeAttribute::inherit)
    }

    pub fn attrs(&self) -> impl Iterator<Item = &syn::Attribute> + '_ {
        self.attrs.iter().filter_map(VersionedAttribute::attr)
    }
}

#[derive(Clone)]
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
