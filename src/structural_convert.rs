// Most variables hold struct Idents or rust types,
// so it is more readable when we keep them Pascal cased
#![allow(non_snake_case)]

use darling::FromAttributes;
use darling::FromDeriveInput;
use darling::FromMeta;
use on_enum_data::on_enum_data;
use on_struct_data::on_struct_data;
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::Data;
use syn::DeriveInput;
use syn::Path;
use syn::PathSegment;
use syn::Result;

mod create_from_impl_for_enum;
mod create_from_impl_for_struct;
mod create_try_from_impl_for_enum;
mod create_try_from_impl_for_struct;
mod on_enum_data;
mod on_fields_named;
mod on_fields_unnamed;
mod on_struct_data;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(convert))]
struct ContainerAttributes {
    #[darling(multiple)]
    into: Vec<Path>,
    #[darling(multiple)]
    from: Vec<Path>,
    #[darling(multiple)]
    try_into: Vec<Path>,
    #[darling(multiple)]
    try_from: Vec<Path>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct TryFromFieldAttributes {
    #[darling(multiple)]
    try_from: Vec<FieldInnerAttributes>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct FromFieldAttributes {
    #[darling(multiple)]
    from: Vec<FieldInnerAttributes>,
}

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
struct FieldInnerAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    skip: bool,
}

// #[derive(Debug, Default, Clone, FromAttributes)]
// #[darling(attributes(convert))]
// pub struct TryIntoFieldAttributes {
//     #[darling(multiple)]
//     try_into: Vec<FieldInnerAttributes>,
// }

// #[derive(Debug, Default, Clone, FromAttributes)]
// #[darling(attributes(convert))]
// pub struct IntoFieldAttributes {
//     #[darling(multiple)]
//     into: Vec<FieldInnerAttributes>,
// }

pub fn structural_convert_impl(input: DeriveInput) -> Result<TokenStream> {
    let container_attributes = ContainerAttributes::from_derive_input(&input)?;

    let DeriveInput {
        ident,
        data,
        attrs: _attrs,
        vis: _vis,
        generics: _generics,
    } = input.clone();

    let input_ident_path_segment = PathSegment {
        ident: ident.clone(),
        arguments: Default::default(),
    };

    let input_ident_path_segments: Punctuated<PathSegment, Colon2> =
        Punctuated::from_iter(vec![input_ident_path_segment]);

    let input_ident_path = Path {
        leading_colon: None,
        segments: input_ident_path_segments,
    };

    let tokens = match data {
        Data::Struct(struct_data) => {
            on_struct_data(&input_ident_path, &struct_data, &container_attributes)
        }
        Data::Enum(enum_data) => on_enum_data(&input_ident_path, &enum_data, &container_attributes),
        Data::Union(_union_data) => unimplemented!("Unions are not implemented"),
    };

    Ok(tokens)
}
