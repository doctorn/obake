#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

use proc_macro::TokenStream;

use quote::quote;

use syn::parse::Nothing;
use syn::parse_macro_input;

mod internal;
mod expand;
mod parse;

#[proc_macro_attribute]
pub fn versioned(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(input as internal::VersionedItem);
    let expanded = quote!(#input);
    TokenStream::from(expanded)
}
