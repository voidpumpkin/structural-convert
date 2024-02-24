use crate::structural_convert::FieldNamedAttributes;
use darling::FromAttributes;
use darling::FromMeta;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use syn::FieldsNamed;
use syn::Path;

use super::create_match_branch_for_fields_named::create_match_branch_for_fields_named;
use super::create_match_branch_for_fields_named::FieldsNamedMatchBranchData;
use super::create_match_branch_for_fields_named::IntoFromPair;
use crate::structural_convert::is_option::is_option;

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(default)]
pub struct TryFromFieldNamedAttributes {
    #[darling(rename = "for")]
    target: Option<Path>,
    rename: Option<Ident>,
    default: bool,
    #[darling(rename = "as")]
    as_type: Option<Path>,
}

pub(crate) fn create_try_from_match_branch_for_fields_named(
    from_path: &Path,
    fields_named: &FieldsNamed,

    into_path: &Path,
) -> darling::Result<TokenStream> {
    let match_branch_data = fields_named
        .named
        .iter()
        .map(|f| {
            let Some(ident) = f.ident.as_ref() else {
                unreachable!()
            };
            let is_option = is_option(&f.ty);

            let attrs = FieldNamedAttributes::from_attributes(&f.attrs)?.try_from;

            let default_attrs = attrs.iter().find(|e| e.target.is_none());
            let has_targeted_attrs = attrs.iter().any(|e| e.target.is_some());
            if default_attrs.is_some() && has_targeted_attrs {
                return Err(darling::Error::custom(
                    "For fields mixing attributes targeted and not targeted is not allowed",
                ));
            }

            let default = attrs.iter().any(|e| match &e.target {
                Some(target) if target == from_path => e.default,
                Some(_) => false,
                None => e.default,
            });

            let as_type = attrs.iter().find_map(|e| match &e.target {
                Some(target) if target == from_path => e.as_type.clone(),
                Some(_) => None,
                None => e.as_type.clone(),
            });

            let from_field_ident: Ident = attrs
                .iter()
                .find_map(|e| match &e.target {
                    Some(target) if target == from_path => e.rename.clone(),
                    Some(_) => None,
                    _ => e.rename.clone(),
                })
                .unwrap_or_else(|| ident.clone());

            let into_from_pair = IntoFromPair {
                into_field_name: ident.clone(),
                from_field_ident: (!default).then_some(from_field_ident.clone()),
            };

            Ok(FieldsNamedMatchBranchData {
                lhs_field_name: (!default).then_some(from_field_ident),
                into_from_pair,
                is_option,
                as_type,
            })
        })
        .collect::<darling::Result<Vec<_>>>()?;

    Ok(create_match_branch_for_fields_named(
        from_path,
        |field_name, as_type| {
            quote!(
                #as_type::try_from(#field_name).map_err(|_| "Failed to convert field".to_string())?
            )
        },
        |field_name| {
            quote!(
                #field_name
                  .try_into().map_err(|_| "Failed to convert field".to_string())?
            )
        },
        into_path,
        match_branch_data,
        &[],
    ))
}
