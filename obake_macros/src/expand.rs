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

            if let Some(terminator) = ty_path.path.segments.last_mut() {
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
        if let Some(derive) = self.attrs.derives().next() {
            return Err(syn::Error::new(
                derive.span,
                "`#[obake(derive(...))]` not valid in this context",
            ));
        }

        #[cfg(feature = "serde")]
        if let Some(serde) = self.attrs.serdes().next() {
            return Err(syn::Error::new(
                serde.span,
                "`#[obake(serde(...))]` not valid in this context",
            ));
        }

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

        Ok(quote!({
            #(#fields)*
        }))
    }
}

impl VersionedVariantFields {
    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        Ok(match &self {
            Self::Unnamed(unnamed) => quote!(#unnamed),
            Self::Named(named) => {
                let fields = named.expand_version(version)?;
                quote!(#fields)
            }
            Self::Unit => quote!(),
        })
    }
}

impl VersionedVariant {
    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        if let Some(derive) = self.attrs.inherits().next() {
            return Err(syn::Error::new(
                derive.span,
                "`#[obake(inherit)]` not valid in this context",
            ));
        }

        if let Some(derive) = self.attrs.derives().next() {
            return Err(syn::Error::new(
                derive.span,
                "`#[obake(derive(...))]` not valid in this context",
            ));
        }

        #[cfg(feature = "serde")]
        if let Some(serde) = self.attrs.serdes().next() {
            return Err(syn::Error::new(
                serde.span,
                "`#[obake(serde(...))]` not valid in this context",
            ));
        }

        let mut reqs: Vec<_> = self.attrs.cfgs().map(|attr| attr.req.clone()).collect();

        // If we have no `#[obake(cfg(...))]` attributes, default to `#[obake(cfg("*"))]`
        if reqs.is_empty() {
            reqs.push(VersionReq::STAR);
        }

        // If we can't find a matching `#[obake(cfg(...))]` variant, this field is disabled
        // in this version, so return nothing
        if !reqs.iter().any(|req| req.matches(version)) {
            return Ok(quote!());
        }

        let attrs = self.attrs.attrs();
        let ident = &self.ident;
        let fields = self.fields.expand_version(version)?;

        Ok(quote! {
            #(#attrs)*
            #ident #fields,
        })
    }
}

impl VersionedVariants {
    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        let variants = self
            .variants
            .iter()
            .map(|variant| variant.expand_version(version))
            .collect::<Result<Vec<_>>>()?
            .into_iter();

        Ok(quote!({
            #(#variants)*
        }))
    }
}

impl VersionedItem {
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
            return Err(syn::Error::new(
                self.keyword_span(),
                "`#[obake::versioned]` items require at least one `#[obake(version(...))]` attribute",
            ));
        }

        Ok(())
    }

    fn alias(&self) -> Option<syn::Ident> {
        self.attrs
            .versions()
            .last()
            .map(|attr| self.ident().version(&attr.version))
    }

    fn versioned_ident(&self) -> syn::Ident {
        format_ident!("Versioned{}", self.ident())
    }

    fn expand_version(&self, version: &Version) -> Result<TokenStream2> {
        let current = self.ident();
        let version_str = &version.to_string();
        let attrs = self.attrs.attrs();
        let vis = &self.vis;
        let ident = self.ident().version(version);
        let body = match &self.kind {
            VersionedItemKind::Struct(inner) => {
                let struct_token = &inner.struct_token;
                let fields = inner.fields.expand_version(version)?;
                quote!(#struct_token #ident #fields)
            }
            VersionedItemKind::Enum(inner) => {
                let enum_token = &inner.enum_token;
                let variants = inner.variants.expand_version(version)?;
                quote!(#enum_token #ident #variants)
            }
        };
        let versioned_ident = self.versioned_ident();

        Ok(quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #(#attrs)*
            #vis #body

            #[automatically_derived]
            impl ::obake::VersionOf<#current> for #ident {
                const VERSION: &'static str = #version_str;

                #[inline]
                fn try_from_versioned(
                    from: ::obake::AnyVersion<#current>,
                ) -> ::core::result::Result<Self, ::obake::VersionMismatch> {
                    use ::obake::VersionTagged;
                    match from {
                        ::obake::AnyVersion::<#current>::#ident(x) => ::core::result::Result::Ok(x),
                        other => ::core::result::Result::Err(::obake::VersionMismatch {
                            expected: Self::VERSION,
                            found: other.version_str(),
                        }),
                    }
                }
            }

            #[automatically_derived]
            impl ::core::convert::From<#ident> for #versioned_ident {
                #[inline]
                fn from(from: #ident) -> #versioned_ident {
                    #versioned_ident::#ident(from)
                }
            }
        })
    }

    fn expand_alias(&self) -> TokenStream2 {
        let vis = &self.vis;
        let ident = self.ident();
        let alias = self.alias().unwrap();

        quote!(#vis type #ident = #alias;)
    }

    fn expand_variants(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.attrs
            .versions()
            .map(move |attr| self.ident().version(&attr.version))
    }

    fn expand_versioned_enum(&self) -> TokenStream2 {
        let enum_ident = self.versioned_ident();
        let vis = &self.vis;
        let variants = self.expand_variants();
        let derives = self.attrs.derives().map(|attr| {
            let tokens = &attr.tokens;
            quote!(#[derive(#tokens)])
        });
        #[cfg(feature = "serde")]
        let derives = derives.chain(self.attrs.serdes().map(|attr| {
            let tokens = &attr.tokens;
            quote!(#[serde(#tokens)])
        }));

        quote! {
            #[doc(hidden)]
            #(#derives)*
            #[allow(clippy::enum_variant_names)]
            #vis enum #enum_ident {
                #(
                    #[allow(non_camel_case_types)]
                    #variants(#variants),
                )*
            }
        }
    }

    fn expand_from_impl(&self, versions: &[VersionAttr]) -> TokenStream2 {
        let ident = self.ident();
        let alias = self.alias().unwrap();
        let enum_ident = self.versioned_ident();
        let migrations = versions
            .iter()
            .skip(1)
            .zip(self.expand_variants())
            .map(|(attr, prev)| {
                let next = ident.version(&attr.version);
                quote!(#enum_ident::#prev(x) => #enum_ident::#next(x.into()),)
            });

        quote! {
            #[automatically_derived]
            impl ::core::convert::From<#enum_ident> for #ident {
                #[inline]
                fn from(mut from: #enum_ident) -> Self {
                    #![allow(unreachable_code)]
                    loop {
                        from = match from {
                            #(#migrations)*
                            #enum_ident::#alias(x) => return x,
                        };
                    }
                }
            }
        }
    }

    fn expand_versioned_impl(&self) -> TokenStream2 {
        let ident = self.ident();
        let enum_ident = self.versioned_ident();

        quote! {
            impl ::obake::Versioned for #ident {
                type Versioned = #enum_ident;
            }
        }
    }

    fn expand_version_tagged_impl(&self) -> TokenStream2 {
        let ident = self.ident();
        let enum_ident = self.versioned_ident();
        let variants = self.expand_variants();

        quote! {
            #[automatically_derived]
            impl ::obake::VersionTagged<#ident> for #enum_ident {
                #[inline]
                fn version_str(&self) -> &'static str {
                    use ::obake::VersionOf;
                    match self {
                        #(#enum_ident::#variants(_) => #variants::VERSION,)*
                    }
                }
            }
        }
    }

    fn expand_macro_rules(&self) -> TokenStream2 {
        let ident = self.ident();
        let rules = self
            .attrs
            .versions()
            .zip(self.expand_variants())
            .map(|(attr, variant)| {
                let version = attr.version.to_string();
                quote!([#version] => { #variant };)
            });

        quote! {
            macro_rules! #ident {
                #(#rules)*
            }
        }
    }

    fn expand(&self) -> TokenStream2 {
        try_expand!(self.check_preconditions());

        let versions = try_expand!(self.extract_versions());
        let defs = try_expand!(versions
            .iter()
            .map(|attr| self.expand_version(&attr.version))
            .collect::<Result<Vec<_>>>())
        .into_iter();

        let alias_decl = self.expand_alias();
        let enum_decl = self.expand_versioned_enum();
        let from_impl = self.expand_from_impl(&versions);
        let versioned_impl = self.expand_versioned_impl();
        let version_tagged_impl = self.expand_version_tagged_impl();
        let macro_rules = self.expand_macro_rules();

        quote! {
            #(#defs)*
            #alias_decl
            #enum_decl
            #from_impl
            #versioned_impl
            #version_tagged_impl
            #macro_rules
        }
    }
}

impl ToTokens for VersionedItem {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(self.expand());
    }
}
