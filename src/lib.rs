extern crate proc_macro;

mod structural_convert;

use syn::parse_macro_input;
use syn::DeriveInput;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(StructuralConvert, attributes(convert, convert_field))]
pub fn structural_convert(item: TokenStream) -> TokenStream {
    structural_convert::structural_convert_impl(parse_macro_input!(item as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
