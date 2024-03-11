// Most variables hold struct Idents or rust types,
// so it is more readable when we keep them Pascal cased
#![allow(non_snake_case)]

use darling::FromDeriveInput;
use on_enum_data::on_enum_data;
use on_struct_data::on_struct_data;
use proc_macro2::TokenStream;

use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::Data;
use syn::DeriveInput;
use syn::Path;
use syn::PathSegment;

use self::attributes::ContainerAttributes;

pub mod attributes;
mod conversion_error;
mod on_enum_data;
pub mod on_field_type;
mod on_fields_named;
mod on_fields_unnamed;
mod on_struct_data;

pub fn structural_convert_impl(input: DeriveInput) -> darling::Result<TokenStream> {
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

    match data {
        Data::Struct(struct_data) => on_struct_data(
            &input_ident_path,
            &struct_data,
            &container_attributes,
            &input,
        ),
        Data::Enum(enum_data) => on_enum_data(&input_ident_path, &enum_data, &container_attributes),
        Data::Union(_union_data) => {
            Err(darling::Error::custom("Unions are not implemented").with_span(&input))
        }
    }
}
