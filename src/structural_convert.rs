// Most variables hold struct Idents or rust types,
// so it is more readable when we keep them Pascal cased
#![allow(non_snake_case)]

use darling::FromAttributes;
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
use syn::Result;

mod create_from_impl_for_enum;
mod create_from_impl_for_struct;
mod on_enum_data;
mod on_fields_named;
mod on_fields_unnamed;
mod on_struct_data;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(convert))]
struct MetaOpts {
    #[darling(multiple)]
    into: Vec<Path>,
    #[darling(multiple)]
    from: Vec<Path>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(default, attributes(convert_field))]
struct FiledOpts {
    from: String,
    into: String,
}

pub fn structural_convert_impl(input: DeriveInput) -> Result<TokenStream> {
    let MetaOpts {
        into: into_paths,
        from: from_paths,
    } = MetaOpts::from_derive_input(&input)?;

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
            on_struct_data(&input_ident_path, &struct_data, &into_paths, &from_paths)
        }
        Data::Enum(enum_data) => {
            on_enum_data(&input_ident_path, &enum_data, &into_paths, &from_paths)
        }
        Data::Union(_union_data) => unimplemented!("Unions are not implemented"),
    };

    Ok(tokens)
}
