use proc_macro2::TokenStream;
use quote::quote;
use syn::FieldsNamed;

/// some_item: some_item, some_item: some_item,
pub(crate) fn on_fields_named(fields_named: &FieldsNamed) -> Vec<TokenStream> {
    fields_named
        .named
        .iter()
        .map(|f| {
            let ident = f
                .ident
                .as_ref()
                .expect("field has no ident, this should be unreachable");
            quote!(#ident)
        })
        .collect::<Vec<_>>()
}
