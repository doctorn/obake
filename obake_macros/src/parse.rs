use std::convert::{TryFrom, TryInto};

use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Token};

use crate::internal::*;

const OKABE: &str = "okabe";

impl Parse for VersionedField {
    fn parse(input: ParseStream) -> Result<Self> {
        /*
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
        */
        unimplemented!()
    }
}

/*
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
*/

impl Parse for VersionedFields {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            versioned: content.parse_terminated(VersionedField::parse)?,
        })
    }
}

impl TryFrom<syn::Attribute> for OkabeAttribute {
    type Error = syn::Error;

    fn try_from(value: syn::Attribute) -> Result<Self> {
        unimplemented!()
    }
}

impl TryFrom<syn::Attribute> for VersionedAttribute {
    type Error = syn::Error;

    fn try_from(attr: syn::Attribute) -> Result<Self> {
        attr.path.get_ident().map_or_else(
            || Ok(VersionedAttribute::Attribute(attr.clone())),
            |ident| {
                if ident == OKABE {
                    Ok(VersionedAttribute::Okabe(attr.clone().try_into()?))
                } else {
                    Ok(VersionedAttribute::Attribute(attr.clone()))
                }
            },
        )
    }
}

impl Parse for VersionedAttributes {
    fn parse(input: ParseStream) -> Result<VersionedAttributes> {
        let attrs = input
            .call(syn::Attribute::parse_outer)?
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { attrs })
    }
}

impl Parse for VersionedStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.parse()?;
        let vis = input.parse()?;
        let struct_token = input.parse()?;
        let ident = input.parse::<syn::Ident>()?;
        let fields = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            struct_token,
            ident,
            fields,
        })
    }
}
