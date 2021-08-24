use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use syn::parse::{Nothing, Parse, ParseStream};
use syn::{braced, parse_macro_input, Result, Token};

use semver::{Version, VersionReq};

trait ToTokensVersioned {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut proc_macro2::TokenStream);
}

trait TokenStreamVersionedExt {
    fn append_versioned<T>(&mut self, version: &Version, t: &T)
    where
        T: ToTokensVersioned;
}

impl TokenStreamVersionedExt for proc_macro2::TokenStream {
    fn append_versioned<T>(&mut self, version: &Version, t: &T)
    where
        T: ToTokensVersioned,
    {
        t.to_tokens_versioned(version, self);
    }
}

struct VerInput {
    ident: syn::Ident,
    _at_token: Token![@],
    version: Version,
    body: Option<proc_macro2::TokenStream>,
}

impl VerInput {
    fn version(&self) -> syn::Ident {
        versioned_ident(&self.ident, &self.version)
    }
}

struct VersionedField {
    requirement: VersionReq,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    ident: syn::Ident,
    colon_token: Token![:],
    ty: syn::Type,
}

struct VersionedFields {
    _brace_token: syn::token::Brace,
    versioned: syn::punctuated::Punctuated<VersionedField, Token![,]>,
}

struct VersionAttr {
    version: Version,
    span: proc_macro2::Span,
    migration: Option<syn::Path>,
}

fn versioned_ident(ident: &syn::Ident, version: &Version) -> syn::Ident {
    format_ident!(
        "{}_v{}_{}_{}",
        ident,
        version.major,
        version.minor,
        version.patch
    )
}

impl VersionAttr {
    fn version(&self, ident: &syn::Ident) -> syn::Ident {
        versioned_ident(ident, &self.version)
    }
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

enum OuterAttribute {
    Version(VersionAttr),
    Attribute(syn::Attribute),
}

struct VersionedStruct {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    struct_token: Token![struct],
    ident: syn::Ident,
    fields: VersionedFields,
    versions: Vec<VersionAttr>,
}

impl Parse for VerInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let at_token = input.parse()?;
        let version_str = input.parse::<syn::LitStr>()?;

        let body = if !input.is_empty() {
            let content;
            braced!(content in input);
            Some(content.parse()?)
        } else {
            None
        };

        let version = Version::parse(&version_str.value())
            .map_err(|err| syn::Error::new(version_str.span(), err))?;

        Ok(Self {
            ident,
            _at_token: at_token,
            version,
            body,
        })
    }
}

impl ToTokens for VerInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.version());
        if let Some(body) = &self.body {
            tokens.append_all(quote! {
                { #body }
            });
        }
    }
}

impl Parse for VersionedField {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = vec![];
        let mut requirement = None;
        let unfiltered = input.call(syn::Attribute::parse_outer)?;

        for attr in unfiltered {
            match attr.path.get_ident() {
                Some(path) if path == "requires" => {
                    if requirement.is_some() {
                        return Err(syn::Error::new(
                            path.span(),
                            "multiple uses of the `#[requires(...)]` attribute on field",
                        ));
                    }

                    let version_str = attr.parse_args::<syn::LitStr>()?;
                    let version_req = VersionReq::parse(&version_str.value())
                        .map_err(|err| syn::Error::new(path.span(), err))?;

                    requirement = Some(version_req);
                }
                _ => attrs.push(attr),
            }
        }

        let requirement = requirement.unwrap_or_default();

        Ok(Self {
            requirement,
            attrs,
            vis: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl ToTokensVersioned for VersionedField {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut proc_macro2::TokenStream) {
        if self.requirement.matches(version) {
            let attrs = &self.attrs;
            let vis = &self.vis;
            let ident = &self.ident;
            let colon_token = &self.colon_token;
            let ty = &self.ty;

            tokens.append_all(quote! {
                #(#attrs)*
                #vis #ident #colon_token #ty,
            })
        }
    }
}

impl Parse for VersionedFields {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _brace_token: braced!(content in input),
            versioned: content.parse_terminated(VersionedField::parse)?,
        })
    }
}

impl ToTokensVersioned for VersionedFields {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut proc_macro2::TokenStream) {
        let mut group = proc_macro2::TokenStream::new();

        for field in &self.versioned {
            group.append_versioned(version, field);
        }

        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Brace, group);
        tokens.append(group);
    }
}

impl Parse for VersionAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let version_str = input.parse::<syn::LitStr>()?;
        let migration = if !input.is_empty() {
            input.parse::<Token![,]>()?;
            Some(input.parse::<syn::Path>()?)
        } else {
            None
        };

        let span = version_str.span();
        let version = Version::parse(&version_str.value())
            .map_err(|err| syn::Error::new(version_str.span(), err))?;

        Ok(Self {
            version,
            span,
            migration,
        })
    }
}

impl OuterAttribute {
    fn parse(input: ParseStream) -> Result<Vec<OuterAttribute>> {
        let mut attrs = vec![];
        let unfiltered = input.call(syn::Attribute::parse_outer)?;

        for attr in unfiltered {
            match attr.path.get_ident() {
                Some(path) if path == "version" => {
                    attrs.push(OuterAttribute::Version(attr.parse_args()?))
                }
                _ => attrs.push(OuterAttribute::Attribute(attr)),
            }
        }

        Ok(attrs)
    }
}

impl Parse for VersionedStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(OuterAttribute::parse)?;
        let vis = input.parse::<syn::Visibility>()?;
        let struct_token = input.parse::<Token![struct]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let fields = input.parse()?;

        let mut versions = vec![];
        let mut filtered_attrs = vec![];

        for attr in attrs {
            match attr {
                OuterAttribute::Version(version_attr) => versions.push(version_attr),
                OuterAttribute::Attribute(attr) => filtered_attrs.push(attr),
            }
        }

        versions.sort();
        for i in 1..versions.len() {
            let attr = &versions[i - 1];
            if versions[i..].contains(&attr) {
                return Err(syn::Error::new(
                    attr.span,
                    format!("duplicate definition of version {}", attr.version),
                ));
            }
        }

        Ok(Self {
            attrs: filtered_attrs,
            vis,
            struct_token,
            ident,
            fields,
            versions,
        })
    }
}

impl ToTokensVersioned for VersionedStruct {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut proc_macro2::TokenStream) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let struct_token = &self.struct_token;
        let ident = versioned_ident(&self.ident, version);

        tokens.append_all(quote! {
            #[allow(non_camel_case_types)]
            #(#attrs)*
            #vis #struct_token #ident
        });
        tokens.append_versioned(version, &self.fields);
    }
}

impl ToTokens for VersionedStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let current = self.versions.iter().map(|attr| &attr.version).max();

        for attr in &self.versions {
            if &attr.version != current.unwrap() {
                tokens.append_all(quote! {
                    #[doc(hidden)]
                })
            }
            tokens.append_versioned(&attr.version, self);
        }

        let vis = &self.vis;
        let ident = &self.ident;
        let versioned_enum = format_ident!("Versioned{}", ident);

        let variants = self.versions.iter().map(|attr| attr.version(ident));
        let migrations = self.versions.iter().skip(1).zip(variants.clone()).map(|(attr, prev)| {
            if let Some(migration) = attr.migration.as_ref() {
                let next = attr.version(ident);
                quote! {
                    #versioned_enum::#prev(x) => #versioned_enum::#next(#migration(x)),
                }
            } else {
                Default::default() 
            }
        }); 

        tokens.append_all(quote! {
            #vis enum #versioned_enum {
                #(
                    #[allow(non_camel_case_types)]
                    #variants(#variants),
                )*
            }
        });

        if let Some(attr) = self.versions.last() {
            let alias = attr.version(ident);

            tokens.append_all(quote! {
                #vis type #ident = #alias;

                impl From<#versioned_enum> for #ident {
                    fn from(mut from: #versioned_enum) -> Self {
                        loop {
                            from = match from {
                                #(#migrations)*
                                #versioned_enum::#alias(x) => return x,
                            };
                        }
                    }
                }
            });
        }
    }
}

#[proc_macro_attribute]
pub fn versioned(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(input as VersionedStruct);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn ver(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as VerInput);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}
