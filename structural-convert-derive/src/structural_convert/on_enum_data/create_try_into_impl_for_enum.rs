use crate::structural_convert::on_enum_data::utils::concat_enum_with_variant;
use crate::structural_convert::on_fields_named::create_try_into_match_branch_for_fields_named::create_try_into_match_branch_for_fields_named;

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
pub struct TryIntoEnumVariantAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
    skip_after: Option<usize>,
    skip: bool,
}

pub(crate) fn create_try_into_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
    default_for_fields: &[Ident],
    default: bool,
) -> darling::Result<TokenStream> {
    let mut catch_all_branches = vec![];

    if default {
        catch_all_branches.push(quote! { _ => #into_path::default()})
    }

    let match_branches = enum_data
        .variants
        .iter()
        .filter_map(|variant| {
            let mut conversion_error = ConversionError::new(from_path, into_path);

            let variant_ident = variant.ident.clone();
            let from_variant_ident = &variant_ident;
            let attrs = match EnumVariantAttributes::from_attributes(&variant.attrs) {
                Ok(ok) => ok,
                Err(err) => return Some(Err(err)),
            }
            .try_into;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                return Some(Err(darling::Error::custom(
                    "Mixing attributes with 'for' path and no path is not allowed",
                )
                .with_span(variant)));
            }
            let skip = attrs.iter().any(|e| match &e.target {
                Some(target) if target == into_path => e.skip,
                Some(_) => false,
                None => e.skip,
            });
            if skip {
                return None;
            }

            let skip_after = attrs.iter().find_map(|e| match &e.target {
                Some(target) if target == into_path => e.skip_after,
                Some(_) => None,
                None => e.skip_after,
            });

            let into_variant_ident: &Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == into_path => e.rename.as_ref(),
                    Some(_) => None,
                    _ => e.rename.as_ref(),
                })
                .unwrap_or(&variant_ident);

            conversion_error.from.named(from_variant_ident);
            conversion_error.into.named(into_variant_ident);

            let from_path = concat_enum_with_variant(from_path, from_variant_ident);
            let into_path = concat_enum_with_variant(into_path, into_variant_ident);

            let branch = match &variant.fields {
                Fields::Unit => {
                    let err = conversion_error.to_string();
                    quote! {
                        #from_path => #into_path.try_into().map_err(|err| #err)?
                    }
                }
                Fields::Unnamed(fields_unnamed) => {
                    match create_match_branch_for_fields_unnamed(
                        &from_path,
                        |field, err| quote!(#field.try_into().map_err(|err| #err)?),
                        &into_path,
                        fields_unnamed,
                        skip_after,
                        conversion_error,
                    ) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    }
                }
                Fields::Named(fields_named) => {
                    match create_try_into_match_branch_for_fields_named(
                        &from_path,
                        fields_named,
                        &into_path,
                        default_for_fields,
                        conversion_error,
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
        impl TryFrom<#from_path> for #into_path {
            type Error = String;

            fn try_from(value: #from_path) -> Result<Self, Self::Error> {
                Ok(match value {
                    #(#match_branches),*
                    #(#catch_all_branches),*
                })
            }
        }
    ))
}
