use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::FieldsUnnamed;

/// item1, item2, item3 ...
pub(crate) fn on_fields_unnamed(
    fields_unnamed: &FieldsUnnamed,
    skip_after: Option<usize>,
) -> Vec<TokenStream> {
    let take_len = skip_after.unwrap_or(fields_unnamed.unnamed.len());
    fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _field)| {
            let ident = format_ident!("item{i}");
            quote!(#ident)
        })
        .take(take_len)
        .collect::<Vec<_>>()
}
