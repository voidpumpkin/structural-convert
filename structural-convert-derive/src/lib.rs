#![doc = include_str!("../README.md")]
#![deny(
    clippy::unwrap_used,
    clippy::panic,
    clippy::expect_used,
    clippy::unimplemented,
    clippy::todo
)]

extern crate proc_macro;

mod ident_as_literal_list;
mod structural_convert;

use syn::parse_macro_input;
use syn::DeriveInput;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(StructuralConvert, attributes(convert))]
pub fn structural_convert(item: TokenStream) -> TokenStream {
    structural_convert::structural_convert_impl(parse_macro_input!(item as DeriveInput))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
