use create_from_impl_for_struct::create_from_impl_for_struct;
use create_into_impl_for_struct::create_into_impl_for_struct;
use create_try_from_impl_for_struct::create_try_from_impl_for_struct;
use create_try_into_impl_for_struct::create_try_into_impl_for_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Path;

pub mod create_from_impl_for_struct;
pub mod create_into_impl_for_struct;
pub mod create_try_from_impl_for_struct;
pub mod create_try_into_impl_for_struct;

use super::ContainerAttributes;

pub(crate) fn on_struct_data(
    input_ident_path: &Path,
    struct_data: &DataStruct,
    container_attributes: &ContainerAttributes,
) -> TokenStream {
    let ContainerAttributes {
        into,
        from,
        try_into,
        try_from,
    } = container_attributes;

    let into_tokens = into
        .iter()
        .map(|attrs| create_into_impl_for_struct(input_ident_path, struct_data, &attrs.path))
        .collect::<Vec<_>>();
    let from_tokens = from
        .iter()
        .map(|attrs| create_from_impl_for_struct(&attrs.path, struct_data, input_ident_path))
        .collect::<Vec<_>>();
    let try_into_tokens = try_into
        .iter()
        .map(|attrs| create_try_into_impl_for_struct(input_ident_path, struct_data, &attrs.path))
        .collect::<Vec<_>>();
    let try_from_tokens = try_from
        .iter()
        .map(|attrs| create_try_from_impl_for_struct(&attrs.path, struct_data, input_ident_path))
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
