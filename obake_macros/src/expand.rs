use syn::Result;

use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::internal::*;

macro_rules! try_expand {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(err) => return err.into_compile_error(),
        }
    };
}

trait VersionExt {
    fn version(&self, version: &Version) -> Self;
}

impl VersionExt for syn::Ident {
    fn version(&self, version: &Version) -> Self {
        format_ident!(
            "{}_v{}_{}_{}",
            self,
            version.major,
            version.minor,
            version.patch
        )
    }
}

impl VersionedField {
    fn expand_ty_versioned(&self, version: &Version) -> Result<TokenStream2> {
        if self.attrs.inherits().next().is_none() {
            let ty = &self.ty;
            return Ok(quote!(#ty));
        }

        if let syn::Type::Path(ty_path) = &self.ty {
            let mut ty_path = ty_path.clone();

            if let Some(mut terminator) = ty_path.path.segments.last_mut() {
                terminator.ident = terminator.ident.version(version);
                return Ok(quote!(#ty_path));
            }
        }

        Err(syn::Error::new(
            self.attrs.inherits().next().unwrap().span,
            "`#[obake(inherit)]` can only be applied to fields with `#[obake::versioned]` types",
        ))
    }

    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        let mut reqs: Vec<_> = self.attrs.cfgs().map(|attr| attr.req.clone()).collect();

        // If we have no `#[obake(cfg(...))]` attributes, default to `#[obake(cfg("*"))]`
        if reqs.is_empty() {
            reqs.push(VersionReq::STAR);
        }

        // If we can't find a matching `#[obake(cfg(...))]` attribute, this field is disabled
        // in this version, so return nothing
        if !reqs.iter().any(|req| req.matches(version)) {
            return Ok(quote!());
        }

        let attrs = self.attrs.attrs();
        let vis = &self.vis;
        let ident = &self.ident;
        let colon_token = &self.colon_token;
        let ty = self.expand_ty_versioned(version)?;

        Ok(quote! {
            #(#attrs)*
            #vis #ident #colon_token #ty,
        })
    }
}

impl VersionedFields {
    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        let fields = self
            .fields
            .iter()
            .map(|field| field.expand_version(version))
            .collect::<Result<Vec<_>>>()?
            .into_iter();

        Ok(quote! {
            {
                #(#fields)*
            }
        })
    }
}

impl VersionedStruct {
    fn extract_versions(&self) -> Result<Vec<VersionAttr>> {
        let mut versions: Vec<_> = self.attrs.versions().cloned().collect();
        versions.sort();

        // Duplicate version declarations result in an ambiguity in the
        // choice of migration, so check that we don't have any duplicates.
        //
        // As versions are sorted and totally ordered, it's enough to check that
        // pairwise adjacent versions are unequal.
        for i in 1..versions.len() {
            let head = &versions[i];
            if head == &versions[i - 1] {
                return Err(syn::Error::new(
                    head.span,
                    format!("duplicate definition of version {}", head.version),
                ));
            }
        }

        Ok(versions)
    }

    fn check_preconditions(&self) -> Result<()> {
        if let Some(inherit) = self.attrs.inherits().next() {
            return Err(syn::Error::new(
                inherit.span,
                "`#[obake(inherit)]` not valid in this context",
            ));
        }

        if let Some(req) = self.attrs.cfgs().next() {
            return Err(syn::Error::new(
                req.span,
                "`#[obake(cfg(...))]` not valid in this context",
            ));
        }

        if self.attrs.versions().next().is_none() {
            return Err(syn::Error::new_spanned(
                self.struct_token,
                "`#[obake::versioned]` structs require at least one `#[obake(version(...))]` attribute",
            ));
        }

        Ok(())
    }

    fn expand_version(&self, version: &Version) -> TokenStream2 {
        let attrs = self.attrs.attrs();
        let vis = &self.vis;
        let struct_token = &self.struct_token;
        let ident = &self.ident.version(version);
        let fields = &try_expand!(self.fields.expand_version(version));

        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #(#attrs)*
            #vis #struct_token #ident #fields
        }
    }

    fn expand_variants(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.attrs
            .versions()
            .map(move |attr| self.ident.version(&attr.version))
    }

    fn expand(&self) -> TokenStream2 {
        try_expand!(self.check_preconditions());

        let versions = try_expand!(self.extract_versions());
        let current = versions.last().unwrap();

        let defs = versions
            .iter()
            .map(|attr| self.expand_version(&attr.version));

        let alias = &self.ident.version(&current.version);
        let alias_decl = {
            let vis = &self.vis;
            let ident = &self.ident;

            quote! {
                #vis type #ident = #alias;
            }
        };

        let enum_ident = format_ident!("Versioned{}", self.ident);
        let enum_decl = {
            let vis = &self.vis;
            let attrs = self.attrs.attrs();
            let variants = self.expand_variants();

            quote! {
                #[doc(hidden)]
                #(#attrs)*
                #vis enum #enum_ident {
                    #(
                        #[allow(non_camel_case_types)]
                        #variants(#variants),
                    )*
                }
            }
        };

        let from_impl = {
            let ident = &self.ident;
            let migrations =
                versions
                    .iter()
                    .skip(1)
                    .zip(self.expand_variants())
                    .map(|(attr, prev)| {
                        let next = ident.version(&attr.version);

                        quote! {
                            #enum_ident::#prev(x) => #enum_ident::#next(x.into()),
                        }
                    });

            quote! {
                #[automatically_derived]
                impl From<#enum_ident> for #ident {
                    fn from(mut from: #enum_ident) -> Self {
                        loop {
                            from = match from {
                                #(#migrations)*
                                #enum_ident::#alias(x) => return x,
                            };
                        }
                    }
                }
            }
        };

        let versioned_impl = {
            let ident = &self.ident;

            quote! {
                impl ::obake::Versioned for #ident {
                    type Versioned = #enum_ident;
                }
            }
        };

        let macro_rules = {
            let ident = &self.ident;
            let rules = self
                .attrs
                .versions()
                .zip(self.expand_variants())
                .map(|(attr, variant)| {
                    let version = format!("{}", attr.version);
                    quote! {
                        [#version] => { #variant };
                    }
                });

            quote! {
                macro_rules! #ident {
                    #(#rules)*
                }
            }
        };

        quote! {
            #(#defs)*
            #alias_decl
            #enum_decl
            #from_impl
            #versioned_impl
            #macro_rules
        }
    }
}

impl ToTokens for VersionedStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(self.expand());
    }
}
