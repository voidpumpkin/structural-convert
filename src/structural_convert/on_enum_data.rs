use super::create_from_impl_for_enum::create_from_impl_for_enum;
use super::ContainerAttributes;
use crate::structural_convert::create_try_from_impl_for_enum::create_try_from_impl_for_enum;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Path;

pub(crate) fn on_enum_data(
    input_ident_path: &Path,
    enum_data: &DataEnum,
    container_attributes: &ContainerAttributes,
) -> TokenStream {
    let ContainerAttributes {
        into: into_paths,
        from: from_paths,
        try_into: try_into_paths,
        try_from: try_from_paths,
    } = container_attributes;

    let into_tokens = into_paths
        .iter()
        .map(|into_path| create_from_impl_for_enum(input_ident_path, enum_data, into_path))
        .collect::<Vec<TokenStream>>();
    let from_tokens = from_paths
        .iter()
        .map(|from_path| create_from_impl_for_enum(from_path, enum_data, input_ident_path))
        .collect::<Vec<TokenStream>>();
    let try_into_tokens = try_into_paths
        .iter()
        .map(|into_path| create_try_from_impl_for_enum(input_ident_path, enum_data, into_path))
        .collect::<Vec<TokenStream>>();
    let try_from_tokens = try_from_paths
        .iter()
        .map(|from_path| create_try_from_impl_for_enum(from_path, enum_data, input_ident_path))
        .collect::<Vec<TokenStream>>();

    quote!(
        #(
            #into_tokens
        )*
        #(
            #from_tokens
        )*
        #(
            #try_into_tokens
        )*
        #(
            #try_from_tokens
        )*
    )
}
