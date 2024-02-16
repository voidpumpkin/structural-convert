use super::ContainerAttributes;
use create_from_impl_for_enum::create_from_impl_for_enum;
use create_into_impl_for_enum::create_into_impl_for_enum;
use create_try_from_impl_for_enum::create_try_from_impl_for_enum;
use create_try_into_impl_for_enum::create_try_into_impl_for_enum;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Path;

pub mod create_from_impl_for_enum;
pub mod create_into_impl_for_enum;
pub mod create_try_from_impl_for_enum;
pub mod create_try_into_impl_for_enum;
pub mod utils;

pub(crate) fn on_enum_data(
    input_ident_path: &Path,
    enum_data: &DataEnum,
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
        .map(|attrs| create_into_impl_for_enum(input_ident_path, enum_data, &attrs.path))
        .collect::<Vec<TokenStream>>();
    let from_tokens = from
        .iter()
        .map(|attrs| create_from_impl_for_enum(&attrs.path, enum_data, input_ident_path))
        .collect::<Vec<TokenStream>>();
    let try_into_tokens = try_into
        .iter()
        .map(|attrs| create_try_into_impl_for_enum(input_ident_path, enum_data, &attrs.path))
        .collect::<Vec<TokenStream>>();
    let try_from_tokens = try_from
        .iter()
        .map(|attrs| create_try_from_impl_for_enum(&attrs.path, enum_data, input_ident_path))
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
