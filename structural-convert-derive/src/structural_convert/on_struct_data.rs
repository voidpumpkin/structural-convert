use create_from_impl_for_struct::create_from_impl_for_struct;
use create_into_impl_for_struct::create_into_impl_for_struct;
use create_try_from_impl_for_struct::create_try_from_impl_for_struct;
use create_try_into_impl_for_struct::create_try_into_impl_for_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::DataStruct;
use syn::Path;

pub mod create_from_impl_for_struct;
pub mod create_into_impl_for_struct;
pub mod create_try_from_impl_for_struct;
pub mod create_try_into_impl_for_struct;

use super::attributes::ContainerAttributes;

pub fn on_struct_data(
    input_ident_path: &Path,
    struct_data: &DataStruct,
    container_attributes: &ContainerAttributes,
    span: &impl Spanned,
) -> darling::Result<TokenStream> {
    let ContainerAttributes {
        into,
        from,
        try_into,
        try_from,
    } = container_attributes;

    let into_default = into.iter().any(|e| e.default);
    let try_into_default = try_into.iter().any(|e| e.default);
    if into_default || try_into_default {
        return Err(darling::Error::custom("default on structs is not supported").with_span(span));
    }

    let into_tokens = into
        .iter()
        .map(|attrs| {
            create_into_impl_for_struct(
                input_ident_path,
                struct_data,
                &attrs.path,
                attrs.skip_after,
                &attrs.default_for_fields.0,
            )
        })
        .collect::<darling::Result<Vec<_>>>()?;
    let from_tokens = from
        .iter()
        .map(|attrs| create_from_impl_for_struct(&attrs.path, struct_data, input_ident_path))
        .collect::<darling::Result<Vec<_>>>()?;
    let try_into_tokens = try_into
        .iter()
        .map(|attrs| {
            create_try_into_impl_for_struct(
                input_ident_path,
                struct_data,
                &attrs.path,
                attrs.skip_after,
                &attrs.default_for_fields.0,
            )
        })
        .collect::<darling::Result<Vec<_>>>()?;
    let try_from_tokens = try_from
        .iter()
        .map(|attrs| create_try_from_impl_for_struct(&attrs.path, struct_data, input_ident_path))
        .collect::<darling::Result<Vec<_>>>()?;

    Ok(quote!(
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
    ))
}
