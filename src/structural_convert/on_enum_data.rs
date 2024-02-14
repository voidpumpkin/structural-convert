use super::create_from_impl_for_enum::create_from_impl_for_enum;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Path;

pub(crate) fn on_enum_data(
    input_ident_path: &Path,
    enum_data: &DataEnum,
    into_paths: &[Path],
    from_paths: &[Path],
) -> TokenStream {
    let into_tokens = into_paths
        .iter()
        .map(|into_path| create_from_impl_for_enum(input_ident_path, enum_data, into_path))
        .collect::<Vec<TokenStream>>();
    let from_tokens = from_paths
        .iter()
        .map(|from_path| create_from_impl_for_enum(from_path, enum_data, input_ident_path))
        .collect::<Vec<TokenStream>>();

    quote!(
        #(
            #into_tokens
        )*
        #(
            #from_tokens
        )*
    )
}
