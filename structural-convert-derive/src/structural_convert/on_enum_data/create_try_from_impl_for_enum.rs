use crate::structural_convert::on_enum_data::utils::concat_enum_with_variant;
use crate::structural_convert::on_fields_named::create_try_from_match_branch_for_fields_named::create_try_from_match_branch_for_fields_named;

use crate::structural_convert::attributes::EnumVariantAttributes;
use crate::structural_convert::conversion_error::ConversionError;
use crate::structural_convert::on_fields_unnamed::create_match_branch_for_fields_unnamed;
use darling::FromAttributes;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;
use syn::Fields;
use syn::Path;

pub fn create_try_from_impl_for_enum(
    from_path: &Path,
    enum_data: &DataEnum,
    into_path: &Path,
) -> darling::Result<TokenStream> {
    let mut catch_all_branches = vec![];

    let match_branches = enum_data
        .variants
        .iter()
        .filter_map(|variant| {
            let mut conversion_error = ConversionError::new(from_path, into_path);

            let variant_ident = variant.ident.clone();
            let into_variant_ident = &variant_ident;
            let attrs = match EnumVariantAttributes::from_attributes(&variant.attrs) {
                Ok(ok) => ok,
                Err(err) => return Some(Err(err)),
            }
            .try_from;

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

            conversion_error.from.named(from_variant_ident);
            conversion_error.into.named(into_variant_ident);

            let from_path = concat_enum_with_variant(from_path, from_variant_ident);
            let into_path = concat_enum_with_variant(into_path, into_variant_ident);

            let branch = match &variant.fields {
                Fields::Unit if default => {
                    catch_all_branches.push(quote!(_ => #into_path.into()));
                    return None;
                }
                Fields::Unit => {
                    let err = conversion_error.to_string();
                    quote! {
                        #from_path => #into_path.try_into().map_err(|err| #err)?
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
                Fields::Unnamed(fields_unnamed) => {
                    match create_match_branch_for_fields_unnamed(
                        &from_path,
                        |field, err| quote!(#field.try_into().map_err(|err| #err)?),
                        &into_path,
                        fields_unnamed,
                        None,
                        conversion_error,
                    ) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    }
                }
                Fields::Named(fields_named) => {
                    match create_try_from_match_branch_for_fields_named(
                        &from_path,
                        fields_named,
                        &into_path,
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
