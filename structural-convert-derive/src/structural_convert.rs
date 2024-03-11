// Most variables hold struct Idents or rust types,
// so it is more readable when we keep them Pascal cased
#![allow(non_snake_case)]

use std::fmt;

use darling::FromDeriveInput;
use on_enum_data::on_enum_data;
use on_struct_data::on_struct_data;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::Data;
use syn::DeriveInput;
use syn::Path;
use syn::PathSegment;

use self::attributes::ContainerAttributes;

pub mod attributes;
mod on_enum_data;
pub mod on_field_type;
mod on_fields_named;
mod on_fields_unnamed;
mod on_struct_data;

#[derive(Clone)]
pub struct ConversionError {
    pub from: ConversionPath,
    pub into: ConversionPath,
}

impl ConversionError {
    pub fn new(from: impl ToTokens, into: impl ToTokens) -> ConversionError {
        Self {
            from: ConversionPath::start(from),
            into: ConversionPath::start(into),
        }
    }

    pub fn empty() -> ConversionError {
        Self {
            from: ConversionPath::empty(),
            into: ConversionPath::empty(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Failed to convert:")?;
        writeln!(f, "  from: {}", self.from)?;
        writeln!(f, "  into: {}", self.into)
    }
}

impl ToTokens for ConversionError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let err = self.to_string();
        tokens.append_all(quote!(format!(
            "{}\noriginal error:\n{}",
            format!(#err),
            err
        )))
    }
}

#[derive(Clone)]
pub struct ConversionPath {
    path: Vec<ConversionStep>,
}

impl fmt::Display for ConversionPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self
            .path
            .iter()
            .map(|p| p.to_string())
            .collect::<String>()
            .to_token_stream();
        write!(f, "{path}")
    }
}

impl ConversionPath {
    pub fn start(start_ident: impl ToTokens) -> ConversionPath {
        ConversionPath {
            path: vec![ConversionStep::StartIdent(
                start_ident.to_token_stream().to_string(),
            )],
        }
    }

    pub fn unnamed(&mut self, i: usize) {
        self.path.push(ConversionStep::UnnamedField(i));
    }

    pub fn named(&mut self, ident: impl ToTokens) {
        self.path.push(ConversionStep::NamedField(
            ident.to_token_stream().to_string(),
        ));
    }

    pub fn named_str(&mut self, ident: &str) {
        self.path
            .push(ConversionStep::NamedField(ident.to_string()));
    }

    pub fn enum_variant(&mut self, ident: &str) {
        self.path
            .push(ConversionStep::EnumVariant(ident.to_string()));
    }

    pub fn dyn_unnamed(&mut self, ident: impl ToTokens) {
        self.path.push(ConversionStep::DynamicUnnamed(
            ident.to_token_stream().to_string(),
        ));
    }

    pub fn empty() -> ConversionPath {
        ConversionPath { path: vec![] }
    }
}

#[derive(Clone)]
pub enum ConversionStep {
    StartIdent(String),
    UnnamedField(usize),
    NamedField(String),
    EnumVariant(String),
    DynamicUnnamed(String),
}

impl fmt::Display for ConversionStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionStep::StartIdent(ident) => write!(f, "{ident}"),
            ConversionStep::UnnamedField(ident) => write!(f, ".{ident}"),
            ConversionStep::NamedField(ident) => write!(f, ".{ident}"),
            ConversionStep::EnumVariant(ident) => write!(f, "::{ident}"),
            ConversionStep::DynamicUnnamed(ident) => write!(f, ".{{{ident}}}"),
        }
    }
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
