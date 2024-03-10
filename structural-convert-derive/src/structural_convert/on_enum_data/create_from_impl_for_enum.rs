use crate::structural_convert::on_enum_data::utils::concat_enum_with_variant;
use crate::structural_convert::on_fields_named::create_from_match_branch_for_fields_named::create_from_match_branch_for_fields_named;

use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;
use crate::structural_convert::ConversionError;
use crate::structural_convert::EnumVariantAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Fields;
use syn::Path;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct FromEnumVariantAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    skip: bool,
    rename: Option<Ident>,
    default: bool,
}

pub(crate) fn create_from_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
) -> darling::Result<TokenStream> {
    let mut catch_all_branches = vec![];

    let match_branches = enum_data
        .variants
        .iter()
        .filter_map(|variant| {
            let variant_ident = variant.ident.clone();
            let into_variant_ident = &variant_ident;
            let attrs = match EnumVariantAttributes::from_attributes(&variant.attrs) {
                Ok(ok) => ok,
                Err(err) => return Some(Err(err)),
            }
            .from;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                return Some(Err(darling::Error::custom(
                    "Mixing attributes with 'for' path and no path is not allowed",
                )
                .with_span(variant)));
            }
            let skip = attrs.iter().any(|e| match &e.target {
                Some(target) if target == from_path => e.skip,
                Some(_) => false,
                None => e.skip,
            });
            if skip {
                return None;
            }

            let default = attrs.iter().any(|e| match &e.target {
                Some(target) if target == from_path => e.default,
                Some(_) => false,
                None => e.default,
            });

            let from_variant_ident: &Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == from_path => e.rename.as_ref(),
                    Some(_) => None,
                    _ => e.rename.as_ref(),
                })
                .unwrap_or(&variant_ident);

            let from_path = concat_enum_with_variant(from_path, from_variant_ident);
            let into_path = concat_enum_with_variant(into_path, into_variant_ident);

            let branch = match &variant.fields {
                Fields::Unit if default => {
                    catch_all_branches.push(quote!(_ => #into_path.into()));
                    return None;
                }
                Fields::Unit => {
                    quote! {
                        #from_path => #into_path.into()
                    }
                }
                Fields::Unnamed(fields_unnamed) if default => {
                    let default_expr = fields_unnamed
                        .unnamed
                        .iter()
                        .map(|_| quote!(Default::default()))
                        .collect::<Vec<_>>();
                    catch_all_branches.push(quote!(_ => #into_path(#(#default_expr,)*)));
                    return None;
                }
                Fields::Unnamed(fields_unnamed) => match create_match_branch_for_fields_unnamed(
                    &from_path,
                    |field, _| quote! {#field.into()},
                    &into_path,
                    fields_unnamed,
                    None,
                    ConversionError::empty(),
                ) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                },
                Fields::Named(fields_named) => {
                    match create_from_match_branch_for_fields_named(
                        &from_path,
                        fields_named,
                        &into_path,
                    ) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    }
                }
            };
            Some(Ok(branch))
        })
        .collect::<darling::Result<Vec<_>>>()?;
    Ok(quote!(
        #[automatically_derived]
        impl From<#from_path> for #into_path {
            fn from(value: #from_path) -> Self {
                match value {
                    #(#match_branches),*
                    #(#catch_all_branches),*
                }
            }
        }
    ))
}
