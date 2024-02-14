use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::FieldsUnnamed;

/// item1, item2, item3 ...
pub(crate) fn on_fields_unnamed(fields_unnamed: &FieldsUnnamed) -> Vec<TokenStream> {
    fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _field)| {
            let ident = format_ident!("item{i}");
            quote!(#ident)
        })
        .collect::<Vec<_>>()
}
