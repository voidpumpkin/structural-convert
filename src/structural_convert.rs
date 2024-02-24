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

use crate::ident_as_literal_list::IdentAsLiteralList;

use self::on_enum_data::create_from_impl_for_enum::FromEnumVariantAttributes;
use self::on_enum_data::create_into_impl_for_enum::IntoEnumVariantAttributes;
use self::on_enum_data::create_try_from_impl_for_enum::TryFromEnumVariantAttributes;
use self::on_enum_data::create_try_into_impl_for_enum::TryIntoEnumVariantAttributes;
use self::on_fields_named::create_from_match_branch_for_fields_named::FromFieldNamedAttributes;
use self::on_fields_named::create_into_match_branch_for_fields_named::IntoFieldNamedAttributes;
use self::on_fields_named::create_try_from_match_branch_for_fields_named::TryFromFieldNamedAttributes;
use self::on_fields_named::create_try_into_match_branch_for_fields_named::TryIntoFieldNamedAttributes;

pub mod is_option;
mod on_enum_data;
mod on_fields_named;
mod on_fields_unnamed;
mod on_struct_data;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(convert))]
struct ContainerAttributes {
    #[darling(multiple)]
    into: Vec<IntoContainerAttributes>,
    #[darling(multiple)]
    from: Vec<FromContainerAttributes>,
    #[darling(multiple)]
    try_into: Vec<TryIntoContainerAttributes>,
    #[darling(multiple)]
    try_from: Vec<TryFromContainerAttributes>,
}

#[derive(Debug, Clone, FromMeta)]
pub struct FromContainerAttributes {
    path: Path,
}

#[derive(Debug, Clone, FromMeta)]
pub struct TryFromContainerAttributes {
    path: Path,
}

#[derive(Debug, Clone, FromMeta)]
pub struct IntoContainerAttributes {
    path: Path,
    skip_after: Option<usize>,
    #[darling(default)]
    default_for_fields: IdentAsLiteralList,
    #[darling(default)]
    default: bool,
}

#[derive(Debug, Clone, FromMeta)]
pub struct TryIntoContainerAttributes {
    path: Path,
    skip_after: Option<usize>,
    #[darling(default)]
    default_for_fields: IdentAsLiteralList,
    #[darling(default)]
    default: bool,
}
#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct EnumVariantAttributes {
    #[darling(multiple)]
    from: Vec<FromEnumVariantAttributes>,
    #[darling(multiple)]
    into: Vec<IntoEnumVariantAttributes>,
    #[darling(multiple)]
    try_from: Vec<TryFromEnumVariantAttributes>,
    #[darling(multiple)]
    try_into: Vec<TryIntoEnumVariantAttributes>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct FieldNamedAttributes {
    #[darling(multiple)]
    from: Vec<FromFieldNamedAttributes>,
    #[darling(multiple)]
    into: Vec<IntoFieldNamedAttributes>,
    #[darling(multiple)]
    try_from: Vec<TryFromFieldNamedAttributes>,
    #[darling(multiple)]
    try_into: Vec<TryIntoFieldNamedAttributes>,
}

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
        Data::Struct(struct_data) => {
            on_struct_data(&input_ident_path, &struct_data, &container_attributes)
        }
        Data::Enum(enum_data) => on_enum_data(&input_ident_path, &enum_data, &container_attributes),
        Data::Union(_union_data) => Err(darling::Error::custom("Unions are not implemented")),
    }
}
