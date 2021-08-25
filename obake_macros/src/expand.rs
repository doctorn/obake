use quote::{format_ident, ToTokens, TokenStreamExt};

use crate::internal::*;

pub trait ToTokensVersioned {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut TokenStream2);
}

pub trait TokenStreamVersionedExt: TokenStreamExt {
    fn append_versioned<T>(&mut self, version: &Version, t: &T)
    where
        T: ToTokensVersioned;
}

impl TokenStreamVersionedExt for TokenStream2 {
    fn append_versioned<T>(&mut self, version: &Version, t: &T)
    where
        T: ToTokensVersioned,
    {
        t.to_tokens_versioned(version, self);
    }
}

impl ToTokensVersioned for syn::Ident {
    fn to_tokens_versioned(&self, version: &Version, tokens: &mut TokenStream2) {
        tokens.append(format_ident!(
            "{}_v{}_{}_{}",
            self,
            version.major,
            version.minor,
            version.patch
        ));
    }
}

/*
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
*/

impl ToTokens for VersionedStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        /*
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
        let migrations = self
            .versions
            .iter()
            .skip(1)
            .zip(variants.clone())
            .map(|(attr, prev)| {
                if let Some(migration) = attr.migration.as_ref() {
                    let next = attr.version(ident);
                    quote! {
                        #versioned_enum::#prev(x) => #versioned_enum::#next(#migration(x)),
                    }
                } else {
                    Default::default()
                }
            });
        let ty_rules = self
            .versions
            .iter()
            .zip(variants.clone())
            .map(|(attr, variant)| {
                let version = format!("{}", attr.version);
                quote! {
                    [#version] => { #variant };
                }
            });

        tokens.append_all(quote! {
            macro_rules! #ident {
                #(#ty_rules)*
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
        */
        unimplemented!()
    }
}
