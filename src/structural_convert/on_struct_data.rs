use crate::structural_convert::create_from_impl_for_struct::create_from_impl_for_struct;
use crate::structural_convert::create_try_from_impl_for_struct::create_try_from_impl_for_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Path;

use super::ContainerAttributes;

pub(crate) fn on_struct_data(
    input_ident_path: &Path,
    struct_data: &DataStruct,
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
        .map(|into_path| create_from_impl_for_struct(input_ident_path, struct_data, into_path))
        .collect::<Vec<_>>();
    let from_tokens = from_paths
        .iter()
        .map(|from_path| create_from_impl_for_struct(from_path, struct_data, input_ident_path))
        .collect::<Vec<_>>();
    let try_into_tokens = try_into_paths
        .iter()
        .map(|into_path| create_try_from_impl_for_struct(input_ident_path, struct_data, into_path))
        .collect::<Vec<_>>();
    let try_from_tokens = try_from_paths
        .iter()
        .map(|from_path| create_try_from_impl_for_struct(from_path, struct_data, input_ident_path))
        .collect::<Vec<_>>();

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
