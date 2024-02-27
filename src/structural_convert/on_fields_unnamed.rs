use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::FieldsUnnamed;
use syn::Path;

use crate::structural_convert::is_option::is_type_option;

/// item1, item2, item3 ...
pub(crate) fn create_match_branch_for_fields_unnamed(
    from_path: &Path,
    into_expr: impl Fn(Ident) -> TokenStream,
    into_path: &Path,
    fields_unnamed: &FieldsUnnamed,
    skip_after: Option<usize>,
) -> TokenStream {
    let take_len = skip_after.unwrap_or(fields_unnamed.unnamed.len());
    let (field_ident, into_expr): (Vec<_>, Vec<_>) = fields_unnamed
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = format_ident!("item{i}");
            let mut into_expr = into_expr(ident.clone());

            if is_type_option(&field.ty) {
                into_expr = quote!(
                    match #ident {
                        None => None,
                        Some(#ident) => Some(#into_expr),
                    }
                );
            };

            (ident, into_expr)
        })
        .take(take_len)
        .unzip();
    quote! {
        #from_path(#(#field_ident,)* ..) => #into_path(#(#into_expr,)*)
    }
}
